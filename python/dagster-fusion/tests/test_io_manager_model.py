import json
from pathlib import Path
from uuid import uuid4

import mlflow
from dagster import AssetGroup, AssetIn, AssetKey, asset

from dagster_fusion import (
    mlflow_tracking,
    mlfusion_configuration,
    model_artifact_io_manager,
)
from dagster_fusion.hooks import end_mlflow_on_run_finished
from dagster_fusion.io.artifact import _TAG_ASSET_KEY


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
        io_manager_key="artifact_io",
        required_resource_keys={"mlflow"},
    )
    def downstream(upstream):
        assert upstream == 2
        return 1 + upstream

    asset_group = AssetGroup(
        [upstream, downstream],
        resource_defs={  # type: ignore
            "mlfusion_config": mlfusion_configuration,
            "mlflow": mlflow_tracking,
            "artifact_io": model_artifact_io_manager,
        },
    )
    asset_job = end_mlflow_on_run_finished(asset_group.build_job(name="my_asset_job"))

    run_config = {"resources": {"mlflow": {"config": {"experiment_name": "testing"}}}}

    result = asset_job.execute_in_process(run_config=run_config)
    assert result.success

    mlflow.set_tag.assert_called_once_with(_TAG_ASSET_KEY, json.dumps([f"downstream_{_id}"]))

    artifact_path = datadir / run_tracker["experiment_id"] / run_tracker["run_id"] / "artifacts/model.pkl"
    assert artifact_path.exists()
