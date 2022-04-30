from typing import Any, Callable, Sequence

from .lib import DataType, Field, Schema, Table, field

__all__ = ("Table", "Field", "Schema", "Field", "DataType", "field")

RecordBatch: Any
schema: Any
map_: Any
list_: Any
struct: Any
type_for_alias: Any
date32: Any
date64: Any
decimal128: Any
float16: Any
float32: Any
float64: Any

py_buffer: Callable[[bytes], Any]
NativeFile: Any
BufferReader: Any

def array(data: Sequence[Any], type: DataType | None = None) -> Any: ...
def _datatype_fn(name: str = "") -> DataType: ...

int8 = _datatype_fn
int16 = _datatype_fn
int32 = _datatype_fn
int64 = _datatype_fn
