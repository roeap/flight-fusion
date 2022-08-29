import json
from pathlib import Path
from uuid import uuid4

from dagster import AssetIn, AssetKey, asset, define_asset_job, with_resources

import mlflow
from dagster_fusion import (
    mlflow_tracking,
    mlfusion_configuration,
    model_artifact_io_manager,
)
from dagster_fusion.hooks import end_mlflow_on_run_finished
from flight_fusion.tags import MlFusionTags


def test_artifacts_io_manager(mocker, datadir: Path):
    mocker.patch("mlflow.set_tag")
    mlflow.set_tracking_uri(f"file://{datadir.as_posix()}")

    _id = f"{uuid4()}".replace("-", "")
    run_tracker = {}

    @asset(name=f"upstream_{_id}", required_resource_keys={"mlflow"})
    def upstream(context):
        active_run = mlflow.active_run()
        if active_run:
            run_tracker["run_id"] = active_run.info.run_id
            run_tracker["experiment_id"] = active_run.info.experiment_id
        return 2

    @asset(
        name=f"downstream_{_id}",
        ins={"upstream": AssetIn(asset_key=AssetKey([f"upstream_{_id}"]))},
        metadata={"file_name": "model.pkl"},
        io_manager_key="model_artifact_io",
        required_resource_keys={"mlflow"},
    )
    def downstream(upstream):
        assert upstream == 2
        return 1 + upstream

    assets = with_resources(
        [upstream, downstream],
        resource_defs={
            "mlfusion_config": mlfusion_configuration,
            "mlflow": mlflow_tracking,
            "model_artifact_io": model_artifact_io_manager,
        },
    )

    asset_job = end_mlflow_on_run_finished(define_asset_job("test_job").resolve(assets, []))
    run_config = {"resources": {"mlflow": {"config": {"experiment_name": "testing"}}}}
    result = asset_job.execute_in_process(run_config=run_config)
    assert result.success

    mlflow.set_tag.assert_called_once_with(MlFusionTags.ASSET_KEY, json.dumps([f"downstream_{_id}"]))

    artifact_path = datadir / run_tracker["experiment_id"] / run_tracker["run_id"] / "artifacts/model.pkl"
    assert artifact_path.exists()


def test_artifacts_io_manager_multiple_tags(datadir: Path):
    # mocker.patch("mlflow.set_tag")
    mlflow.set_tracking_uri(f"file://{datadir.as_posix()}")

    # _id = f"{uuid4()}".replace("-", "")
    run_tracker = {}

    @asset(name="upstream", required_resource_keys={"mlflow"})
    def upstream(context):
        active_run = mlflow.active_run()
        if active_run:
            run_tracker["run_id"] = active_run.info.run_id
            run_tracker["experiment_id"] = active_run.info.experiment_id
        return 2

    @asset(
        name="downstream",
        ins={"upstream": AssetIn(asset_key=AssetKey(["upstream"]))},
        metadata={"file_name": "model.pkl"},
        io_manager_key="model_artifact_io",
        required_resource_keys={"mlflow"},
    )
    def downstream(upstream):
        assert upstream == 2
        return 1 + upstream

    @asset(
        name="downstream2",
        ins={"downstream": AssetIn(asset_key=AssetKey(["downstream"]))},
        metadata={"file_name": "model2.pkl"},
        io_manager_key="model_artifact_io",
        required_resource_keys={"mlflow"},
    )
    def downstream2(downstream):
        assert downstream == 3
        return 1 + downstream

    assets = with_resources(
        [upstream, downstream, downstream2],
        resource_defs={  # type: ignore
            "mlfusion_config": mlfusion_configuration,
            "mlflow": mlflow_tracking,
            "model_artifact_io": model_artifact_io_manager,
        },
    )

    asset_job = end_mlflow_on_run_finished(define_asset_job("test_job").resolve(assets, []))
    run_config = {"resources": {"mlflow": {"config": {"experiment_name": "testing"}}}}
    result = asset_job.execute_in_process(run_config=run_config)
    assert result.success

    tags_path = datadir / run_tracker["experiment_id"] / run_tracker["run_id"] / "tags"
    assert (tags_path / MlFusionTags.ASSET_KEY).open("r").read() == json.dumps(["downstream"])
    assert (tags_path / f"{MlFusionTags.ASSET_KEY}.1").open("r").read() == json.dumps(["downstream2"])
