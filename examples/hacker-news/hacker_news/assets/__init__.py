from dagster import AssetGroup, in_process_executor
from hacker_news.resources import RESOURCES_LOCAL  # , RESOURCES_PROD, RESOURCES_STAGING
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

from .comment_stories import comment_stories
from .id_range_for_time import id_range_for_time
from .items import comments, items, stories
from .recommender_model import component_top_stories, recommender_model
from .user_story_matrix import user_story_matrix

# prod_assets = AssetGroup.from_package_name(__name__, resource_defs=RESOURCES_PROD)
# staging_assets = AssetGroup.from_package_name(__name__, resource_defs=RESOURCES_STAGING)
local_assets = AssetGroup(
    assets=[
        comments,
        stories,
        items,
        comment_stories,
        id_range_for_time,
        user_story_matrix,
        recommender_model,
        component_top_stories,
    ],
    resource_defs=RESOURCES_LOCAL,
    executor_def=in_process_executor,  # type: ignore
)

# hacker_assets = AssetGroup(
#     assets=[comments, stories, items],
#     # source_assets=[table_1],
#     resource_defs={
#         "io_manager": flight_fusion_io_manager,
#         "fusion_client": flight_fusion_resource.configured({"host": "127.0.0.1", "port": 50051}),
#         "partition_bounds": partition_bounds.configured(
#             {
#                 "start": "2020-12-30 00:00:00",
#                 "end": "2020-12-30 01:00:00",
#             }
#         ),
#         "hn_client": hn_snapshot_client,
#     },
# )
