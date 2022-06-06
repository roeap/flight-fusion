# ML Fusion

MlFusion combines multiple state of the art technologies into a single,
opinionated platform for end to end data science workflows.

- A unified client to meet all your data needs
- testable - pytest integration

## Getting Started

```py
from flight_fusion import FusionServiceClient, ClientOptions, SaveMode
import numpy as np
import pandas as pd

np.random.seed(42)

# and create an instance of the service client
ffc = FusionServiceClient(ClientOptions(host="localhost", port=50051))

# when interacting with a single dataset, you need a dataset client
fds = ffc.get_dataset_client(name="new_dataset", areas=["demo"])
```

```py
# write data into dataset
df_example = pd.DataFrame(np.random.randn(5, 3), columns=["col1", "col2", "col3"])
fds.write_into(df_example, SaveMode.SAVE_MODE_OVERWRITE)
```

## Configuration

- fusion server
- artifacts server
- mlflow server
- mlserver server

## Dagster Asset IO Managers

### model artifacts

- autolog -> still use io manager to assign asset tags
  - call log_model method
- pure version -> optionally yield artifacts ...

### datasets

- use metadata to configure asset behavior

## Versioning

- data via delta
- models via runs / experiments

## CLI

`fusion server start`

: start a new instance of the flight-fusion service

`fusion server mlflow`

: start the mlflow server
new line
