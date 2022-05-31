# Generated by the protocol buffer compiler.  DO NOT EDIT!
# sources: flight_fusion/ipc/v1alpha1/common.proto, flight_fusion/ipc/v1alpha1/delta.proto, flight_fusion/ipc/v1alpha1/flight.proto, flight_fusion/ipc/v1alpha1/message.proto, flight_fusion/ipc/v1alpha1/signals.proto
# plugin: python-betterproto
from dataclasses import dataclass
from typing import Dict, List, Optional

import betterproto
from betterproto.grpc.grpclib_server import ServiceBase


class FileFormat(betterproto.Enum):
    """File format for a file stroed on disk"""

    # Undefined file format
    FILE_FORMAT_UNSPECIFIED = 0
    # Stored in parquet
    FILE_FORMAT_PARQUET = 1
    # Avro
    FILE_FORMAT_AVRO = 2
    # Csv
    FILE_FORMAT_CSV = 3


class SaveMode(betterproto.Enum):
    SAVE_MODE_UNSPECIFIED = 0
    SAVE_MODE_APPEND = 1
    SAVE_MODE_OVERWRITE = 2
    SAVE_MODE_ERROR_IF_EXISTS = 3


class SignalType(betterproto.Enum):
    SIGNAL_TYPE_UNSPECIFIED = 0
    SIGNAL_TYPE_OBSERVATION = 1
    SIGNAL_TYPE_CONSTANT = 2
    SIGNAL_TYPE_EXPRESSION = 3
    SIGNAL_TYPE_MODEL = 4


class ActionStatus(betterproto.Enum):
    ACTION_STATUS_UNSPECIFIED = 0
    ACTION_STATUS_SUCCESS = 1
    ACTION_STATUS_FAILURE = 2


@dataclass(eq=False, repr=False)
class FileReference(betterproto.Message):
    path: str = betterproto.string_field(1)
    format: "FileFormat" = betterproto.enum_field(2)


@dataclass(eq=False, repr=False)
class TableReference(betterproto.Message):
    file: "FileReference" = betterproto.message_field(3, group="table")


@dataclass(eq=False, repr=False)
class EntityUri(betterproto.Message):
    uri: str = betterproto.string_field(1)


@dataclass(eq=False, repr=False)
class EntityId(betterproto.Message):
    id: str = betterproto.string_field(1)


@dataclass(eq=False, repr=False)
class EntityPath(betterproto.Message):
    path: List[str] = betterproto.string_field(1)


@dataclass(eq=False, repr=False)
class AreaTableLocation(betterproto.Message):
    name: str = betterproto.string_field(1)
    areas: List[str] = betterproto.string_field(2)


@dataclass(eq=False, repr=False)
class AreaTableId(betterproto.Message):
    id: str = betterproto.string_field(1)


@dataclass(eq=False, repr=False)
class AreaTableUri(betterproto.Message):
    uri: str = betterproto.string_field(1)


@dataclass(eq=False, repr=False)
class AreaSourceReference(betterproto.Message):
    location: "AreaTableLocation" = betterproto.message_field(1, group="table")
    id: "AreaTableId" = betterproto.message_field(2, group="table")
    uri: "AreaTableUri" = betterproto.message_field(3, group="table")


@dataclass(eq=False, repr=False)
class SourceCollection(betterproto.Message):
    sources: List["AreaSourceReference"] = betterproto.message_field(1)


@dataclass(eq=False, repr=False)
class Tag(betterproto.Message):
    key: str = betterproto.string_field(1)
    value: str = betterproto.string_field(2)


@dataclass(eq=False, repr=False)
class DeltaCreateOperation(betterproto.Message):
    save_mode: "SaveMode" = betterproto.enum_field(1)
    metadata: str = betterproto.string_field(2)


@dataclass(eq=False, repr=False)
class DeltaWriteOperation(betterproto.Message):
    save_mode: "SaveMode" = betterproto.enum_field(1)
    partition_by: List[str] = betterproto.string_field(2)
    predicate: Optional[str] = betterproto.string_field(
        3, optional=True, group="_predicate"
    )


@dataclass(eq=False, repr=False)
class DeltaReadOperation(betterproto.Message):
    # version of delta table to load
    version: int = betterproto.int64_field(1)
    # load delta version from point in time
    timestamp: str = betterproto.string_field(2)
    predicate: str = betterproto.string_field(3)
    # column selection to load
    column_names: List[str] = betterproto.string_field(4)


@dataclass(eq=False, repr=False)
class DeltaOperationRequest(betterproto.Message):
    source: "AreaSourceReference" = betterproto.message_field(1)
    create: "DeltaCreateOperation" = betterproto.message_field(10, group="operation")
    write: "DeltaWriteOperation" = betterproto.message_field(11, group="operation")
    read: "DeltaReadOperation" = betterproto.message_field(12, group="operation")


@dataclass(eq=False, repr=False)
class DeltaOperationResponse(betterproto.Message):
    stats: str = betterproto.string_field(1)


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
    traits: List["SignalTrait"] = betterproto.message_field(10)


@dataclass(eq=False, repr=False)
class SignalTrait(betterproto.Message):
    sensitive: "SensitiveDataTrait" = betterproto.message_field(1, group="trait")
    time_series: "TimeSeriesTrait" = betterproto.message_field(2, group="trait")
    entity_reference: "EntityReferenceTrait" = betterproto.message_field(
        3, group="trait"
    )


@dataclass(eq=False, repr=False)
class SensitiveDataTrait(betterproto.Message):
    level: str = betterproto.string_field(1)


@dataclass(eq=False, repr=False)
class TimeSeriesTrait(betterproto.Message):
    level: str = betterproto.string_field(1)


@dataclass(eq=False, repr=False)
class EntityReferenceTrait(betterproto.Message):
    level: str = betterproto.string_field(1)


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
class CommandKqlOperation(betterproto.Message):
    """Describes a KQL query operation"""

    # name of the Kusto service to be queried
    service_name: str = betterproto.string_field(1)
    # The KQL syntax.
    query: str = betterproto.string_field(2)


@dataclass(eq=False, repr=False)
class CommandListSources(betterproto.Message):
    """List all sources defined under an area node"""

    # If true, all sources in child nodes are listed as well
    recursive: bool = betterproto.bool_field(1)


@dataclass(eq=False, repr=False)
class CommandReadDataset(betterproto.Message):
    """Read entire table from storage"""

    # source identifier
    source: "AreaSourceReference" = betterproto.message_field(1)
    # column selection to load
    column_names: List[str] = betterproto.string_field(2)


@dataclass(eq=False, repr=False)
class CommandDropSource(betterproto.Message):
    """Drop a source (e.g. a Table) from the service"""

    # source identifier
    source: "AreaSourceReference" = betterproto.message_field(1)


@dataclass(eq=False, repr=False)
class CommandSetMetadata(betterproto.Message):
    """Update metadata associated with source"""

    # source identifier
    source: "AreaSourceReference" = betterproto.message_field(1)
    # metadata to be written to source
    meta: "AreaSourceMetadata" = betterproto.message_field(2)


@dataclass(eq=False, repr=False)
class CommandWriteIntoDataset(betterproto.Message):
    """Request to write data to area storage"""

    # source identifier
    source: "AreaSourceReference" = betterproto.message_field(1)
    # denotes how to beahve for existing data - defaults to append
    save_mode: "SaveMode" = betterproto.enum_field(3)


@dataclass(eq=False, repr=False)
class CommandExecuteQuery(betterproto.Message):
    """Execute a query against a given context"""

    query: str = betterproto.string_field(1)
    source: "AreaSourceReference" = betterproto.message_field(10, group="context")
    frame: "SignalFrame" = betterproto.message_field(11, group="context")
    collection: "SourceCollection" = betterproto.message_field(12, group="context")


@dataclass(eq=False, repr=False)
class ResultActionStatus(betterproto.Message):
    """result when a source is dropped"""

    status: "ActionStatus" = betterproto.enum_field(1)


@dataclass(eq=False, repr=False)
class ResultDoPutUpdate(betterproto.Message):
    statistics: "BatchStatistics" = betterproto.message_field(1)


@dataclass(eq=False, repr=False)
class AreaSourceMetadata(betterproto.Message):
    """Metadata associated with an area source"""

    # globally unique idetifier for the source
    id: str = betterproto.string_field(1)
    # A human readable name for the source
    name: str = betterproto.string_field(2)
    # A short descrptive text that describes the content and purpose of the data
    # source
    description: str = betterproto.string_field(3)
    # tags associated with source
    tags: List["Tag"] = betterproto.message_field(9)
    # user defined properties
    properties: Dict[str, str] = betterproto.map_field(
        10, betterproto.TYPE_STRING, betterproto.TYPE_STRING
    )


@dataclass(eq=False, repr=False)
class AreaSourceDetails(betterproto.Message):
    """Detialed metadata and statistics about a source"""

    # globally unique idetifier for the source
    id: str = betterproto.string_field(1)
    # Metadata associated with the source
    metadata: "AreaSourceMetadata" = betterproto.message_field(2)


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
    # If true, any field that is defined is the actual value in the data provided
    # by the operator (it is not an estimate). Any or all other fields might
    # still be None, in which case no information is known. if false, any field
    # that is has a value may contain an inexact estimate and may not be the
    # actual value.
    is_exact: bool = betterproto.bool_field(4)


@dataclass(eq=False, repr=False)
class ColumnStatistics(betterproto.Message):
    """This table statistics are estimates about column properties"""

    # Number of null values on column
    null_count: int = betterproto.int64_field(1)
    # Maximum value of column
    max_value: str = betterproto.string_field(2)
    # Minimum value of column
    min_value: str = betterproto.string_field(3)
    # Number of distinct values
    distinct_count: int = betterproto.int64_field(4)


@dataclass(eq=False, repr=False)
class FlightDoGetRequest(betterproto.Message):
    """Requests submitted against the `do_get` endpoint"""

    # execute a KQL query against a registered Kusto cluster
    kql: "CommandKqlOperation" = betterproto.message_field(1, group="command")
    # Read data from a registered source
    read: "CommandReadDataset" = betterproto.message_field(2, group="command")
    # Execute a query against a pre-defined context
    query: "CommandExecuteQuery" = betterproto.message_field(3, group="command")
    # Perform a read operation against Delta table
    delta: "DeltaOperationRequest" = betterproto.message_field(4, group="command")


@dataclass(eq=False, repr=False)
class FlightDoPutRequest(betterproto.Message):
    """Requests submitted against the `do_put` endpoint"""

    # Write data into a registered source
    storage: "CommandWriteIntoDataset" = betterproto.message_field(2, group="command")
    delta: "DeltaOperationRequest" = betterproto.message_field(3, group="command")


@dataclass(eq=False, repr=False)
class FlightDoPutResponse(betterproto.Message):
    """Response recieved from `do_put` operations`"""

    # statistics for data written to source
    update: "ResultDoPutUpdate" = betterproto.message_field(1, group="payload")


@dataclass(eq=False, repr=False)
class FlightActionRequest(betterproto.Message):
    """Requests submitted against the `do_action` endpoint"""

    # command to remove a dataset from the area store
    drop: "CommandDropSource" = betterproto.message_field(2, group="action")
    # Set the metadata for a data source
    set_meta: "CommandSetMetadata" = betterproto.message_field(3, group="action")


@dataclass(eq=False, repr=False)
class FlightActionResponse(betterproto.Message):
    # Result when actions reports its execution status
    status: "ResultActionStatus" = betterproto.message_field(1, group="payload")
