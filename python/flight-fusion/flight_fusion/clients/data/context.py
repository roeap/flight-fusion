from __future__ import annotations

from typing import List

import pyarrow as pa

from flight_fusion.clients.service import FusionServiceClient
from flight_fusion.ipc.v1alpha1 import (
    AreaSourceReference,
    CommandExecuteQuery,
    FlightDoGetRequest,
    SourceCollection,
)


class ContextClient:
    def __init__(self, client: FusionServiceClient, sources: List[AreaSourceReference]) -> None:
        self._sources = sources
        self._client = client

    def load(self) -> List[pa.Table]:
        raise NotImplementedError

    def query(self, query: str) -> pa.Table:
        command = FlightDoGetRequest(
            query=CommandExecuteQuery(
                query=query, collection=SourceCollection(sources=self._sources)
            )
        )
        return self._client._do_get(command)
