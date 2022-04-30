import pyarrow as pa
import pyarrow.flight as pa_flight
from betterproto import Message
from pydantic import BaseSettings

import flight_fusion.errors as errors


class ClientOptions(BaseSettings):
    host: str
    port: int
    timeout: int = 5

    class Config:
        env_prefix = "ff_"


class BaseClient:
    def __init__(
        self, client: pa_flight.FlightClient | None = None, options: ClientOptions | None = None
    ):
        if client is None and options is None:
            try:
                options = ClientOptions()  # type: ignore
            except Exception:
                raise errors.ConfigError("Either 'client' or 'options' must be defined.") from None

        self._client = client
        self._options = options

    @property
    def _flight(self) -> pa_flight.FlightClient:
        if self._client is None:
            if self._options is None:
                raise errors.ConfigError("Either 'client' or 'options' must be defined.")
            self._client = pa_flight.connect(f"grpc://{self._options.host}:{self._options.port}")
            self._client.wait_for_available(timeout=self._options.timeout)
        return self._client

    def _do_put(self, table: pa.Table, command: Message) -> bytes:
        descriptor = pa_flight.FlightDescriptor.for_command(command.SerializeToString())
        writer, reader = self._flight.do_put(descriptor, table.schema)
        writer.write_table(table)
        writer.done_writing()
        return reader.read()

    def _do_get(self, command: Message) -> pa.Table:
        ticket = pa_flight.Ticket(ticket=command.SerializeToString())
        reader = self._flight.do_get(ticket)
        return reader.read_all()
