from __future__ import annotations

from typing import TYPE_CHECKING, Iterable, List, Tuple

import pyarrow as pa
from betterproto import Message
from pyarrow.flight import FlightDescriptor, Ticket, connect
from pydantic import BaseSettings

from flight_fusion.ipc.v1alpha1 import AreaSourceReference, AreaTableLocation

if TYPE_CHECKING:
    from flight_fusion.clients import AreaClient, BaseDatasetClient, ContextClient


class FlightActions:
    REGISTER_TABLE = b"register-table"
    REGISTER_DELTA_TABLE = b"register-delta-table"


class ClientOptions(BaseSettings):
    host: str
    port: int

    class Config:
        env_prefix = "ff_"


class FusionServiceClient:
    def __init__(
        self,
        options: ClientOptions,
    ) -> None:
        self._client = connect(f"grpc://{options.host}:{options.port}")
        self._client.wait_for_available()

    def _do_put(self, table: pa.Table, command: Message) -> bytes:
        descriptor = FlightDescriptor.for_command(command.SerializeToString())
        writer, reader = self._client.do_put(descriptor, table.schema)
        writer.write_table(table)
        writer.done_writing()
        response = reader.read()
        return response

    def _do_get(self, command: Message) -> pa.Table:
        ticket = Ticket(ticket=command.SerializeToString())
        reader = self._client.do_get(ticket)
        table = reader.read_all()
        return table

    def get_source_reference(self, name: str, areas: List[str]) -> AreaSourceReference:
        return AreaSourceReference(location=AreaTableLocation(name=name, areas=areas))

    def get_area_client(self, areas: List[str]) -> AreaClient:
        from flight_fusion.clients.area import AreaClient

        return AreaClient(self, areas)

    def get_context(self, refs: Iterable[Tuple[str, List[str]]]) -> ContextClient:
        from flight_fusion.clients.data.context import ContextClient

        return ContextClient(
            client=self,
            sources=[self.get_source_reference(name, areas) for name, areas in refs],
        )

    def get_dataset_client(self, name: str, areas: List[str]) -> BaseDatasetClient:
        from flight_fusion.clients import DatasetClient

        return DatasetClient(
            client=self.get_area_client(areas),
            reference=AreaSourceReference(location=AreaTableLocation(name=name, areas=areas)),
        )

    def get_versioned_dataset_client(self, name: str, areas: List[str]) -> BaseDatasetClient:
        from flight_fusion.clients import VersionedDatasetClient

        return VersionedDatasetClient(
            client=self.get_area_client(areas),
            reference=AreaSourceReference(location=AreaTableLocation(name=name, areas=areas)),
        )
