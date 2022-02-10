from typing import List, Optional, Protocol, Set, TypedDict, Union

import pandas as pd
import pyarrow as pa
from dagster import (
    Array,
    AssetKey,
    Enum,
    Field,
    IOManager,
    Selector,
    Shape,
    String,
    io_manager,
)
from dagster_fusion._types import TableReference, TypedInputContext, TypedOutputContext
from flight_fusion import AreaClient, DatasetClient, FusionServiceClient
from flight_fusion.ipc.v1alpha1 import AreaSourceReference, AreaTableLocation, SaveMode

_INPUT_CONFIG_SCHEMA = {
    "columns": Field(
        # HACK this should be an Array(String) but somehow we run into trouble with
        # list vs frozenlist type checks
        Array(String),
        is_required=False,
        description="Sub-selection of columns to load from dataset",
    ),
}

_OUTPUT_CONFIG_SCHEMA = {
    "location": Field(
        Selector(
            {
                "key": Field(String, is_required=False),
                # TODO find a better name then `source`
                "source": Field(
                    Shape(
                        fields={
                            "name": Field(
                                String,
                                is_required=True,
                                description="Table / location name where data is loaded from",
                            ),
                            "areas": Field(Array(String)),
                        }
                    ),
                    is_required=False,
                ),
            }
        ),
        is_required=True,
    ),
    "save_mode": Field(
        Enum.from_python_enum(SaveMode),
        is_required=False,
        default_value="SAVE_MODE_OVERWRITE",
        description="Specifies behavior when saving data into a table location",
    ),
}


class InputConfig(TypedDict, total=False):
    location: TableReference
    columns: List[str]


class OutputConfig(TypedDict, total=False):
    location: TableReference
    columns: List[str]


class IOManagerResources(Protocol):
    fusion_client: FusionServiceClient


class FlightFusionIOManager(IOManager):
    def _get_dataset_client(
        self, client: FusionServiceClient, config: Union[OutputConfig, InputConfig]
    ) -> DatasetClient:
        location = config.get("location")
        if location is None:
            raise ValueError("Location must be configured")

        key = location.get("key")
        if key is not None:
            parts = key.split("/")
            areas = parts[:-1]
            name = parts[-1]

            return DatasetClient(
                client=AreaClient(client=client, areas=areas),
                reference=AreaSourceReference(location=AreaTableLocation(name=name, areas=areas)),
            )

        source = location.get("source")
        if source is not None:
            return DatasetClient(
                client=AreaClient(client=client, areas=source["areas"]),
                reference=AreaSourceReference(
                    location=AreaTableLocation(name=source["name"], areas=source["areas"])
                ),
            )

        raise ValueError("Missing configuration")

    def handle_output(
        self,
        context: TypedOutputContext[OutputConfig, IOManagerResources],
        obj: Union[pd.DataFrame, pa.Table],
    ) -> None:
        client = self._get_dataset_client(
            client=context.resources.fusion_client, config=context.config
        )
        # TODO yield metadata
        client.write_into(obj)

    def load_input(self, context: TypedInputContext[InputConfig, IOManagerResources]) -> pa.Table:
        if context.upstream_output is None or context.upstream_output.config is None:
            raise ValueError
        client = self._get_dataset_client(
            client=context.resources.fusion_client, config=context.upstream_output.config
        )
        return client.load()

    def get_output_asset_key(self, _context) -> Optional[AssetKey]:
        """User-defined method that associates outputs handled by this IOManager with a particular
        AssetKey.

        Args:
            context (OutputContext): The context of the step output that produces this object.
        """
        return None

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
    return FlightFusionIOManager()
