# Generated by the protocol buffer compiler.  DO NOT EDIT!
# sources: common.proto, message.proto, signals.proto
# plugin: python-betterproto
from dataclasses import dataclass
from typing import List

import betterproto
from betterproto.grpc.grpclib_server import ServiceBase


class FileFormat(betterproto.Enum):
    FILE_FORMAT_UNSPECIFIED = 0
    FILE_FORMAT_PARQUET = 1
    FILE_FORMAT_AVRO = 2
    FILE_FORMAT_CSV = 3


class DatasetFormat(betterproto.Enum):
    DATASET_FORMAT_FILE = 0
    DATASET_FORMAT_DATASET = 1
    DATASET_FORMAT_DELTA = 2


class SaveMode(betterproto.Enum):
    SAVE_MODE_UNSPECIFIED = 0
    SAVE_MODE_APPEND = 1
    SAVE_MODE_OVERWRITE = 2
    SAVE_MODE_ERROR_IF_EXISTS = 3


class StorageType(betterproto.Enum):
    STORAGE_TYPE_UNSPECIFIED = 0
    STORAGE_TYPE_LOCAL = 1
    STORAGE_TYPE_HDFS = 2
    STORAGE_TYPE_AZURE_ADLS_V2 = 3
    STORAGE_TYPE_AZURE_BLOB = 4
    STORAGE_TYPE_S3 = 5


class SignalType(betterproto.Enum):
    SIGNAL_TYPE_UNSPECIFIED = 0
    SIGNAL_TYPE_OBSERVATION = 1
    SIGNAL_TYPE_CONSTANT = 2
    SIGNAL_TYPE_EXPRESSION = 3
    SIGNAL_TYPE_MODEL = 4


@dataclass(eq=False, repr=False)
class DeltaReference(betterproto.Message):
    location: str = betterproto.string_field(1)


@dataclass(eq=False, repr=False)
class ListingReference(betterproto.Message):
    path: str = betterproto.string_field(1)
    format: "FileFormat" = betterproto.enum_field(2)
    partition_columns: List[str] = betterproto.string_field(3)


@dataclass(eq=False, repr=False)
class FileReference(betterproto.Message):
    path: str = betterproto.string_field(1)
    format: "FileFormat" = betterproto.enum_field(2)


@dataclass(eq=False, repr=False)
class TableReference(betterproto.Message):
    delta: "DeltaReference" = betterproto.message_field(1, group="table")
    listing: "ListingReference" = betterproto.message_field(2, group="table")
    file: "FileReference" = betterproto.message_field(3, group="table")


@dataclass(eq=False, repr=False)
class ExpressionReference(betterproto.Message):
    uid: str = betterproto.string_field(1)
    expression: str = betterproto.string_field(2)


@dataclass(eq=False, repr=False)
class ModelReference(betterproto.Message):
    uri: str = betterproto.string_field(1)


@dataclass(eq=False, repr=False)
class Signal(betterproto.Message):
    uid: str = betterproto.string_field(1)
    name: str = betterproto.string_field(2)
    description: str = betterproto.string_field(3)


@dataclass(eq=False, repr=False)
class SignalProvider(betterproto.Message):
    uid: str = betterproto.string_field(1)
    name: str = betterproto.string_field(2)
    description: str = betterproto.string_field(3)
    signals: List["Signal"] = betterproto.message_field(4)
    inputs: List["Signal"] = betterproto.message_field(5)
    table: "TableReference" = betterproto.message_field(100, group="source")
    expression: "ExpressionReference" = betterproto.message_field(101, group="source")
    model: "ModelReference" = betterproto.message_field(102, group="source")


@dataclass(eq=False, repr=False)
class SignalFrame(betterproto.Message):
    """
    A SignalFrame defines the context for a specialized query across multiple
    data sources
    """

    uid: str = betterproto.string_field(1)
    name: str = betterproto.string_field(2)
    description: str = betterproto.string_field(3)
    providers: List["SignalProvider"] = betterproto.message_field(4)


@dataclass(eq=False, repr=False)
class RegisterDatasetRequest(betterproto.Message):
    format: "DatasetFormat" = betterproto.enum_field(1)
    path: str = betterproto.string_field(2)
    name: str = betterproto.string_field(3)


@dataclass(eq=False, repr=False)
class RegisterDatasetResponse(betterproto.Message):
    message: str = betterproto.string_field(1)


@dataclass(eq=False, repr=False)
class DropDatasetRequest(betterproto.Message):
    name: str = betterproto.string_field(1)


@dataclass(eq=False, repr=False)
class DropDatasetResponse(betterproto.Message):
    name: str = betterproto.string_field(1)


@dataclass(eq=False, repr=False)
class FlightActionRequest(betterproto.Message):
    """Requests submitted against the `do_action` endpoint"""

    register: "RegisterDatasetRequest" = betterproto.message_field(1, group="action")
    drop: "DropDatasetRequest" = betterproto.message_field(2, group="action")


@dataclass(eq=False, repr=False)
class CommandSqlOperation(betterproto.Message):
    """Describes an SQL query operation"""

    # The SQL syntax.
    query: str = betterproto.string_field(1)


@dataclass(eq=False, repr=False)
class CommandKqlOperation(betterproto.Message):
    """Describes a KQL query operation"""

    # name of the Kusto service to be queried
    service_name: str = betterproto.string_field(1)
    # The KQL syntax.
    query: str = betterproto.string_field(2)


@dataclass(eq=False, repr=False)
class SignalFrameOperation(betterproto.Message):
    """Describes a signal frame operation"""

    frame: "SignalFrame" = betterproto.message_field(1)


@dataclass(eq=False, repr=False)
class FlightDoGetRequest(betterproto.Message):
    """Requests submitted against the `do_get` endpoint"""

    sql: "CommandSqlOperation" = betterproto.message_field(1, group="operation")
    kql: "CommandKqlOperation" = betterproto.message_field(2, group="operation")
    frame: "SignalFrameOperation" = betterproto.message_field(3, group="operation")


@dataclass(eq=False, repr=False)
class DeltaCreateOperation(betterproto.Message):
    save_mode: "SaveMode" = betterproto.enum_field(1)


@dataclass(eq=False, repr=False)
class DeltaWriteOperation(betterproto.Message):
    save_mode: "SaveMode" = betterproto.enum_field(1)
    partition_columns: List[str] = betterproto.string_field(2)
    predicate: str = betterproto.string_field(3)


@dataclass(eq=False, repr=False)
class DeltaOperationRequest(betterproto.Message):
    table: "DeltaReference" = betterproto.message_field(1)
    create: "DeltaCreateOperation" = betterproto.message_field(10, group="operation")
    write: "DeltaWriteOperation" = betterproto.message_field(11, group="operation")


@dataclass(eq=False, repr=False)
class DeltaOperationResponse(betterproto.Message):
    stats: str = betterproto.string_field(1)


@dataclass(eq=False, repr=False)
class PutMemoryTableRequest(betterproto.Message):
    name: str = betterproto.string_field(1)


@dataclass(eq=False, repr=False)
class PutMemoryTableResponse(betterproto.Message):
    name: str = betterproto.string_field(1)


@dataclass(eq=False, repr=False)
class PutRemoteTableRequest(betterproto.Message):
    name: str = betterproto.string_field(1)
    path: str = betterproto.string_field(2)


@dataclass(eq=False, repr=False)
class PutRemoteTableResponse(betterproto.Message):
    name: str = betterproto.string_field(1)


@dataclass(eq=False, repr=False)
class FlightDoPutRequest(betterproto.Message):
    """Requests submitted against the `do_put` endpoint"""

    memory: "PutMemoryTableRequest" = betterproto.message_field(1, group="operation")
    remote: "PutRemoteTableRequest" = betterproto.message_field(2, group="operation")
    delta: "DeltaOperationRequest" = betterproto.message_field(3, group="operation")


@dataclass(eq=False, repr=False)
class DoPutUpdateResult(betterproto.Message):
    """
    Returned from the RPC call DoPut when a CommandStatementUpdate
    CommandPreparedStatementUpdate was in the request, containing results from
    the update.
    """

    statistics: "BatchStatistics" = betterproto.message_field(1)


@dataclass(eq=False, repr=False)
class BatchStatistics(betterproto.Message):
    """
    Statistics for a physical plan node Fields are optional and can be inexact
    because the sources sometimes provide approximate estimates for performance
    reasons and the transformations output are not always predictable.
    """

    # The number of table rows
    record_count: int = betterproto.int64_field(1)
    # total byte of the table rows
    total_byte_size: int = betterproto.int64_field(2)
    # Statistics on a column level
    column_statistics: List["ColumnStatistics"] = betterproto.message_field(3)
    # If true, any field that is `Some(..)` is the actual value in the data
    # provided by the operator (it is not an estimate). Any or all other fields
    # might still be None, in which case no information is known. if false, any
    # field that is `Some(..)` may contain an inexact estimate and may not be the
    # actual value.
    is_exact: bool = betterproto.bool_field(4)


@dataclass(eq=False, repr=False)
class ColumnStatistics(betterproto.Message):
    """This table statistics are estimates about column"""

    # Number of null values on column
    null_count: int = betterproto.int64_field(1)
    # Maximum value of column
    max_value: str = betterproto.string_field(2)
    # Minimum value of column
    min_value: str = betterproto.string_field(3)
    # Number of distinct values
    distinct_count: int = betterproto.int64_field(4)
