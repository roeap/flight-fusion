from dagster import define_asset_job, repository

from model_training.assets import data_assets_with_resources


@repository
def model_training_local():
    return [
        *data_assets_with_resources,
        define_asset_job("data_update", selection="*taxi/data/refined"),
    ]
