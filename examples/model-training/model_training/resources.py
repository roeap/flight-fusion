from dagster import make_values_resource

from dagster_fusion import (
    flight_fusion_io_manager,
    flight_fusion_resource,
    mlflow_tracking,
    mlfusion_configuration,
    model_artifact_io_manager,
)

dataset_properties = make_values_resource(n_samples=int, test_split=float)


RESOURCES_LOCAL = {
    "mlfusion_config": mlfusion_configuration,
    "fusion_io": flight_fusion_io_manager,
    "model_artifact_io": model_artifact_io_manager,
    "fusion_client": flight_fusion_resource,
    "dataset_properties": dataset_properties.configured(
        {
            "n_samples": 1000,
            "test_split": 0.3,
        }
    ),
    "mlflow": mlflow_tracking.configured({"experiment_name": "demo_experiment"}),
}
