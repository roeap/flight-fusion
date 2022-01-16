from __future__ import annotations

from typing import TYPE_CHECKING, List, Union

import pandas as pd
import pyarrow as pa
from pydantic import BaseSettings

from flight_fusion._internal import FusionClient as RawFusionClient
from flight_fusion.ipc.v1alpha1 import (
    AreaSourceReference,
    AreaTableLocation,
    ResultActionStatus,
    SaveMode,
)

if TYPE_CHECKING:
    from flight_fusion.clients import AreaClient, DatasetClient


class FlightActions:
    REGISTER_TABLE = b"register-table"
    REGISTER_DELTA_TABLE = b"register-delta-table"


class ClientOptions(BaseSettings):
    host: str
    port: int

    class Config:
        env_prefix = "ff_"


class FlightFusionClient:
    def __init__(
        self,
        options: ClientOptions,
    ) -> None:
        self._raw = RawFusionClient(options.host, options.port)

    @property
    def fusion(self) -> RawFusionClient:
        """Native flight fusion service client"""
        return self._raw

    def get_area_client(self, areas: List[str]) -> AreaClient:
        from flight_fusion.clients.area import AreaClient

        return AreaClient(self, areas)

    def get_dataset_client(self, name: str, areas: List[str]) -> DatasetClient:
        from flight_fusion.clients.dataset import TableClient

        return TableClient(
            client=self.get_area_client(areas),
            reference=AreaSourceReference(
                location=AreaTableLocation(name=name, areas=areas)
            ),
        )

    def put_memory_table(
        self,
        table_ref: str,
        data: Union[pd.DataFrame, pa.Table],
        save_mode: SaveMode = SaveMode.SAVE_MODE_OVERWRITE,
    ) -> ResultActionStatus:
        if isinstance(data, pd.DataFrame):
            data = pa.Table.from_pandas(data)
        batches = data.to_batches()
        raw_response = self._raw.put_memory_table(table_ref, batches)
        return ResultActionStatus().parse(raw_response)
