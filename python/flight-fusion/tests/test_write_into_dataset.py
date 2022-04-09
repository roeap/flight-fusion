import numpy as np
import pandas as pd

from flight_fusion import FusionServiceClient
from flight_fusion.ipc.v1alpha1 import SaveMode


def test_roundtrip(ff_client: FusionServiceClient):
    np.random.seed(42)
    df = pd.DataFrame(np.random.randn(5, 3), columns=["col1", "col2", "col3"])

    fds = ff_client.get_dataset_client(name="new_dataset", areas=["asd", "fgh"])
    fds.write_into(df, SaveMode.SAVE_MODE_OVERWRITE)

    df_loaded = fds.load().to_pandas()

    assert df.equals(df_loaded)


def test_save_mode(ff_client: FusionServiceClient):
    np.random.seed(42)
    df = pd.DataFrame(np.random.randn(5, 3), columns=["col1", "col2", "col3"])

    fds = ff_client.get_dataset_client(name="new_dataset", areas=["test_save_mode"])

    fds.write_into(df)
    df_loaded = fds.load().to_pandas()
    assert df_loaded.shape == (5, 3)

    fds.write_into(df, SaveMode.SAVE_MODE_OVERWRITE)
    df_loaded = fds.load().to_pandas()
    assert df_loaded.shape == (5, 3)

    fds.write_into(df, SaveMode.SAVE_MODE_APPEND)
    df_loaded = fds.load().to_pandas()
    assert df_loaded.shape == (10, 3)
