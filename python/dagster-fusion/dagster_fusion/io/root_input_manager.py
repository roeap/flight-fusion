from typing import Protocol, TypedDict

from dagster import root_input_manager
from dagster_fusion._types import TableReference, TypedInputContext
from dagster_fusion.config import (
    FIELD_COLUMN_SELECTION,
    FIELD_LOCATION,
    table_reference_to_area_source,
)
from dagster_fusion.errors import MissingConfiguration
from flight_fusion import AreaClient, FusionServiceClient, TableClient

_INPUT_CONFIG_SCHEMA = {
    "location": FIELD_LOCATION,
    "columns": FIELD_COLUMN_SELECTION,
}


class InputConfig(TypedDict):
    location: TableReference


class LoaderResources(Protocol):
    fusion_client: FusionServiceClient


@root_input_manager(
    input_config_schema=_INPUT_CONFIG_SCHEMA,
    required_resource_keys={"fusion_client"},
)
def flight_fusion_loader(context: TypedInputContext[InputConfig, LoaderResources]):
    location = context.config.get("location")
    if location is None:
        raise MissingConfiguration("Field `location` must be configured")

    reference = table_reference_to_area_source(location)
    client = TableClient(
        client=AreaClient(client=context.resources.fusion_client, areas=reference.location.areas),
        reference=reference,
    )

    return client.load()
