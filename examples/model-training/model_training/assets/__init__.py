from dagster import AssetGroup, in_process_executor
from model_training.resources import RESOURCES_LOCAL

from .model_training import ml_model, model_performance
from .training_data import dataset, test_data, test_filter, training_data

local_assets = AssetGroup(
    assets=[dataset, test_data, training_data, test_filter, ml_model, model_performance],
    resource_defs=RESOURCES_LOCAL,
    executor_def=in_process_executor,  # type: ignore
)
