from __future__ import annotations

import json
from enum import Enum

import mlflow
from dagster import (
    InitResourceContext,
    InputContext,
    IOManager,
    OutputContext,
    io_manager,
)
from mlflow.utils.file_utils import TempDir
from pydantic import BaseSettings

from dagster_fusion.errors import MissingConfiguration
from dagster_fusion.resources import MlFlow

_TAG_ASSET_KEY = "mlfusion.asset_key"


class FileType(Enum):
    PICKLE = "pickle"
    JSON = "json"
    YAML = "yaml"
    CLOUDPICKLE = "cloudpickle"


class ArtifactMetaData(BaseSettings):
    file_name: str
    artifact_path: str | None
    file_type: FileType | None

    @property
    def file_type_inferred(self) -> FileType:
        if self.file_type is not None:
            return self.file_type
        if self.file_name.endswith(".pkl"):
            return FileType.PICKLE
        if self.file_name.endswith(".json"):
            return FileType.JSON
        if self.file_name.endswith(".yaml") or self.file_name.endswith(".yml"):
            return FileType.YAML
        raise MissingConfiguration("failed to infer file type for output")


class ModelArtifactIOManager(IOManager):
    def __init__(self, mlflow: MlFlow) -> None:
        self._mlflow = mlflow

    def handle_output(self, context: OutputContext, obj):
        if self._mlflow.experiment is None:
            raise ValueError("No active mlflow experiment")

        metadata = ArtifactMetaData(**(context.metadata or {}))
        asset_key = context.asset_key
        serialized_asset_key = json.dumps(asset_key.path)

        mlflow.set_tag(_TAG_ASSET_KEY, json.dumps(asset_key.path))

        client = mlflow.tracking.MlflowClient()
        # fetch most recent experiment, since we may have tagged it since we started the run
        experiment = client.get_experiment(self._mlflow.experiment.experiment_id)
        experiment_is_tagged = False
        tag_count = 0
        for key, value in experiment.tags.items():
            if _TAG_ASSET_KEY in key:
                tag_count += 1
                if value == serialized_asset_key:
                    experiment_is_tagged = True

        if not experiment_is_tagged:
            if tag_count > 0:
                client.set_experiment_tag(
                    self._mlflow.experiment.experiment_id, f"{_TAG_ASSET_KEY}.{tag_count}", serialized_asset_key
                )
            else:
                client.set_experiment_tag(self._mlflow.experiment.experiment_id, _TAG_ASSET_KEY, serialized_asset_key)

        if metadata.file_type_inferred == FileType.PICKLE:
            import pickle  # nosec

            with TempDir() as src_dir:
                artifact_src_path = src_dir.path(metadata.file_name)
                with open(artifact_src_path, "wb") as f:
                    pickle.dump(obj=obj, file=f)
                mlflow.log_artifact(local_path=artifact_src_path, artifact_path=metadata.artifact_path)

        elif metadata.file_type_inferred == FileType.JSON:
            with TempDir() as src_dir:
                artifact_src_path = src_dir.path(metadata.file_name)
                with open(artifact_src_path, "w") as f:
                    json.dump(obj=obj, fp=f)
                mlflow.log_artifact(local_path=artifact_src_path, artifact_path=metadata.artifact_path)

        elif metadata.file_type_inferred == FileType.YAML:
            import yaml

            with TempDir() as src_dir:
                artifact_src_path = src_dir.path(metadata.file_name)
                with open(artifact_src_path, "w") as f:
                    yaml.dump(data=obj, stream=f)
                mlflow.log_artifact(local_path=artifact_src_path, artifact_path=metadata.artifact_path)

    def load_input(self, context: InputContext):
        raise NotImplementedError


@io_manager(required_resource_keys={"mlflow"})
def model_artifact_io_manager(context: InitResourceContext) -> ModelArtifactIOManager:
    return ModelArtifactIOManager(mlflow=context.resources.mlflow)  # type: ignore
