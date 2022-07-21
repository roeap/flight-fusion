from dagster import load_assets_from_modules, with_resources

from model_training.resources import RESOURCES_LOCAL

from . import data
from .data import DATA_PARTITION

data_assets = load_assets_from_modules([data], group_name="data_engineering")
data_assets_with_resources = with_resources(data_assets, resource_defs=RESOURCES_LOCAL)
