import numpy as np
import pandas as pd
import pytest
from flight_fusion import FlightFusionClient


@pytest.mark.integration
def test_write_into_dataset(ff_client: FlightFusionClient):
    np.random.seed(42)
    df = pd.DataFrame(np.random.randn(5, 3), columns=["col1", "col2", "col3"])

    fds = ff_client.get_dataset_client(name="new_dataset", areas=["asd", "fgh"])
    fds.write_into(df)

    df_loaded = fds.load().to_pandas()

    assert df.equals(df_loaded)
