import numpy as np
import polars as pl
import pyarrow as pa
import pyarrow.compute as pc
from dagster import OpExecutionContext, asset
from sklearn.datasets import make_regression
from sklearn.model_selection import train_test_split


@asset(
    namespace=["demo", "model_training"],
    required_resource_keys={"dataset_properties"},
    description="Randomly generated dataset for regression problem.",
)
def dataset(context: OpExecutionContext) -> pa.Table:
    n_samples = context.resources.dataset_properties.get("n_samples", 10)
    X, y = make_regression(n_samples=n_samples, n_features=10, n_informative=6, n_targets=1)  # type: ignore
    df = pl.from_records(X)
    df.columns = [f"feature_{i}" for i, _ in enumerate(df.columns)]
    return df.with_column(pl.Series("target", y)).to_arrow()


@asset(
    namespace=["demo", "model_training"],
    required_resource_keys={"dataset_properties"},
    description="Filter vector for selecting test samples from dataset.",
)
def test_filter(context: OpExecutionContext) -> pa.Array:
    n_samples = context.resources.dataset_properties.get("n_samples", 10)
    test_split = context.resources.dataset_properties.get("test_split", 0.2)
    indices = np.arange(n_samples)
    _, indices_test = train_test_split(indices, test_size=test_split)
    return pc.is_in(pa.array(indices), pa.array(indices_test))  # type: ignore


@asset(
    namespace=["demo", "model_training"],
    io_manager_key="fusion_io",
    description="Data used for testing predictions",
)
def test_data(context: OpExecutionContext, dataset: pa.Table, test_filter: pa.Array) -> pa.Table:
    return dataset.filter(test_filter)


@asset(
    namespace=["demo", "model_training"],
    io_manager_key="fusion_io",
    description="Data used for training the model",
)
def training_data(context: OpExecutionContext, dataset: pa.Table, test_filter: pa.Array) -> pa.Table:
    return dataset.filter(pc.invert(test_filter))  # type: ignore
