# flake8: noqa 405
from . import _internal
from ._internal import KustoClient, rust_core_version
from .version import __version__

__doc__ = _internal.__doc__

__all__ = ("rust_core_version", "KustoClient")
