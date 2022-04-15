from dagster_fusion import flight_fusion_io_manager, flight_fusion_resource

from .hn_resource import hn_api_client, hn_api_subsample_client, hn_snapshot_client

RESOURCES_LOCAL = {
    "fusion_io_manager": flight_fusion_io_manager,
    "fusion_client": flight_fusion_resource.configured({"host": "127.0.0.1", "port": 50051}),
    "hn_client": hn_api_subsample_client.configured({"sample_rate": 10}),
}
