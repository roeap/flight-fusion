from dagster_fusion import flight_fusion_io_manager, flight_fusion_resource

from .dataset_spec import dataset_properties

RESOURCES_LOCAL = {
    "fusion_io_manager": flight_fusion_io_manager,
    "fusion_client": flight_fusion_resource.configured({"host": "127.0.0.1", "port": 50051}),
    "dataset_properties": dataset_properties.configured(
        {
            "n_samples": 1000,
            "test_split": 0.3,
        }
    ),
}
