from __future__ import annotations

from typing import List

import pyarrow as pa

from flight_fusion.clients.service import FlightFusionClient
from flight_fusion.ipc.v1alpha1 import (
    AreaSourceReference,
    CommandExecuteQuery,
    SourceCollection,
)


class ContextClient:
    def __init__(self, client: FlightFusionClient, sources: List[AreaSourceReference]) -> None:
        self._sources = sources
        self._client = client

    def load(self) -> List[pa.Table]:
        raise NotImplementedError

    def query(self, query: str) -> pa.Table:
        command = CommandExecuteQuery(
            query=query, collection=SourceCollection(sources=self._sources)
        )
        batches = self._client.fusion.execute_query(command=command.SerializeToString())
        return pa.Table.from_batches(batches)
