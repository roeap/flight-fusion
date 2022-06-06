from dagster import InitResourceContext, resource
from flight_fusion import ClientOptions, FusionServiceClient

from dagster_fusion.resources.configuration import MlFusionConfiguration


@resource(  # type: ignore
    description="Service client to interact with flight-fusion service",
    required_resource_keys={"mlfusion_config"},
)
def flight_fusion_resource(context: InitResourceContext) -> FusionServiceClient:
    config: MlFusionConfiguration = context.resources.mlfusion_config  # type: ignore
    return FusionServiceClient(ClientOptions(host=config.flight_host, port=config.flight_port))
