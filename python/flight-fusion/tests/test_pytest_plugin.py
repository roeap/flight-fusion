import numpy as np
import pandas as pd

from flight_fusion import FusionServiceClient
from flight_fusion.asset_key import AssetKey
from flight_fusion.ipc.v1alpha1 import SaveMode


def test_roundtrip(fusion_client: FusionServiceClient):
    np.random.seed(42)
    df = pd.DataFrame(np.random.randn(5, 3), columns=["col1", "col2", "col3"])

    fds = fusion_client.get_dataset_client(AssetKey(["test_roundtrip", "table"]))
    fds.write_into(df, SaveMode.SAVE_MODE_OVERWRITE)

    df_loaded = fds.load().to_pandas()

    assert df.equals(df_loaded)
