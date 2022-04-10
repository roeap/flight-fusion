import pyarrow as pa
from dagster import Out, graph, op

from dagster_fusion import flight_fusion_io_manager, flight_fusion_resource

_FUSION_CLIENT = flight_fusion_resource.configured({"host": "127.0.0.1", "port": 50051})
_RUN_CONFIG = {
    "ops": {
        "solid_a": {
            "outputs": {
                "out_a": {"location": {"key": "scope/out_a"}, "save_mode": "SAVE_MODE_OVERWRITE"}
            }
        },
        "solid_b": {
            "outputs": {
                "out_b": {"location": {"key": "scope/out_b"}, "save_mode": "SAVE_MODE_OVERWRITE"}
            }
        },
    }
}


@op(out={"out_a": Out(dagster_type=pa.Table)})
def solid_a(_context):
    data1 = [
        pa.array([-10, -5, 0, 5, 10], type=pa.int64()),
        pa.array([-100, -50, 0, 50, 100], type=pa.int64()),
    ]
    table = pa.Table.from_arrays(data1, names=["a", "b"])
    return table


@op(out={"out_b": Out(dagster_type=pa.Table)})
def solid_b(_context, df):
    return df


@graph
def asset_pipeline():
    solid_b(solid_a())  # type: ignore


job = asset_pipeline.to_job(
    resource_defs={"io_manager": flight_fusion_io_manager, "fusion_client": _FUSION_CLIENT},
    config=_RUN_CONFIG,
)
