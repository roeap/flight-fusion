from typing import TypedDict

from dagster import resource
from dagster_fusion._types import TypedInitResourceContext

from flight_fusion import ClientOptions, FlightFusionClient


class FusionConfig(TypedDict):
    host: str
    port: int


@resource(config_schema={"host": str, "port": int})
def flight_fusion_resource(
    init_context: TypedInitResourceContext[FusionConfig],
) -> FlightFusionClient:
    options = ClientOptions(
        host=init_context.resource_config["host"], port=init_context.resource_config["port"]
    )
    return FlightFusionClient(options)
