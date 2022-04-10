from __future__ import annotations

from typing import List, Optional, Protocol, Set, TypedDict

import pandas as pd
import pyarrow as pa
from dagster import AssetKey, IOManager, io_manager
from dagster_fusion._types import TableReference, TypedInputContext, TypedOutputContext
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
    def _get_dataset_client(
        self, client: FusionServiceClient, config: OutputConfig | InputConfig
    ) -> DatasetClient:
        location = config.get("location")
        if location is None:
            raise MissingConfiguration("Field `location` must be configured")

        reference = table_reference_to_area_source(location)
        return TableClient(
            client=AreaClient(client=client, areas=reference.location.areas), reference=reference
        )

    def handle_output(
        self,
        context: TypedOutputContext[OutputConfig, IOManagerResources],
        obj: pd.DataFrame | pa.Table,
    ) -> None:
        client = self._get_dataset_client(
            client=context.resources.fusion_client, config=context.config
        )
        save_mode = context.config.get("save_mode") or SaveMode.SAVE_MODE_APPEND
        # TODO yield metadata
        client.write_into(obj, save_mode)

    def load_input(self, context: TypedInputContext[InputConfig, IOManagerResources]) -> pa.Table:
        if context.upstream_output is None or context.upstream_output.config is None:
            raise ValueError
        client = self._get_dataset_client(
            client=context.resources.fusion_client, config=context.upstream_output.config
        )
        return client.load()

    def get_output_asset_key(
        self, context: TypedOutputContext[OutputConfig, IOManagerResources]
    ) -> Optional[AssetKey]:
        """User-defined method that associates outputs handled by this IOManager with a particular
        AssetKey.

        Args:
            context (OutputContext): The context of the step output that produces this object.
        """
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


@io_manager(
    required_resource_keys={"fusion_client"},
    description="IO Manager for handling dagster assets within a flight fusion service.",
    input_config_schema=_INPUT_CONFIG_SCHEMA,
    output_config_schema=_OUTPUT_CONFIG_SCHEMA,
)
def flight_fusion_io_manager(init_context):
    return TableIOManager()
