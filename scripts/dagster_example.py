"""Example Dagster Job and Software Defined Assets integrating with flight fusion service.
"""
import warnings

import dagster

warnings.filterwarnings("ignore", category=dagster.ExperimentalWarning)

import pyarrow as pa  # noqa
from dagster import (  # noqa
    AssetGroup,
    AssetIn,
    AssetKey,
    In,
    Out,
    SkipReason,
    SourceAsset,
    asset,
    graph,
    op,
    repository,
    schedule,
)

from dagster_fusion import (  # noqa
    flight_fusion_io_manager,
    flight_fusion_loader,
    flight_fusion_resource,
)

# Job definitions
#
# Jobs are defined using the ops/graph/job abstractions from Dagster.


@op(
    ins={
        "table1": In(
            asset_key=AssetKey(["static", "table1"]),
            root_manager_key="fusion_loader",
            dagster_type=pa.Table,
        )
    },
    out={"out_a": Out(asset_key=AssetKey(["scope", "out_a"]), dagster_type=pa.Table)},
)
def solid_a(_context, table1):
    return table1


@op(
    ins={
        "df": In(
            asset_key=AssetKey(["scope", "out_a"]),
            dagster_type=pa.Table,
        )
    },
    out={"out_b": Out(asset_key=AssetKey(["scope", "out_b"]), dagster_type=pa.Table)},
)
def solid_b(_context, df):
    return df


@graph
def asset_pipeline():
    solid_b(solid_a())  # type: ignore


job = asset_pipeline.to_job(
    description="Demo job showcasing features of `dagster-fusion`",
    resource_defs={
        "io_manager": flight_fusion_io_manager,
        "fusion_loader": flight_fusion_loader,
        "fusion_client": flight_fusion_resource.configured({"host": "127.0.0.1", "port": 50051}),
    },
    config={
        "ops": {
            "solid_a": {
                "outputs": {"out_a": {"save_mode": "SAVE_MODE_OVERWRITE"}},
            },
            "solid_b": {"outputs": {"out_b": {"save_mode": "SAVE_MODE_OVERWRITE"}}},
        }
    },
)


@schedule(cron_schedule="45 23 * * 6", job=job)
def job_schedule():
    return SkipReason("Not implemented")


# Asset definitions
#
# Jobs are defined using the asset abstractions from Dagster.

table_1 = SourceAsset(
    key=AssetKey(["static", "table1"]),
    description="A static table used for demo purposes",
)


@asset(
    name="result1",
    namespace=["computed"],
    ins={"table_1": AssetIn(asset_key=AssetKey(["static", "table1"]))},
    description="Process external input table",
    required_resource_keys={"fusion_client"},
)
def compute_result1(_context, table_1: pa.Table) -> pa.Table:
    return table_1


@asset(
    name="result2",
    namespace=["computed"],
    ins={"table_1": AssetIn(asset_key=AssetKey(["static", "table1"]))},
    description="Process external input table",
    required_resource_keys={"fusion_client"},
)
def compute_result2(_context, table_1: pa.Table) -> pa.Table:
    return table_1


pipeline_assets = AssetGroup(
    assets=[compute_result1, compute_result2],
    source_assets=[table_1],
    resource_defs={
        "io_manager": flight_fusion_io_manager,
        "fusion_client": flight_fusion_resource.configured({"host": "127.0.0.1", "port": 50051}),
    },
)


@schedule(
    cron_schedule="45 23 * * 6",
    job=pipeline_assets.build_job(name="asset_schedule_job", selection=["*computed.result2"]),
)
def asset_schedule():
    return SkipReason("Not implemented")


@repository
def software_defined_assets():
    return [pipeline_assets, job, job_schedule, asset_schedule]
