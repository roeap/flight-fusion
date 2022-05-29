import numpy as np
import pandas as pd

from flight_fusion import FusionServiceClient
from flight_fusion.asset_key import AssetKey
from flight_fusion.ipc.v1alpha1 import SaveMode


def test_roundtrip(fusion_client: FusionServiceClient):
    np.random.seed(42)
    df = pd.DataFrame(np.random.randn(5, 3), columns=["col1", "col2", "col3"])

    fds = fusion_client.get_versioned_dataset_client(
        AssetKey(["test_roundtrip_versioned", "table"])
    )
    fds.write_into(df, SaveMode.SAVE_MODE_OVERWRITE)

    df_loaded = fds.load().to_pandas()

    assert df.equals(df_loaded)


def test_save_mode(fusion_client: FusionServiceClient):
    np.random.seed(42)
    df = pd.DataFrame(np.random.randn(5, 3), columns=["col1", "col2", "col3"])

    fds = fusion_client.get_versioned_dataset_client(
        AssetKey(["test_save_mode", "table"])
    )

    fds.write_into(df, SaveMode.SAVE_MODE_OVERWRITE)
    df_loaded = fds.load().to_pandas()
    assert df_loaded.shape == (5, 3)

    fds.write_into(df, SaveMode.SAVE_MODE_OVERWRITE)
    df_loaded = fds.load().to_pandas()
    assert df_loaded.shape == (5, 3)

    fds.write_into(df, SaveMode.SAVE_MODE_APPEND)
    df_loaded = fds.load().to_pandas()
    assert df_loaded.shape == (10, 3)


def test_load_columns(fusion_client: FusionServiceClient):
    np.random.seed(42)
    df = pd.DataFrame(np.random.randn(5, 3), columns=["col1", "col2", "col3"])

    fds = fusion_client.get_versioned_dataset_client(
        AssetKey(["test_load_columns", "data"])
    )

    fds.write_into(df, SaveMode.SAVE_MODE_OVERWRITE)
    df_query = fds.load(columns=["col1", "col3"]).to_pandas()
    assert df_query.shape == (5, 2)
