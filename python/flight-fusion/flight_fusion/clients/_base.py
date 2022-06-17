from __future__ import annotations

import pyarrow as pa
import pyarrow.flight as pa_flight
from betterproto import Message
from pydantic import BaseSettings

import flight_fusion.errors as errors
from flight_fusion.ipc.v1alpha1 import (
    AreaSourceMetadata,
    AreaSourceReference,
    AreaTableLocation,
)

from ..asset_key import AssetKey
from ..cache import timed_lru_cache
from ..errors import ResourceDoesNotExist


class ClientOptions(BaseSettings):
    host: str
    port: int
    timeout: int = 5

    class Config:
        env_prefix = "ff_"


@timed_lru_cache()
def _area_meta(flight_client, asset_key: AssetKey) -> AreaSourceMetadata:
    source = asset_key_to_source(asset_key=asset_key)
    request = pa_flight.FlightDescriptor.for_command(source.SerializeToString())
    try:
        flight_info = flight_client.get_flight_info(request)
    except Exception:
        raise ResourceDoesNotExist from None
    return AreaSourceMetadata.FromString(flight_info.descriptor.command)


class BaseClient:
    def __init__(self, client: pa_flight.FlightClient | None = None, options: ClientOptions | None = None):
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

    def _get_metadata(self, asset_key: AssetKey) -> AreaSourceMetadata:
        return _area_meta(flight_client=self._flight, asset_key=asset_key)


def asset_key_to_source(asset_key: AssetKey) -> AreaSourceReference:
    name = asset_key.path[-1]
    namespace = asset_key.path[:-1]
    return AreaSourceReference(location=AreaTableLocation(name=name, areas=namespace))
