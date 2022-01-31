from typing import List, TypedDict

from dagster import root_input_manager
from dagster_fusion._types import TypedInputContext
from flight_fusion import AreaClient, DatasetClient, FlightFusionClient
from flight_fusion.ipc.v1alpha1 import AreaSourceReference, AreaTableLocation


class InputConfig(TypedDict):
    name: str
    areas: List[str]


class LoaderResources(TypedDict):
    fusion_client: FlightFusionClient


@root_input_manager(
    input_config_schema={"name": str, "areas": List[str]}, required_resource_keys=["fusion_client"]
)
def fusion_loader(context: TypedInputContext[InputConfig, LoaderResources]):
    client = DatasetClient(
        client=AreaClient(client=context.resources["fusion_client"], areas=context.config["areas"]),
        reference=AreaSourceReference(
            location=AreaTableLocation(name=context.config["name"], areas=context.config["areas"])
        ),
    )
    return client.load()
