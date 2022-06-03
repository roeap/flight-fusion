import pandas as pd
import polars as pl
import pyarrow as pa
from dagster import IOManager, io_manager
from dagster_fusion.config import FIELD_COLUMN_SELECTION, FIELD_SAVE_MODE
from flight_fusion import FusionServiceClient

from .table_io_manager import TableIOManager

_INPUT_CONFIG_SCHEMA = {"columns": FIELD_COLUMN_SELECTION}
_OUTPUT_CONFIG_SCHEMA = {"save_mode": FIELD_SAVE_MODE}


class MlFusionIOManager(IOManager):
    def __init__(self, client: FusionServiceClient) -> None:
        self._fusion = client
        self._data_handler = TableIOManager(self._fusion)

    def load_input(self, context):
        return super().load_input(context)

    def handle_output(self, context, obj):
        if isinstance(obj, (pd.DataFrame, pl.DataFrame, pa.Table)):
            self._data_handler.handle_output(context=context, obj=obj)
        raise NotImplementedError


@io_manager(
    description="IO Manager for handling mlfusion assets in dagster pipelines.",
    input_config_schema=_INPUT_CONFIG_SCHEMA,
    output_config_schema=_OUTPUT_CONFIG_SCHEMA,
    required_resource_keys={"fusion_client"},
)
def mlfusion_io_manager(init_context):
    return TableIOManager(init_context.resources.fusion_client)
