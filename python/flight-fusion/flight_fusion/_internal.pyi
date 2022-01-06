from typing import Callable, List

import pyarrow as pa

rust_core_version: Callable[[], str]

class FusionClient:
    def drop_table(self, table_name: str) -> bytes: ...
    def register_memory_table(
        self, table_name: str, batches: List[pa.RecordBatch]
    ) -> bytes: ...
