import numpy as np
import pandas as pd

from flight_fusion import ClientOptions, FusionServiceClient
from flight_fusion.ipc.v1alpha1 import SaveMode

np.random.seed(42)
df_example = pd.DataFrame(np.random.randn(5, 3), columns=["col1", "col2", "col3"])

# and create an instance of the service client
options = ClientOptions(host="localhost", port=50051)
ffc = FusionServiceClient(options)

ffc.get_dataset_client(name="table1", areas=["static"]).write_into(
    df_example, SaveMode.SAVE_MODE_OVERWRITE
)
