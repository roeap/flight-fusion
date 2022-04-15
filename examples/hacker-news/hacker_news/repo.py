from dagster import repository

from .assets import local_assets
from .jobs import download_local_job

# from .jobs.story_recommender import (
#     story_recommender_local_job,
#     story_recommender_prod_job,
#     story_recommender_staging_job,
# )
# from .sensors import make_hn_tables_updated_sensor

# from .sensors.slack_on_failure_sensor import make_slack_on_failure_sensor


@repository
def hacker_news_local():
    return [
        local_assets,
        download_local_job,
    ]  # , activity_stats_local_job, story_recommender_local_job]


# @repository
# def hacker_news_prod():
#     return [
#         build_schedule_from_partitioned_job(download_prod_job),
#         # make_slack_on_failure_sensor(base_url="my_prod_dagit_url.com"),
#         make_hn_tables_updated_sensor(story_recommender_prod_job),
#         make_hn_tables_updated_sensor(dbt_prod_job),
#     ]
#
#
# @repository
# def hacker_news_staging():
#     return [
#         build_schedule_from_partitioned_job(download_staging_job),
#         # make_slack_on_failure_sensor(base_url="my_staging_dagit_url.com"),
#         make_hn_tables_updated_sensor(story_recommender_staging_job),
#         make_hn_tables_updated_sensor(dbt_staging_job),
#     ]
