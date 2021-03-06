import pyarrow as pa
import pyarrow.parquet as pq
import requests
from dagster import AssetIn, MonthlyPartitionsDefinition, OpExecutionContext, asset

_ASSET_PREFIX = ["taxi", "data"]
_BASE_URL = "https://s3.amazonaws.com/nyc-tlc/trip+data/yellow_tripdata_{}.parquet"
DATA_PARTITION = MonthlyPartitionsDefinition(start_date="2015-01-01")

_TAXI_SCHEMA_RAW = pa.schema(
    [
        pa.field("year", pa.int64()),
        pa.field("month", pa.int64()),
        pa.field("vendor_id", pa.int64()),
        pa.field("tpep_pickup_datetime", pa.timestamp("us")),
        pa.field("tpep_dropoff_datetime", pa.timestamp("us")),
        pa.field("passenger_count", pa.float64()),
        pa.field("trip_distance", pa.float64()),
        pa.field("RatecodeID", pa.float64()),
        pa.field("store_and_fwd_flag", pa.string()),
        pa.field("pu_location_id", pa.int64()),
        pa.field("do_location_id", pa.int64()),
        pa.field("payment_type", pa.int64()),
        pa.field("fare_amount", pa.float64()),
        pa.field("extra", pa.float64()),
        pa.field("mta_tax", pa.float64()),
        pa.field("tip_amount", pa.float64()),
        pa.field("tolls_amount", pa.float64()),
        pa.field("improvement_surcharge", pa.float64()),
        pa.field("total_amount", pa.float64()),
        pa.field("congestion_surcharge", pa.float64()),
        pa.field("airport_fee", pa.float64()),
    ]
)

_RENAME_MAP = {"VendorID": "vendor_id", "PULocationID": "pu_location_id", "DOLocationID": "do_location_id"}


@asset(
    key_prefix=_ASSET_PREFIX,
    io_manager_key="fusion_io",
    partitions_def=DATA_PARTITION,
    description="New York Taxi data loaded from authority.",
    metadata={"partition_by": ["year", "month"]},
    # config_schema={"iterations": int}
)
def raw(context: OpExecutionContext) -> pa.Table:
    # asset_partition_key is encoded as YYYY-MM--DD
    partition_key = context.asset_partition_key_for_output()
    url = _BASE_URL.format(partition_key[:-3])
    context.log.debug(url)
    response = requests.get(url)
    table = pq.read_table(pa.py_buffer(response.content), schema=_TAXI_SCHEMA_RAW)
    columns = [_RENAME_MAP.get(col, col) for col in table.column_names]
    table = table.rename_columns(columns)
    table = table.add_column(0, pa.field("year", pa.int64()), [[int(partition_key[:4])] * table.shape[0]])
    table = table.add_column(1, pa.field("month", pa.int64()), [[int(partition_key[5:-3])] * table.shape[0]])
    return table


@asset(
    key_prefix=_ASSET_PREFIX,
    io_manager_key="fusion_io",
    ins={"raw": AssetIn(key_prefix=_ASSET_PREFIX)},
    partitions_def=DATA_PARTITION,
    description="Filter vector for selecting test samples from dataset.",
)
def refined(context: OpExecutionContext, raw: pa.Table) -> pa.Table:
    return raw


# @asset_sensor(asset_key=AssetKey("my_table"), job=my_job)
# def my_asset_sensor(context, asset_event):
#     yield RunRequest(
#         run_key=context.cursor,
#         run_config={
#             "ops": {
#                 "read_materialization": {
#                     "config": {
#                         "asset_key": asset_event.dagster_event.asset_key.path,
#                     }
#                 }
#             }
#         },
#     )


# @asset(
#     namespace=["demo", "model_training"],
#     io_manager_key="fusion_io",
#     description="Data used for testing predictions",
# )
# def test_data(context: OpExecutionContext, dataset: pa.Table, test_filter: pa.Array) -> pa.Table:
#     return dataset.filter(test_filter)
#
#
# @asset(
#     namespace=["demo", "model_training"],
#     io_manager_key="fusion_io",
#     description="Data used for training the model",
# )
# def training_data(context: OpExecutionContext, dataset: pa.Table, test_filter: pa.Array) -> pa.Table:
#     return dataset.filter(pc.invert(test_filter))  # type: ignore
