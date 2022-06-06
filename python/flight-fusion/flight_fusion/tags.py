class _MlFlowTags:
    RUN_ID: str = "mlfusion.mlflow.run_id"
    EXPERIMENT_ID: str = "mlfusion.mlflow.experiment_id"
    ARTIFACT_PATH: str = "mlfusion.mlflow.artifact_path"


class _DagsterTags:
    RUN_ID: str = "mlfusion.dagster.run_id"


class MlFusionTags:
    ASSET_KEY: str = "mlfusion.asset_key"
    mlflow: _MlFlowTags = _MlFlowTags()
    dagster: _DagsterTags = _DagsterTags()
