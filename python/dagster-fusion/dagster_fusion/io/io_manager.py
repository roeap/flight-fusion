from __future__ import annotations

from typing import Iterable, List, Optional, Protocol, Set, TypedDict

import pandas as pd
import polars as pl
import pyarrow as pa
from dagster import (
    AssetKey,
    IOManager,
    MetadataEntry,
    MetadataValue,
    TableColumn,
    TableSchema,
    io_manager,
)
from dagster_fusion._types import (
    AreaConfig,
    TableReference,
    TypedInputContext,
    TypedOutputContext,
)
from dagster_fusion.config import (
    FIELD_COLUMN_SELECTION,
    FIELD_LOCATION,
    FIELD_SAVE_MODE,
    area_source_to_asset_key,
    table_reference_to_area_source,
)
from dagster_fusion.errors import MissingConfiguration
from flight_fusion import AreaClient, DatasetClient, FusionServiceClient, TableClient
from flight_fusion.ipc.v1alpha1 import SaveMode

_INPUT_CONFIG_SCHEMA = {"columns": FIELD_COLUMN_SELECTION}

_OUTPUT_CONFIG_SCHEMA = {
    "location": FIELD_LOCATION,
    "save_mode": FIELD_SAVE_MODE,
}


class InputConfig(TypedDict, total=False):
    columns: List[str]


class OutputConfig(TypedDict, total=False):
    location: TableReference
    save_mode: SaveMode
    partition_columns: List[str]


class IOManagerResources(Protocol):
    fusion_client: FusionServiceClient


class TableIOManager(IOManager):
    def __init__(self, client: FusionServiceClient) -> None:
        self._fusion = client

    def _get_dataset_client(self, config: OutputConfig | InputConfig | AssetKey) -> DatasetClient:
        if isinstance(config, AssetKey):
            location = TableReference(
                source=AreaConfig(name=config.path[-1], areas=config.path[:-1])  # type: ignore
            )
        else:
            location = config.get("location")
            if location is None:
                raise MissingConfiguration("Field `location` must be configured")

        reference = table_reference_to_area_source(location)

        return TableClient(
            client=AreaClient(client=self._fusion, areas=reference.location.areas),
            reference=reference,
        )

    def handle_output(
        self,
        context: TypedOutputContext[OutputConfig, IOManagerResources],
        obj: pd.DataFrame | pa.Table | pl.DataFrame,
    ) -> Iterable[MetadataEntry]:
        client = self._get_dataset_client(config=context.asset_key or context.config)
        save_mode = context.config.get("save_mode") or SaveMode.SAVE_MODE_APPEND

        data = obj
        if isinstance(obj, pd.DataFrame):
            data = pa.Table.from_pandas(obj)
        elif isinstance(obj, pl.DataFrame):
            data = obj.to_arrow()
        data = data.replace_schema_metadata({})

        client.write_into(data, save_mode)

        yield MetadataEntry.int(data.nbytes, "size (bytes)")
        yield MetadataEntry.int(data.num_rows, "row count")

        schema = TableSchema(
            columns=[TableColumn(name=col.name, type=str(col.type)) for col in data.schema]
        )
        yield MetadataEntry("table_schema", value=MetadataValue.table_schema(schema))

        df = pl.from_arrow(data)
        df_null = df.null_count()

        column_names = ["statistic"] + df_null.columns  # type: ignore
        stats = [dict(zip(column_names, row)) for row in df.describe().rows()]
        stats.append(dict(zip(column_names, ("null_count",) + df_null.row(0))))  # type: ignore

        yield MetadataEntry(
            "column_statistics",
            value=MetadataValue.json({"stats": stats}),
        )

        # stats_schema = TableSchema(
        #     columns=[TableColumn(name="statistic", type="string")]
        #     + [TableColumn(name=col, type="float") for col in column_names[1:]]
        # )
        # stats = [TableRecord(**dict(zip(column_names, row))) for row in df.describe().rows()]
        # stats.append(TableRecord(**dict(zip(column_names, ("null_count",) + df_null.row(0)))))  # type: ignore

        # yield MetadataEntry(
        #     "column_statistics",
        #     value=TableMetadataValue(
        #         records=stats,
        #         schema=stats_schema,
        #     ),
        # )

    def load_input(
        self,
        context: TypedInputContext[
            InputConfig, IOManagerResources, TypedOutputContext[OutputConfig, IOManagerResources]
        ],
    ) -> pa.Table | pl.DataFrame | pd.DataFrame:
        config = (
            context.asset_key or context.upstream_output.asset_key or context.upstream_output.config
        )
        if config is None:
            raise MissingConfiguration("Filed to get source reference")

        client = self._get_dataset_client(config=config)
        data = client.load()

        # determine supported return types based on the type of the downstream input
        if context.dagster_type.typing_type == pl.DataFrame:
            return pl.from_arrow(data)

        if context.dagster_type.typing_type == pd.DataFrame:
            return data.to_pandas()

        return data

    def get_output_asset_key(
        self, context: TypedOutputContext[OutputConfig, IOManagerResources]
    ) -> Optional[AssetKey]:
        """Associates outputs handled by this IOManager with a particular AssetKey."""
        if context.asset_key is not None:
            return None

        location = context.config.get("location")
        if location is None:
            raise MissingConfiguration("Field `location` must be configured")

        reference = table_reference_to_area_source(location)
        return area_source_to_asset_key(reference)

    def get_output_asset_partitions(self, _context) -> Set[str]:
        """User-defined method that associates outputs handled by this IOManager with a set of
        partitions of an AssetKey.

        Args:
            context (OutputContext): The context of the step output that produces this object.
        """
        return set()

    def get_input_asset_key(
        self,
        context: TypedInputContext[
            InputConfig, IOManagerResources, TypedOutputContext[OutputConfig, IOManagerResources]
        ],
    ) -> Optional[AssetKey]:
        """Associates inputs handled by this IOManager with a particular AssetKey."""
        if context.upstream_output is None:
            return None

        if context.upstream_output.asset_key is not None:
            return None

        location = context.upstream_output.config.get("location")
        if location is None:
            return None

        reference = table_reference_to_area_source(location)
        return area_source_to_asset_key(reference)


@io_manager(
    description="IO Manager for handling dagster assets within a flight fusion service.",
    input_config_schema=_INPUT_CONFIG_SCHEMA,
    output_config_schema=_OUTPUT_CONFIG_SCHEMA,
    required_resource_keys={"fusion_client"},
)
def flight_fusion_io_manager(init_context):
    return TableIOManager(init_context.resources.fusion_client)
