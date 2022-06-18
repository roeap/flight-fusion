from pathlib import Path

import pytest
import requests_mock
from dagster import JobDefinition, ResourceDefinition, define_asset_job, with_resources

from dagster_fusion import flight_fusion_io_manager
from flight_fusion import FusionServiceClient
from model_training.assets import data_assets
from model_training.assets.data import DATA_PARTITION
from model_training.jobs import job_data_prep_unresolved


@pytest.fixture
def data_mocker(shared_datadir: Path):
    with requests_mock.Mocker() as m:
        taxi_dir = shared_datadir / "taxi"
        with (taxi_dir / "2015-01.parquet").open("rb") as f_:
            data = f_.read()

        m.get("https://s3.amazonaws.com/nyc-tlc/trip+data/yellow_tripdata_2015-01.parquet", content=data)
        yield m


@pytest.fixture
def test_assets(data_mocker, fusion_client: FusionServiceClient):
    client = ResourceDefinition.hardcoded_resource(fusion_client)
    return with_resources(
        data_assets,
        resource_defs={"fusion_io": flight_fusion_io_manager, "fusion_client": client},
    )


@pytest.fixture
def raw_assets_job(test_assets):
    unresolved = define_asset_job("test", selection="taxi/data/raw", partitions_def=DATA_PARTITION)
    return unresolved.resolve(test_assets, [])


@pytest.fixture
def data_prep_job(test_assets):
    return job_data_prep_unresolved.resolve(test_assets, [])


def test_asset_load(raw_assets_job: JobDefinition):
    # TODO figure out how to generate valid op context for partitioned asset,
    # so we don't have to test it as a job
    result = raw_assets_job.execute_in_process(partition_key="2015-01-01", run_config={})
    table = result.output_for_node("taxi__data__raw")
    assert table.shape == (100, 19)


def test_data_prep(data_prep_job: JobDefinition):
    result = data_prep_job.execute_in_process(partition_key="2015-01-01")
    table = result.output_for_node("taxi__data__refined").to_arrow()
    assert table.shape == (100, 19)
