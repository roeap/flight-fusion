import numpy as np
import pandas as pd
import pytest

from flight_fusion import FusionServiceClient
from flight_fusion.asset_key import AssetKey
from flight_fusion.ipc.v1alpha1 import SaveMode


@pytest.fixture
def df():
    np.random.seed(42)
    return pd.DataFrame(np.random.randn(5, 3), columns=["col1", "col2", "col3"])


def test_roundtrip(fusion_client: FusionServiceClient, df: pd.DataFrame):
    fds = fusion_client.new_dataset_client(AssetKey(["test_roundtrip", "data"]), versioned=False)
    fds.write_into(df, SaveMode.SAVE_MODE_OVERWRITE)

    df_loaded = fds.load().to_pandas()

    assert df.equals(df_loaded)


def test_save_mode(fusion_client: FusionServiceClient, df: pd.DataFrame):
    fds = fusion_client.new_dataset_client(AssetKey(["test_save_mode", "data"]), versioned=False)

    fds.write_into(df, SaveMode.SAVE_MODE_OVERWRITE)
    df_loaded = fds.load().to_pandas()
    assert df_loaded.shape == (5, 3)

    fds.write_into(df, SaveMode.SAVE_MODE_OVERWRITE)
    df_loaded = fds.load().to_pandas()
    assert df_loaded.shape == (5, 3)

    fds.write_into(df, SaveMode.SAVE_MODE_APPEND)
    df_loaded = fds.load().to_pandas()
    assert df_loaded.shape == (10, 3)


def test_query(fusion_client: FusionServiceClient, df: pd.DataFrame):
    fds = fusion_client.new_dataset_client(AssetKey(["test_query", "data"]), versioned=False)

    fds.write_into(df, SaveMode.SAVE_MODE_OVERWRITE)
    df_query = fds.query("SELECT avg(col1) FROM data").to_pandas()
    assert df_query.shape == (1, 1)


def test_load_columns(fusion_client: FusionServiceClient, df: pd.DataFrame):
    fds = fusion_client.new_dataset_client(AssetKey(["test_load_columns", "data"]), versioned=False)

    fds.write_into(df, SaveMode.SAVE_MODE_OVERWRITE)
    df_query = fds.load(columns=["col1", "col3"]).to_pandas()
    assert df_query.shape == (5, 2)
