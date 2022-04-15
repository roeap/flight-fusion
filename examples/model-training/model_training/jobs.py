from dagster import AssetGroup, JobDefinition
from model_training.assets import local_assets  # , prod_assets, staging_assets


def make_model_training_job(asset_group: AssetGroup) -> JobDefinition:
    return asset_group.build_job(
        name="model_training",
        selection=["*demo.model_training.model_performance"],
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


model_training_local_job = make_model_training_job(local_assets)