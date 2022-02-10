from .clients import (
    AreaClient,
    ClientOptions,
    DatasetClient,
    FusionServiceClient,
    TableClient,
)

try:
    from importlib.metadata import version
except ImportError:
    from importlib_metadata import version

__version__ = version(__name__)

__all__ = (
    "AreaClient",
    "ClientOptions",
    "DatasetClient",
    "FusionServiceClient",
    "TableClient",
)
