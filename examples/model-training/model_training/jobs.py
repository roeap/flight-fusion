from datetime import datetime

from dagster import (
    AssetGroup,
    JobDefinition,
    define_asset_job,
    monthly_partitioned_config,
    monthly_schedule,
)

from dagster_fusion.hooks import end_mlflow_on_run_finished
from model_training.assets import DATA_PARTITION


def make_model_training_job(asset_group: AssetGroup) -> JobDefinition:
    return end_mlflow_on_run_finished(
        asset_group.build_job(
            name="model_training",
            selection=["*demo/model_training/model_performance"],
            tags={
                "dagster-k8s/config": {
                    "container_config": {
                        "resources": {
                            "requests": {"cpu": "500m", "memory": "2Gi"},
                        }
                    },
                }
            },
        )
    )


@monthly_partitioned_config(start_date=datetime(2020, 1, 1))
def taxi_load_config(start: datetime, _end: datetime):
    return {"ops": {"raw": {"config": {"date": start.strftime("%Y-%m-%d")}}}}


# data loading job
job_data_prep_unresolved = define_asset_job(
    "data_refinement", selection="*taxi/data/refined", partitions_def=DATA_PARTITION
)

schedule_data_prep = monthly_schedule("data_load", DATA_PARTITION.start)
