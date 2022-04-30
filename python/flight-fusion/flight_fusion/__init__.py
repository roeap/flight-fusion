from importlib.metadata import version

from .asset_key import AssetKey
from .clients import (
    BaseDatasetClient,
    ClientOptions,
    DatasetClient,
    FusionServiceClient,
    ModelServiceClient,
)
from .ipc.v1alpha1 import SaveMode

__version__ = version(__name__)

__all__ = (
    "AssetKey",
    "ClientOptions",
    "BaseDatasetClient",
    "FusionServiceClient",
    "DatasetClient",
    "ModelServiceClient",
    "SaveMode",
)
