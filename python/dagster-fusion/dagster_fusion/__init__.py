from .io import (
    flight_fusion_io_manager,
    flight_fusion_loader,
    model_artifact_io_manager,
)
from .resources import flight_fusion_resource, mlflow_tracking, mlfusion_configuration

try:
    from importlib.metadata import version
except ImportError:
    from importlib_metadata import version

__version__ = version(__name__)

__all__ = (
    "flight_fusion_io_manager",
    "flight_fusion_loader",
    "flight_fusion_resource",
    "mlflow_tracking",
    "mlfusion_configuration",
    "model_artifact_io_manager",
)
