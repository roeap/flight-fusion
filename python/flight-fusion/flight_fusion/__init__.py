from importlib.metadata import version

from .clients import (
    AreaClient,
    ClientOptions,
    DatasetClient,
    FusionServiceClient,
    ModelServiceClient,
    TableClient,
)

__version__ = version(__name__)

__all__ = (
    "AreaClient",
    "ClientOptions",
    "DatasetClient",
    "FusionServiceClient",
    "TableClient",
    "ModelServiceClient",
)
