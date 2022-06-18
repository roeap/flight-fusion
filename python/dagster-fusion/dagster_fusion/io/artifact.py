from __future__ import annotations

import json
from dataclasses import dataclass
from enum import Enum
from pathlib import Path
from typing import Any

from dagster import (
    InitResourceContext,
    InputContext,
    IOManager,
    MetadataEntry,
    MetadataValue,
    OutputContext,
    io_manager,
)
from pydantic import BaseSettings

import mlflow
from dagster_fusion.errors import MissingConfiguration
from dagster_fusion.resources import MlFlow, MlFusionConfiguration
from flight_fusion.asset_key import IAssetKey
from flight_fusion.tags import MlFusionTags
from mlflow.utils.file_utils import TempDir


class FileType(Enum):
    PICKLE = "pickle"
    JSON = "json"
    YAML = "yaml"
    CLOUDPICKLE = "cloudpickle"


@dataclass
class RegisteredModel:
    # if no payload is passed, we only make sure all connections are made.
    # often the model is written using the autolog APIs which will yield much more complete artifacts.
    payload: Any | None = None

    # A descriptive text for the model to be shown in UIs
    description: str | None = None

    # Tags associated with the model
    tags: dict[str, Any] | None = None

    # the folder where artifacts associated with model are placed.
    # the default "model" is used in mlflows autolog features.
    model_folder: str = "model"


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

    @property
    def path_rel(self) -> str:
        return f"{self.artifact_path}/{self.file_name}" if self.artifact_path else self.file_name


@io_manager(required_resource_keys={"mlflow", "mlfusion_config"})
def model_artifact_io_manager(context: InitResourceContext) -> ModelArtifactIOManager:
    return ModelArtifactIOManager(mlflow=context.resources.mlflow, config=context.resources.mlfusion_config)  # type: ignore


class ModelArtifactIOManager(IOManager):
    def __init__(self, mlflow: MlFlow, config: MlFusionConfiguration | None = None) -> None:
        self._mlflow = mlflow
        self._config = config or MlFusionConfiguration()

    def handle_output(self, context: OutputContext, obj: Any):
        if self._mlflow.experiment is None:
            raise ValueError("No active mlflow experiment")

        asset_key = context.asset_key
        self._ensure_tagged_experiment(asset_key=asset_key)
        run = self._get_tagged_run(asset_key=asset_key)
        experiment_url = f"{self._config.mlflow_link}/#/experiments/{run.info.experiment_id}/runs/{run.info.run_id}"

        write_obj = True
        fusion_path = None
        if isinstance(obj, RegisteredModel):
            write_obj = obj.payload is not None
            model = self._mlflow.get_or_create_registered_model(asset_key=asset_key)
            fusion_path = f"fusion:{run.info.experiment_id}/{run.info.run_id}/{obj.model_folder}"
            if len(model.latest_versions) == 0:
                self._mlflow.tracking_client.create_model_version(
                    name=model.name,
                    source=fusion_path,
                    run_link=experiment_url,
                    run_id=run.info.run_id,
                    tags={f"{MlFusionTags.ASSET_KEY}": asset_key.to_string()},
                )

        if write_obj:
            metadata = ArtifactMetaData(**(context.metadata or {}))
            self._write_artifact(metadata=metadata, obj=obj)
            fusion_path = f"fusion:{run.info.experiment_id}/{run.info.run_id}/{metadata.path_rel}"

        yield MetadataEntry(MlFusionTags.mlflow.RUN_ID, value=MetadataValue.text(run.info.run_id))
        yield MetadataEntry(MlFusionTags.mlflow.EXPERIMENT_ID, value=MetadataValue.text(run.info.experiment_id))
        yield MetadataEntry(MlFusionTags.mlflow.EXPERIMENT_URL, value=MetadataValue.url(experiment_url))

        if fusion_path:
            yield MetadataEntry(MlFusionTags.mlflow.ARTIFACT_PATH, value=MetadataValue.path(fusion_path))

    def load_input(self, context: InputContext):
        run = mlflow.active_run()
        if run is None:
            run_id = self._mlflow._get_current_run_id()
            if run_id is None:
                raise ValueError("No active mlflow run")
            run = mlflow.get_run(run_id=run_id)

        metadata = ArtifactMetaData(**(context.upstream_output.metadata or {}))  # type: ignore

        with TempDir() as dest_dir:
            dest_path = dest_dir.path()
            self._mlflow.tracking_client.download_artifacts(
                run_id=run.info.run_id, path=metadata.path_rel, dst_path=dest_path
            )
            local_path = Path(dest_dir.path(metadata.path_rel))

            if metadata.file_type_inferred == FileType.PICKLE:
                import cloudpickle

                with local_path.open("rb") as f:
                    data = cloudpickle.load(f)  # nosec
                return data

            elif metadata.file_type_inferred == FileType.JSON:
                with local_path.open("r") as f:
                    data = json.load(f)
                return data

            elif metadata.file_type_inferred == FileType.YAML:
                import yaml

                with local_path.open("r") as f:
                    data = yaml.safe_load(f)
                return data

            else:
                raise NotImplementedError

    def _ensure_tagged_experiment(self, asset_key: IAssetKey):
        if self._mlflow.experiment is None:
            raise ValueError("No active mlflow experiment")

        # fetch most recent experiment, since we may have tagged it since we started the run
        experiment = self._mlflow.tracking_client.get_experiment(self._mlflow.experiment.experiment_id)
        experiment_is_tagged = False
        tag_count = 0
        for key, value in experiment.tags.items():
            if MlFusionTags.ASSET_KEY in key:
                tag_count += 1
                if value == asset_key.to_string():
                    experiment_is_tagged = True

        if not experiment_is_tagged:
            if tag_count > 0:
                self._mlflow.tracking_client.set_experiment_tag(
                    self._mlflow.experiment.experiment_id,
                    f"{MlFusionTags.ASSET_KEY}.{tag_count}",
                    asset_key.to_string(),
                )
            else:
                self._mlflow.tracking_client.set_experiment_tag(
                    self._mlflow.experiment.experiment_id, MlFusionTags.ASSET_KEY, asset_key.to_string()
                )

    def _get_tagged_run(self, asset_key: IAssetKey):
        run = mlflow.active_run()
        if run is None:
            run_id = self._mlflow._get_current_run_id()
            if run_id is None:
                raise ValueError("No active mlflow run")
            run = mlflow.get_run(run_id=run_id)

        # fetch fresh tags to make sure we have the latest tags
        run = self._mlflow.tracking_client.get_run(run_id=run.info.run_id)
        tag_count = 0
        run_is_tagged = False
        for key, value in run.data.tags.items():
            if MlFusionTags.ASSET_KEY in key:
                tag_count += 1
                if value == asset_key.to_string():
                    run_is_tagged = True

        if not run_is_tagged:
            if tag_count > 0:
                mlflow.set_tag(f"{MlFusionTags.ASSET_KEY}.{tag_count}", asset_key.to_string())
            else:
                mlflow.set_tag(MlFusionTags.ASSET_KEY, json.dumps(asset_key.path))

        return run

    def _write_artifact(self, metadata: ArtifactMetaData, obj: Any):
        if metadata.file_type_inferred == FileType.PICKLE:
            import cloudpickle

            with TempDir() as src_dir:
                artifact_src_path = src_dir.path(metadata.file_name)
                with open(artifact_src_path, "wb") as f:
                    cloudpickle.dump(obj=obj, file=f)
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

        else:
            raise NotImplementedError
