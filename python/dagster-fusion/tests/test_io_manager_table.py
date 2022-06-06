import pyarrow as pa
import pytest
from dagster import AssetKey, In, Out, ResourceDefinition, graph, op
from flight_fusion import FusionServiceClient

from dagster_fusion import flight_fusion_io_manager


@pytest.fixture
def test_data():
    data1 = [
        pa.array([-10, -5, 0, 5, 10], type=pa.int64()),
        pa.array([-100, -50, 0, 50, 100], type=pa.int64()),
    ]
    table = pa.Table.from_arrays(data1, names=["a", "b"])
    return table


@pytest.fixture
def test_graph(test_data):
    @op(out={"out_a": Out(dagster_type=pa.Table, asset_key=AssetKey(["scope", "out_a"]))})
    def solid_a(_context):
        return test_data

    @op(out={"out_b": Out(dagster_type=pa.Table, asset_key=AssetKey(["scope", "out_b"]))})
    def solid_b(_context, df):
        return df

    @graph
    def asset_pipeline():
        solid_b(solid_a())  # type: ignore

    return asset_pipeline


@pytest.fixture
def test_graph_columns(test_data):
    @op(
        ins={
            "df": In(
                dagster_type=pa.Table,
                asset_key=AssetKey(["scope", "out_b"]),
                metadata={"columns": ["b"]},
            )
        },
        out={"out_c": Out(dagster_type=pa.Table, asset_key=AssetKey(["scope", "out_c"]))},
    )
    def solid_load(_context, df):
        return df

    @graph
    def asset_pipeline():
        solid_load()

    return asset_pipeline


run_config = {
    "ops": {
        "solid_a": {"outputs": {"out_a": {"save_mode": "SAVE_MODE_OVERWRITE"}}},
        "solid_b": {"outputs": {"out_b": {"save_mode": "SAVE_MODE_OVERWRITE"}}},
    }
}


def test_graph_in_out(test_graph, test_data, fusion_client: FusionServiceClient):
    client = ResourceDefinition.hardcoded_resource(fusion_client)
    job = test_graph.to_job(
        resource_defs={"io_manager": flight_fusion_io_manager, "fusion_client": client},
        config=run_config,
    )

    result = job.execute_in_process()
    assert result.success

    handled_output_events = list(filter(lambda evt: evt.is_handled_output, result.all_node_events))  # type: ignore
    assert len(handled_output_events) == 2
    out_a = result.output_for_node("solid_a", "out_a")

    assert out_a.equals(test_data)

    fds = fusion_client.get_dataset_client(AssetKey(["scope", "out_b"]))  # type: ignore
    result_table = fds.load()
    assert result_table.equals(test_data)


def test_column_selection(test_graph, test_graph_columns, test_data: pa.Table, fusion_client: FusionServiceClient):
    client = ResourceDefinition.hardcoded_resource(fusion_client)
    job = test_graph.to_job(
        resource_defs={"io_manager": flight_fusion_io_manager, "fusion_client": client},
        config=run_config,
    )

    result = job.execute_in_process()
    assert result.success

    job_cols = test_graph_columns.to_job(
        resource_defs={"io_manager": flight_fusion_io_manager, "fusion_client": client},
    )

    result = job_cols.execute_in_process()
    assert result.success

    out = result.output_for_node("solid_load", "out_c")
    assert out.equals(test_data.select(["b"]))
