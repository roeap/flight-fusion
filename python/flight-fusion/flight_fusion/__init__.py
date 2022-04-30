from importlib.metadata import version

from .clients import (
    AreaClient,
    ClientOptions,
    BaseDatasetClient,
    FusionServiceClient,
    ModelServiceClient,
    DatasetClient,
)
from .ipc.v1alpha1 import SaveMode

__version__ = version(__name__)

__all__ = (
    "AreaClient",
    "ClientOptions",
    "BaseDatasetClient",
    "FusionServiceClient",
    "DatasetClient",
    "ModelServiceClient",
    "SaveMode",
)
