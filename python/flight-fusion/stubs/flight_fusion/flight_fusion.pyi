from typing import Callable

from flight_fusion.proto.actions_pb2 import DropDatasetResponse

class FusionClient:
    def drop_table(self, table_name: str) -> DropDatasetResponse: ...

rust_core_version: Callable[[], str]
