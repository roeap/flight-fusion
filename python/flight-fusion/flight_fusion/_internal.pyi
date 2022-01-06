from typing import Callable, List

import pyarrow as pa

rust_core_version: Callable[[], str]

class FusionClient:
    """Client for interaction with flight fusion service"""

    def drop_table(self, table_name: str) -> bytes:
        """Drop a table registered in the FlightFusion service

        Args:
            table_name (str): Name of the table to be dropped

        Returns:
            bytes: Serialized return message
        """
        ...
    def register_memory_table(
        self, table_name: str, batches: List[pa.RecordBatch]
    ) -> bytes: ...
