from typing import Protocol, TypedDict

from dagster import root_input_manager

from dagster_fusion._types import TableReference, TypedInputContext
from dagster_fusion.config import FIELD_COLUMN_SELECTION
from dagster_fusion.errors import MissingConfiguration
from flight_fusion import DatasetClient, FusionServiceClient

_INPUT_CONFIG_SCHEMA = {"columns": FIELD_COLUMN_SELECTION}


class InputConfig(TypedDict):
    location: TableReference


class LoaderResources(Protocol):
    fusion_client: FusionServiceClient


@root_input_manager(
    description="`RootInputManager` for loading tables from flight fusion service.",
    input_config_schema=_INPUT_CONFIG_SCHEMA,
    required_resource_keys={"fusion_client"},
)
def flight_fusion_loader(context: TypedInputContext[InputConfig, LoaderResources, None]):
    if context.asset_key is None:
        raise MissingConfiguration("An `asset_key` must be provided")

    client = DatasetClient(
        client=context.resources.fusion_client._flight,
        asset_key=context.asset_key,  # type: ignore
    )

    return client.load()
