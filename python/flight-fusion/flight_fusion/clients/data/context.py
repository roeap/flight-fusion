from __future__ import annotations

import pyarrow as pa
import pyarrow.flight as pa_flight

from flight_fusion.ipc.v1alpha1 import (
    AreaSourceReference,
    CommandExecuteQuery,
    FlightDoGetRequest,
    SourceCollection,
)

from .._base import BaseClient, ClientOptions


class ContextClient(BaseClient):
    def __init__(
        self,
        sources: list[AreaSourceReference],
        client: pa_flight.FlightClient | None = None,
        options: ClientOptions | None = None,
    ) -> None:
        super().__init__(client=client, options=options)
        self._sources = sources

    def load(self) -> list[pa.Table]:
        raise NotImplementedError

    def query(self, query: str) -> pa.Table:
        command = FlightDoGetRequest(
            query=CommandExecuteQuery(query=query, collection=SourceCollection(sources=self._sources))
        )
        return self._do_get(command)
