"""
@generated by mypy-protobuf.  Do not edit manually!
isort:skip_file
"""
import builtins
import flight_fusion.proto.common_pb2 as common_pb2
import google.protobuf.descriptor
import google.protobuf.internal.containers
import google.protobuf.message
import typing
import typing_extensions

DESCRIPTOR: google.protobuf.descriptor.FileDescriptor = ...

class SqlTicket(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor = ...
    QUERY_FIELD_NUMBER: builtins.int
    query: typing.Text = ...
    def __init__(
        self,
        *,
        query: typing.Text = ...,
    ) -> None: ...
    def ClearField(
        self, field_name: typing_extensions.Literal["query", b"query"]
    ) -> None: ...

global___SqlTicket = SqlTicket

class KqlTicket(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor = ...
    QUERY_FIELD_NUMBER: builtins.int
    query: typing.Text = ...
    def __init__(
        self,
        *,
        query: typing.Text = ...,
    ) -> None: ...
    def ClearField(
        self, field_name: typing_extensions.Literal["query", b"query"]
    ) -> None: ...

global___KqlTicket = KqlTicket

class DeltaCreateOperation(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor = ...
    SAVE_MODE_FIELD_NUMBER: builtins.int
    save_mode: common_pb2.SaveMode.V = ...
    def __init__(
        self,
        *,
        save_mode: common_pb2.SaveMode.V = ...,
    ) -> None: ...
    def ClearField(
        self, field_name: typing_extensions.Literal["save_mode", b"save_mode"]
    ) -> None: ...

global___DeltaCreateOperation = DeltaCreateOperation

class DeltaWriteOperation(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor = ...
    SAVE_MODE_FIELD_NUMBER: builtins.int
    PARTITION_COLUMNS_FIELD_NUMBER: builtins.int
    PREDICATE_FIELD_NUMBER: builtins.int
    save_mode: common_pb2.SaveMode.V = ...
    @property
    def partition_columns(
        self,
    ) -> google.protobuf.internal.containers.RepeatedScalarFieldContainer[
        typing.Text
    ]: ...
    predicate: typing.Text = ...
    def __init__(
        self,
        *,
        save_mode: common_pb2.SaveMode.V = ...,
        partition_columns: typing.Optional[typing.Iterable[typing.Text]] = ...,
        predicate: typing.Text = ...,
    ) -> None: ...
    def ClearField(
        self,
        field_name: typing_extensions.Literal[
            "partition_columns",
            b"partition_columns",
            "predicate",
            b"predicate",
            "save_mode",
            b"save_mode",
        ],
    ) -> None: ...

global___DeltaWriteOperation = DeltaWriteOperation

class DeltaOperationRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor = ...
    TABLE_FIELD_NUMBER: builtins.int
    CREATE_FIELD_NUMBER: builtins.int
    WRITE_FIELD_NUMBER: builtins.int
    @property
    def table(self) -> common_pb2.DeltaReference: ...
    @property
    def create(self) -> global___DeltaCreateOperation: ...
    @property
    def write(self) -> global___DeltaWriteOperation: ...
    def __init__(
        self,
        *,
        table: typing.Optional[common_pb2.DeltaReference] = ...,
        create: typing.Optional[global___DeltaCreateOperation] = ...,
        write: typing.Optional[global___DeltaWriteOperation] = ...,
    ) -> None: ...
    def HasField(
        self,
        field_name: typing_extensions.Literal[
            "create",
            b"create",
            "operation",
            b"operation",
            "table",
            b"table",
            "write",
            b"write",
        ],
    ) -> builtins.bool: ...
    def ClearField(
        self,
        field_name: typing_extensions.Literal[
            "create",
            b"create",
            "operation",
            b"operation",
            "table",
            b"table",
            "write",
            b"write",
        ],
    ) -> None: ...
    def WhichOneof(
        self, oneof_group: typing_extensions.Literal["operation", b"operation"]
    ) -> typing.Optional[typing_extensions.Literal["create", "write"]]: ...

global___DeltaOperationRequest = DeltaOperationRequest

class DeltaOperationResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor = ...
    STATS_FIELD_NUMBER: builtins.int
    stats: typing.Text = ...
    def __init__(
        self,
        *,
        stats: typing.Text = ...,
    ) -> None: ...
    def ClearField(
        self, field_name: typing_extensions.Literal["stats", b"stats"]
    ) -> None: ...

global___DeltaOperationResponse = DeltaOperationResponse

class PutMemoryTableRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor = ...
    NAME_FIELD_NUMBER: builtins.int
    name: typing.Text = ...
    def __init__(
        self,
        *,
        name: typing.Text = ...,
    ) -> None: ...
    def ClearField(
        self, field_name: typing_extensions.Literal["name", b"name"]
    ) -> None: ...

global___PutMemoryTableRequest = PutMemoryTableRequest

class PutMemoryTableResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor = ...
    NAME_FIELD_NUMBER: builtins.int
    name: typing.Text = ...
    def __init__(
        self,
        *,
        name: typing.Text = ...,
    ) -> None: ...
    def ClearField(
        self, field_name: typing_extensions.Literal["name", b"name"]
    ) -> None: ...

global___PutMemoryTableResponse = PutMemoryTableResponse

class PutRemoteTableRequest(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor = ...
    NAME_FIELD_NUMBER: builtins.int
    PATH_FIELD_NUMBER: builtins.int
    name: typing.Text = ...
    path: typing.Text = ...
    def __init__(
        self,
        *,
        name: typing.Text = ...,
        path: typing.Text = ...,
    ) -> None: ...
    def ClearField(
        self, field_name: typing_extensions.Literal["name", b"name", "path", b"path"]
    ) -> None: ...

global___PutRemoteTableRequest = PutRemoteTableRequest

class PutRemoteTableResponse(google.protobuf.message.Message):
    DESCRIPTOR: google.protobuf.descriptor.Descriptor = ...
    NAME_FIELD_NUMBER: builtins.int
    name: typing.Text = ...
    def __init__(
        self,
        *,
        name: typing.Text = ...,
    ) -> None: ...
    def ClearField(
        self, field_name: typing_extensions.Literal["name", b"name"]
    ) -> None: ...

global___PutRemoteTableResponse = PutRemoteTableResponse
