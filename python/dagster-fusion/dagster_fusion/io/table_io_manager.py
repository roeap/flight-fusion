from __future__ import annotations

from typing import Iterable

import pandas as pd
import polars as pl
import pyarrow as pa
from dagster import (
    AssetKey,
    InitResourceContext,
    InputContext,
    IOManager,
    MetadataEntry,
    MetadataValue,
    OutputContext,
    TableColumn,
    TableSchema,
    io_manager,
    root_input_manager,
)
from dagster.core.errors import DagsterInvariantViolationError
from flight_fusion import BaseDatasetClient, FusionServiceClient
from flight_fusion.ipc.v1alpha1 import SaveMode

from dagster_fusion.config import FIELD_COLUMN_SELECTION, FIELD_SAVE_MODE
from dagster_fusion.errors import MissingConfiguration

_INPUT_CONFIG_SCHEMA = {"columns": FIELD_COLUMN_SELECTION}
_OUTPUT_CONFIG_SCHEMA = {"save_mode": FIELD_SAVE_MODE}


class TableIOManager(IOManager):
    def __init__(self, client: FusionServiceClient) -> None:
        self._fusion = client

    def _get_dataset_client(self, asset_key: AssetKey) -> BaseDatasetClient:
        return self._fusion.get_dataset_client(asset_key=asset_key)  # type: ignore

    def handle_output(
        self,
        context: OutputContext,
        obj: pd.DataFrame | pa.Table | pl.DataFrame,
    ) -> Iterable[MetadataEntry]:
        if context.asset_key is None:
            raise MissingConfiguration("'asset_key' must be provided")

        client = self._get_dataset_client(asset_key=context.asset_key)
        # TODO get save_mode from metadata
        save_mode = context.config.get("save_mode") or SaveMode.SAVE_MODE_APPEND

        data = obj
        if isinstance(obj, pd.DataFrame):
            data = pa.Table.from_pandas(obj)
        elif isinstance(obj, pl.DataFrame):
            data = obj.to_arrow()
        data = data.replace_schema_metadata({})

        if data.num_rows > 0:
            client.write_into(data, save_mode)
        else:
            context.log.warning(f"Tried writing empty data for asset: {context.asset_key}")
            return

        yield MetadataEntry("size (bytes)", value=MetadataValue.int(data.nbytes))
        yield MetadataEntry("row count", value=MetadataValue.int(data.num_rows))
        yield MetadataEntry("save mode", value=MetadataValue.text(save_mode.name))

        schema = TableSchema(columns=[TableColumn(name=col.name, type=str(col.type)) for col in data.schema])
        yield MetadataEntry("table_schema", value=MetadataValue.table_schema(schema))

        try:
            df: pl.DataFrame = pl.from_arrow(data)  # type: ignore

            stats: list[pl.DataFrame] = []
            for col in df.columns:
                try:
                    series_stats = df.get_column(col).describe()
                    series_stats.columns = ["statistic", col]
                    stats.append(series_stats)
                except Exception:
                    context.log.warning(f"Error computing statistics for column: '{col}'.")

            df_series = stats[0]
            if len(stats) > 1:
                for tbl in stats[1:]:
                    df_series = df_series.join(tbl, on="statistic", how="outer")

            df_stats = (
                df_series[:, 1:]
                .transpose(column_names=df_series[:, 0])  # type: ignore
                .with_column(pl.Series(name="column_name", values=df_series.columns[1:]))
            )

            yield MetadataEntry(
                "column_statistics",
                value=MetadataValue.md(df_stats.to_pandas().to_markdown()),
            )

        except Exception:
            context.log.warning("Error computing table statistics.")

    def load_input(self, context: InputContext) -> pa.Table | pl.DataFrame | pd.DataFrame:
        try:
            asset_key = context.asset_key
        except DagsterInvariantViolationError:
            asset_key = context.upstream_output.asset_key if context.upstream_output else None

        if asset_key is None:
            raise MissingConfiguration("'asset_key' must be provided")

        client = self._get_dataset_client(asset_key=asset_key)
        data = client.load(columns=(context.metadata or {}).get("columns"))

        # determine supported return types based on the type of the downstream input
        if context.dagster_type.typing_type == pl.DataFrame:
            return pl.from_arrow(data)  # type: ignore

        if context.dagster_type.typing_type == pd.DataFrame:
            return data.to_pandas()

        return data


@io_manager(
    description="IO Manager for handling dagster assets within a flight fusion service.",
    input_config_schema=_INPUT_CONFIG_SCHEMA,
    output_config_schema=_OUTPUT_CONFIG_SCHEMA,
    required_resource_keys={"fusion_client"},
)
def flight_fusion_io_manager(context: InitResourceContext):
    client: FusionServiceClient = context.resources.fusion_client  # type: ignore
    return TableIOManager(client=client)


@root_input_manager(
    description="`RootInputManager` for loading tables from flight fusion service.",
    input_config_schema=_INPUT_CONFIG_SCHEMA,
    required_resource_keys={"fusion_client"},
)
def flight_fusion_loader(context: InputContext):
    if context.asset_key is None:
        raise MissingConfiguration("An `asset_key` must be provided")
    service: FusionServiceClient = context.resources.fusion_client
    return service.get_dataset_client(asset_key=context.asset_key)  # type: ignore
