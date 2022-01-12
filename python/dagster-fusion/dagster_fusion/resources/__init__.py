from dagster import resource
from flight_fusion import FlightFusionClient


@resource(config_schema={"host": str, "port": int})
def flight_fusion_resource(init_context):
    host = init_context.resource_config["host"]
    port = init_context.resource_config["port"]
    return FlightFusionClient(host=host, port=port)
