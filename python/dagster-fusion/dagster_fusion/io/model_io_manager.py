from __future__ import annotations

from dagster import (
    InitResourceContext,
    InputContext,
    IOManager,
    OutputContext,
    resource,
)

from dagster_fusion.resources import MlFlow


class ModelArtifactIOManager(IOManager):
    def __init__(self, mlflow: MlFlow) -> None:
        self._mlflow = mlflow

    def handle_output(self, context: OutputContext, obj):
        return super().handle_output(context, obj)

    def load_input(self, context: InputContext):
        return super().load_input(context)


@resource(required_resource_keys={"mlflow"})
def model_artifact_io_manager(context: InitResourceContext) -> ModelArtifactIOManager:
    return ModelArtifactIOManager(mlflow=context.resources.mlflow)  # type: ignore
