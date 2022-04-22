from importlib.metadata import version

from .clients import (
    AreaClient,
    ClientOptions,
    DatasetClient,
    FusionServiceClient,
    ModelServiceClient,
    TableClient,
)
from .ipc.v1alpha1 import SaveMode

__version__ = version(__name__)

__all__ = (
    "AreaClient",
    "ClientOptions",
    "DatasetClient",
    "FusionServiceClient",
    "TableClient",
    "ModelServiceClient",
    "SaveMode",
)
