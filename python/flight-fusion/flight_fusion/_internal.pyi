"""Module containing compiled rust modules used in this library
"""
from typing import Callable, List

import pyarrow as pa

rust_core_version: Callable[[], str]

class FusionClient:
    """Client for interaction with flight fusion service"""

    def drop_table(self, table_ref: str) -> bytes:
        """Drop a table registered in the FlightFusion service

        Args:
            table_ref (str): Name of the table to be dropped

        Returns:
            bytes: Serialized return message
        """
        ...
    def write_into_table(
        self, table_ref: str, save_mode: int, batches: List[pa.RecordBatch]
    ) -> bytes:
        """Write data into a persisted table

        Args:
            table_ref (str): A name to reference the table in queries
            save_mode (int): Controls behavior for existing data in table
            batches (List[pa.RecordBatch]): Data to be written to the table

        Returns:
            bytes: Serialized return message
        """
        ...
    def read_table(self, table_ref: str) -> List[pa.RecordBatch]:
        """Read table data into record batches

        Args:
            table_ref (str): A name to reference the table in queries

        Returns:
            List[pa.RecordBatch]: Data contained in table
        """
        ...
    def put_memory_table(self, table_ref: str, batches: List[pa.RecordBatch]) -> bytes:
        """Register a table in memory to be available in queries

        This will not persist the table in any long lived storage. The table
        is only available for as long as the server is running or util the
        table is dropped from the context.

        Args:
            table_ref (str): A name to reference the table in queries
            batches (List[pa.RecordBatch]): Data to be written to the table

        Returns:
            bytes: Serialized return message
        """
        ...
