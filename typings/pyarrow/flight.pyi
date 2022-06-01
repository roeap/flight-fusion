from enum import Enum
from typing import Iterator

import pyarrow.flight as pa_flight

from .lib import RecordBatch, Schema, Table

class Location: ...
class ClientMiddlewareFactory: ...
class FlightCallOptions: ...
class ClientAuthHandler: ...

class DescriptorType(Enum):
    UNKNOWN = 0
    PATH = 1
    CMD = 2

class FlightDescriptor:
    @classmethod
    def for_command(cls, cmd: bytes) -> FlightDescriptor: ...
    @property
    def type(self) -> DescriptorType:
        """Describes what type of descriptor is defined."""
        ...
    @property
    def cmd(self) -> bytes:
        """que value used to express a command. Should only be defined when type = CMD."""
        ...
    @property
    def path(self) -> list[str]:
        """List of strings identifying a particular dataset. Should only be defined when type = PATH."""
        ...

class ActionType: ...
class Action: ...

class FlightStreamChunk:
    """A RecordBatch with application metadata on the side."""

    @property
    def data(self) -> bytes: ...
    @property
    def app_metadata(self) -> bytes: ...

class FlightStreamReader:
    def cancel(self) -> None: ...
    def read_all(self) -> Table:
        """Read the entire contents of the stream as a Table."""
        ...

class RecordBatchReader: ...

class MetadataRecordBatchWriter:
    """A RecordBatchWriter that also allows writing application metadata.
    This class is a context manager; on exit, close() will be called.
    """

    def begin(self, schema: Schema, options=None):
        """Prepare to write data to this stream with the given schema."""
        ...
    def write_batch(self, batch: RecordBatch) -> None:
        """
        Write RecordBatch to stream.
        Parameters
        ----------
        batch : RecordBatch
        """
        ...
    def write_table(self, table: Table, max_chunksize: int | None = None, **kwargs) -> None:
        """Write Table to stream in (contiguous) RecordBatch objects.

        Parameters
        ----------
        table : Table
        max_chunksize : int, default None
            Maximum size for RecordBatch chunks. Individual chunks may be
            smaller depending on the chunk layout of individual columns.
        """
        ...

class FlightMetadataReader:
    """A reader for Flight metadata messages sent during a DoPut."""

    def read(self) -> bytes:
        """Read the next metadata message."""
        ...
    def write_metadata(self, buf: bytes):
        """Write Flight metadata by itself."""
        ...

class FlightStreamWriter(MetadataRecordBatchWriter):
    """A writer that also allows closing the write side of a stream."""

    def done_writing(self) -> None:
        """Indicate that the client is done writing, but not done reading."""
        ...

class Ticket:
    """An opaque identifier that the service can use to retrieve a particular  portion of a stream.

    Tickets are meant to be single use. It is an error/application-defined behavior to reuse a ticket.
    """

    def __init__(self, ticket: bytes) -> None: ...
    @property
    def ticket(self) -> bytes: ...

class MetadataRecordBatchReader:
    def __iter__(self) -> Iterator[FlightStreamChunk]: ...
    @property
    def schema(self):
        """Get the schema for this reader."""
        ...
    def read_chunk(self) -> FlightStreamChunk:
        """Read the next RecordBatch along with any metadata.

        Returns:
            data: The next RecordBatch in the stream.
            app_metadata : Buffer or None
            Application-specific metadata for the batch as defined by
            Flight.

        Raises:
            StopIteration when the stream is finished
        """
        ...
    def to_reader(self) -> RecordBatchReader:
        """Convert this reader into a regular RecordBatchReader.

        This may fail if the schema cannot be read from the remote end.

        Returns:
            reader: RecordBatchReader
        """
        ...

class Result:
    @property
    def body(self) -> bytes: ...

class FlightClient:
    """A client to a Flight service.

    Connect to a Flight service on the given host and port.

    Parameters
    ----------
    location : str, tuple or Location
        Location to connect to. Either a gRPC URI like `grpc://localhost:port`,
        a tuple of (host, port) pair, or a Location instance.
    tls_root_certs : bytes or None
        PEM-encoded
    cert_chain: bytes or None
        Client certificate if using mutual TLS
    private_key: bytes or None
        Client private key for cert_chain is using mutual TLS
    override_hostname : str or None
        Override the hostname checked by TLS. Insecure, use with caution.
    middleware : list optional, default None
        A list of ClientMiddlewareFactory instances.
    write_size_limit_bytes : int optional, default None
        A soft limit on the size of a data payload sent to the
        server. Enabled if positive. If enabled, writing a record
        batch that (when serialized) exceeds this limit will raise an
        exception; the client can retry the write with a smaller
        batch.
    disable_server_verification : boolean optional, default False
        A flag that indicates that, if the client is connecting
        with TLS, that it skips server verification. If this is
        enabled, all other TLS settings are overridden.
    generic_options : list optional, default None
        A list of generic (string, int or string) option tuples passed
        to the underlying transport. Effect is implementation
        dependent.
    """

    def __init__(
        self,
        location: str | tuple | Location,
        *,
        tls_root_certs: bytes | None = None,
        cert_chain: bytes | None = None,
        private_key: bytes | None = None,
        override_hostname: str | None = None,
        middleware: list[ClientMiddlewareFactory] | None = None,
        write_size_limit_bytes: int | None = None,
        disable_server_verification: bool | None = None,
        generic_options=None,
    ): ...
    def authenticate(self, auth_handler: ClientAuthHandler, options: FlightCallOptions | None = None):
        """Authenticate to the server.

        Args:
            auth_handler : The authentication mechanism to use.
            options: Options for this call.
        """
        ...
    def authenticate_basic_token(
        self, username: str, password: str, options: FlightCallOptions | None = None
    ) -> tuple[str, str]:
        """Authenticate to the server with HTTP basic authentication.

        Args:
            username: Username to authenticate with
            password: Password to authenticate with
            options: Options for this call. Defaults to None.

        Returns:
            options: A tuple representing the FlightCallOptions authorization header entry of a bearer token.
        """
        ...
    def wait_for_available(self, timeout: int = 5):
        """Block until the server can be contacted.

        Args:
            timeout: The maximum seconds to wait. Defaults to 5
        """
        ...
    def list_actions(self, options: FlightCallOptions | None = None) -> list[ActionType]:
        """List the actions available on a service.

        Args:
            options: Options for this call. Defaults to None.

        Returns:
            actions: List of available actions.
        """
        ...
    def do_action(
        self, action: str | bytes | tuple | Action, options: FlightCallOptions | None = None
    ) -> Iterator[Result]:
        """Execute an action on a service.

        Args:
            action: Can be action type name (no body), type and body, or any Action  object
            options: RPC options

        Returns:
            results : iterator of Result values
        """
    def list_flights(self, criteria: bytes | None = None, options: FlightCallOptions | None = None):
        """List the flights available on a service.

        Args:
            criteria: A service specific expression that can be used to return a limited set
                of available Arrow Flight streams.
            options: RPC options

        Returns:
            flights:
        """
        ...
    def get_flight_info(self, descriptor: FlightDescriptor, options: FlightCallOptions | None = None):
        """Request information about an available flight."""
        ...
    def get_schema(self, descriptor: FlightDescriptor, options: FlightCallOptions | None = None):
        """Request schema for an available flight."""
        ...
    def do_get(self, ticket: Ticket, options: FlightCallOptions | None = None) -> FlightStreamReader:
        """Request the data for a flight.

        Returns:
            reader : FlightStreamReader
        """
        ...
    def do_put(
        self, descriptor: FlightDescriptor, schema: Schema, options: FlightCallOptions | None = None
    ) -> tuple[FlightStreamWriter, FlightMetadataReader]:
        """Upload data to a flight."""
        ...

def connect(
    location: str | tuple | Location,
    tls_root_certs: bytes | None = None,
    cert_chain: bytes | None = None,
    private_key: bytes | None = None,
    override_hostname: str | None = None,
    disable_server_verification: bool | None = None,
) -> FlightClient:
    """_summary_

    Args:
        location (str | tuple | Location): _description_
        tls_root_certs (bytes | None, optional): _description_. Defaults to None.
        cert_chain (bytes | None, optional): _description_. Defaults to None.
        private_key (bytes | None, optional): _description_. Defaults to None.
        override_hostname (str | None, optional): _description_. Defaults to None.
        disable_server_verification (bool | None, optional): _description_. Defaults to None.
    """
    ...
