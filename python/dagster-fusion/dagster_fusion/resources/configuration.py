from dagster import Field, InitResourceContext, IntSource, StringSource, resource
from pydantic import BaseSettings
from pydantic import Field as PydanticField

_RESOURCE_CONFIG_SCHEMA = {
    "flight_host": Field(StringSource, is_required=False, description="Mlfusion flight service host"),
    "flight_port": Field(IntSource, is_required=False, description="Mlfusion flight service port"),
    "mlflow_tracking_uri": Field(StringSource, is_required=False, description="MlFlow tracking server uri."),
    "mlserver_host": Field(StringSource, is_required=False, description="Mlserver model serving url"),
    "mlserver_port": Field(IntSource, is_required=False, description="Mlserver model serving port"),
}


class MlFusionConfiguration(BaseSettings):
    flight_host: str = PydanticField(default="localhost")
    flight_port: int = PydanticField(default=50051)
    mlflow_tracking_uri: str = PydanticField(default=None)
    mlserver_host: str = PydanticField(default="localhost")
    mlserver_port: int = PydanticField(default=8081)

    class Config:
        fields = {
            "flight_host": {
                "env": ["ff_flight_host", "ff_host"],
            },
            "flight_port": {
                "env": ["ff_flight_port", "ff_port"],
            },
            "flight_use_ssl": {
                "env": ["ff_flight_use_ssl", "ff_use_ssl"],
            },
            "mlserver_host": {
                "env": ["ff_mlserver_host"],
            },
            "mlserver_port": {
                "env": ["ff_mlserver_port"],
            },
        }


@resource(
    config_schema=_RESOURCE_CONFIG_SCHEMA,
    description="Configuration helper to manage options / config for multiple resources",
)
def mlfusion_configuration(context: InitResourceContext):
    config = {k: v for k, v in (context.resource_config or {}).items() if v is not None}
    return MlFusionConfiguration(**config)
