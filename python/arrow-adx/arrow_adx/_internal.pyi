from __future__ import annotations

from typing import Callable

import pyarrow as pa

rust_core_version: Callable[[], str]

class KustoClient:
    def __init__(self, service_url: str) -> None:
        """Create a new instance of a KustoClient"""
        ...
    def execute(self, database: str, query: str) -> list[pa.RecordBatch]:
        """_summary_

        Args:
            database (str): Name of the database in scope that is the target of the query
            query (str): Text of the query to execute

        Returns:
            list[pa.RecordBatch]: Primary query results as list of RecordBatches.
        """
        ...
