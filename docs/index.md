# ML Fusion

MlFusion combines multiple state of the art technologies into a single,
opinionated platform for end to end data science workflows.

- A unified client to meet all your data needs
- testable - pytest integration

## Getting Started

This will walk you through the different steps of building an end to end data pipeline
with mlfusion.

When working locally, we have the option to work on a project basis, or against a "global"
instance, where all local (or multiple) local projects manage their data. For this tutorial
it is recommended to use an isolated workspace, for general work, we hope that a global
workspace is the more convenient way of interacting.

Right now we rely on git to discover data directories, so create a new folder and initialize
a git repository there.

```sh
cd /path/to/my/project/folder && git init
```

within the folder, create a new virtual environment.

```sh
venv .venv
.venv/bin/activate
```

Finally we can install mlfusion

```sh
poetry add flight-fusion --extras full
pip install flight-fusion[full]
```

and initialize the mlfusion workspace.

```sh
fusion env init
```

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

The concept of assets within the entire platform is completely compatible with - in fact motivated by -
the concept of assets within Dagster. A key enabler to fully manage the lifecycle of assets is to
handle how assets are persisted and queried. This is where IO managers come in. IO managers handle
saving an loading data assets from the persistence layer. The mlfusion platform introduces two main
artifact handlers - [datasets](#datasets) and [model artifacts](#model-artifacts).

### Configuration

A lot of configuration is shared between the different resources integrated with Dagster. To make
configuration easier and more consistent between scenarios we introduce a global configuration resource.
A simple configuration running all services locally might look like.

```py
from dagster_fusion import (
    flight_fusion_io_manager,
    flight_fusion_resource,
    mlflow_tracking,
    mlfusion_configuration,
    model_artifact_io_manager,
)

RESOURCES_LOCAL = {
    "mlfusion_config": mlfusion_configuration.configured({"mlflow_tracking_uri": "http://localhost:5000"}),
    "fusion_client": flight_fusion_resource,
    "fusion_io": flight_fusion_io_manager,
    "model_artifact_io": model_artifact_io_manager,
    "mlflow": mlflow_tracking.configured({"experiment_name": "demo_experiment"}),
}
```

It is always encouraged to provide all relevant configuration explicitly via the "configured" APIs
from dagster - potentially making use of the `{"env": ENV_VARIABLE}` syntax to map config to available
environment variables. However the internal configuration will pick up on specific environment variables
by default.

- TODO describe environment variables

### Datasets

Datasets are the main source of data in most data-engineering / -science workflows. The core APIs
are designed to provide uniform access to data assets in all environments (e.g. locally or deployed)
or frameworks (e.g. pipelines or apps).

Lets define a simple asset the would just generate some random data to be used later in the pipeline.

```py
from dagster import OpExecutionContext, asset
import pyarrow as pa

@asset(
    name="dataset",
    namespace=["demo", "model_training"],
    description="Randomly generated dataset for regression problem.",
    io_manager_key="fusion_io",
)
def dataset(context: OpExecutionContext) -> pa.Table:
    ...
```

An asset is uniquely identified by the `namespace` and `name` parameters. These are somewhat
synonymous to `AssetKey`'s the core abstraction how we reference assets
throughout the platform. An equivalent asset key definition would look like

```py
# this import of the AssetKey will load the definition from Dagster package if available
# and use a local implementation in environments where dagster is not installed.
from flight_fusion import AssetKey

asset_key = AssetKey(["demo", "model_training", "dataset"])
```

When consuming an asset, we are not necessarily interested in loading the full dataset.
THe io managers natively support column sub-selections when loading data. This needs to
be configured via the `metadata` property of your asset definition.

```py hl_lines="1 9-14"
from dagster import AssetIn, OpExecutionContext, asset
import pyarrow as pa

@asset(
    name="training_data",
    namespace=["demo", "model_training"],
    description="Data used for model training",
    io_manager_key="fusion_io",
    ins={
        "dataset": AssetIn(
            asset_key=AssetKey(["demo", "model_training", "dataset"]),
            metadata={"columns": ["feature_1", "feature_2", "target"]}
        )
    },
)
def training_data(context: OpExecutionContext, dataset: pa.Table) -> pa.Table:
    ...
```

#### Controlling the return type of data

At the core all internal data movement and processing is dome in apache arrow format.
This however readily converts to other data representations and popular dataframe
libraries.

The dataset IO manager will pick up on any known type annotation and serve data in that format.
THe same way it will inspect the type of the returned data and perform internal conversions.
Current supported data formats are:

- `pandas.DataFrame`
- `polars.DataFrame`
- `pyarrow.Table`

```py hl_lines="3 17"
from dagster import AssetIn, OpExecutionContext, asset
import pyarrow as pa
import polars as pl

@asset(
    name="training_data",
    namespace=["demo", "model_training"],
    description="Data used for model training",
    io_manager_key="fusion_io",
    ins={
        "dataset": AssetIn(
            asset_key=AssetKey(["demo", "model_training", "dataset"]),
            metadata={"columns": ["feature_1", "feature_2", "target"]}
        )
    },
)
def training_data(context: OpExecutionContext, dataset: pl.DataFrame) -> pa.Table:
    ...
```

### Model artifacts

- autolog -> still use io manager to assign asset tags
  - call log_model method
- pure version -> optionally yield artifacts ...

## Versioning

- data via delta
- models via runs / experiments

## CLI

`fusion env init`

<!-- prettier-ignore -->
: initialize boilerplate files in current repository or system wide for logged in user.

`fusion start flight`

<!-- prettier-ignore -->
: start a new instance of the flight-fusion service. This service provides file management APIs
  relating to managing data assets (flight-api) as well a model artifacts (artifact-store). The model
  artifact APIs are not meant to be used directly, but rather via the mlflow abstractions.

`fusion start dagit`

<!-- prettier-ignore -->
: start an instance of the Dagit UI for local development.

`fusion start mlflow`

<!-- prettier-ignore -->
: start an instance of the MlFlow UI for local development.

`fusion start mlserver`

<!-- prettier-ignore -->
: start an instance of the mlserver locally serving models for inference.
