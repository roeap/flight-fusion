from dagster import repository

from .assets import local_assets
from .jobs import model_training_local_job


@repository
def model_training_local():
    return [local_assets, model_training_local_job]
