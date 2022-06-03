from .io import flight_fusion_io_manager, flight_fusion_loader, mlfusion_io_manager
from .resources import flight_fusion_resource

try:
    from importlib.metadata import version
except ImportError:
    from importlib_metadata import version

__version__ = version(__name__)

__all__ = ("flight_fusion_io_manager", "flight_fusion_loader", "flight_fusion_resource", "mlfusion_io_manager")
