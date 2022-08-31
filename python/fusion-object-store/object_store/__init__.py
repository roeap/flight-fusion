from __future__ import annotations

from io import BytesIO
from pathlib import Path as PythonPath

from ._internal import ListResult as ListResult
from ._internal import ObjectMeta as ObjectMeta
from ._internal import ObjectStore as _RawObjectStore
from ._internal import Path as Path

try:
    import importlib.metadata as importlib_metadata
except ImportError:
    import importlib_metadata

__version__ = importlib_metadata.version(__name__)

PathLike = str | list[str] | Path
BytesLike = bytes | BytesIO

DELIMITER = "/"


def _as_path(raw: PathLike) -> Path:
    if isinstance(raw, str):
        return Path(raw)
    if isinstance(raw, list):
        return Path(DELIMITER.join(raw))
    if isinstance(raw, Path):
        return raw
    raise ValueError(f"Cannot convert type '{type(raw)}' to type Path.")


def _as_bytes(raw: BytesLike) -> bytes:
    if isinstance(raw, bytes):
        return raw
    if isinstance(raw, BytesIO):
        return raw.read()
    raise ValueError(f"Cannot convert type '{type(raw)}' to type bytes.")


class ObjectStore:
    def __init__(self, root: str | PythonPath) -> None:
        if isinstance(root, PythonPath):
            root = str(root.absolute())
        self._store = _RawObjectStore(root)

    def get(self, location: PathLike) -> bytes:
        return self._store.get(_as_path(location))

    def put(self, location: PathLike, bytes: BytesLike) -> None:
        return self._store.put(_as_path(location), _as_bytes(bytes))

    def list(self, prefix: PathLike | None = None) -> list[ObjectMeta]:
        prefix_ = _as_path(prefix) if prefix else None
        return self._store.list(prefix_)

    def list_with_delimiter(self, prefix: PathLike | None = None) -> ListResult:
        prefix_ = _as_path(prefix) if prefix else None
        return self._store.list_with_delimiter(prefix_)

    def head(self, location: PathLike) -> ObjectMeta:
        return self._store.head(_as_path(location))
