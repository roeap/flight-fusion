import pyarrow as pa
import pytest
from dagster import Out, ResourceDefinition, graph, op
from dagster_fusion import flight_fusion_io_manager
from flight_fusion import FusionServiceClient


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
    @op(out={"out_a": Out(dagster_type=pa.Table)})
    def solid_a(_context):
        return test_data

    @op(out={"out_b": Out(dagster_type=pa.Table)})
    def solid_b(_context, df):
        return df

    @graph
    def asset_pipeline():
        solid_b(solid_a())  # type: ignore

    return asset_pipeline


run_config = {
    "ops": {
        "solid_a": {"outputs": {"out_a": {"location": {"key": "scope/out_a"}}}},
        "solid_b": {"outputs": {"out_b": {"location": {"key": "scope/out_b"}}}},
    }
}


def test_graph_in_out(test_graph, test_data, ff_client: FusionServiceClient):
    fusion_client = ResourceDefinition.hardcoded_resource(ff_client)
    job = test_graph.to_job(
        resource_defs={"io_manager": flight_fusion_io_manager, "fusion_client": fusion_client},
        config=run_config,
    )

    result = job.execute_in_process()
    assert result.success

    handled_output_events = list(filter(lambda evt: evt.is_handled_output, result.all_node_events))  # type: ignore
    assert len(handled_output_events) == 2
    out_a = result.output_for_node("solid_a", "out_a")

    assert out_a.equals(test_data)

    fds = ff_client.get_dataset_client(name="out_b", areas=["scope"])
    result_table = fds.load()
    assert result_table.equals(test_data)
