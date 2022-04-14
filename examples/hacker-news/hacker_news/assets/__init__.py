from dagster import AssetGroup
from hacker_news.resources.hn_resource import (
    hn_api_subsample_client,
    hn_snapshot_client,
)
from hacker_news.resources.partition_bounds import partition_bounds

from dagster_fusion import (  # noqa
    flight_fusion_io_manager,
    flight_fusion_loader,
    flight_fusion_resource,
)

from .download_items import build_comments, build_stories, download_items

hacker_assets = AssetGroup(
    assets=[download_items, build_comments, build_stories],
    # source_assets=[table_1],
    resource_defs={
        "io_manager": flight_fusion_io_manager,
        "fusion_client": flight_fusion_resource.configured({"host": "127.0.0.1", "port": 50051}),
        "partition_bounds": partition_bounds.configured(
            {
                "start": "2020-12-30 00:00:00",
                "end": "2020-12-30 01:00:00",
            }
        ),
        "hn_client": hn_snapshot_client,
    },
)
