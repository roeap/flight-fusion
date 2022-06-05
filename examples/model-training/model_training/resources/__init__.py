from dagster_mlflow import mlflow_tracking

from dagster_fusion import (
    flight_fusion_resource,
    mlfusion_configuration,
    mlfusion_io_manager,
)

from .dataset_spec import dataset_properties

RESOURCES_LOCAL = {
    "mlfusion_config": mlfusion_configuration,
    "fusion_io_manager": mlfusion_io_manager,
    "fusion_client": flight_fusion_resource,
    "dataset_properties": dataset_properties.configured(
        {
            "n_samples": 1000,
            "test_split": 0.3,
        }
    ),
    "mlflow": mlflow_tracking.configured(
        {"experiment_name": "demo_experiment", "mlflow_tracking_uri": "http://localhost:5000"}
    ),
}
