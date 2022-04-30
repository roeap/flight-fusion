from __future__ import annotations

import pandas as pd
import pyarrow as pa
import pyarrow.flight as pa_flight

from flight_fusion.asset_key import AssetKey
from flight_fusion.ipc.v1alpha1 import (
    CommandReadDataset,
    CommandWriteIntoDataset,
    FlightDoGetRequest,
    FlightDoPutRequest,
    ResultDoPutUpdate,
    SaveMode,
)

from .._base import ClientOptions
from ._dataset import BaseDatasetClient


class DatasetClient(BaseDatasetClient):
    def __init__(
        self,
        asset_key: AssetKey,
        client: pa_flight.FlightClient | None = None,
        options: ClientOptions | None = None,
    ):
        super().__init__(asset_key=asset_key, client=client, options=options)

    def write_into(
        self,
        data: pd.DataFrame | pa.Table,
        save_mode: SaveMode = SaveMode.SAVE_MODE_APPEND,
    ) -> ResultDoPutUpdate:
        if isinstance(data, pd.DataFrame):
            data = pa.Table.from_pandas(data)
        data = data.replace_schema_metadata({})
        command = FlightDoPutRequest(
            storage=CommandWriteIntoDataset(source=self._reference, save_mode=save_mode)
        )
        response = self._do_put(table=data, command=command)
        return ResultDoPutUpdate().parse(response)

    def load(self) -> pa.Table:
        command = FlightDoGetRequest(read=CommandReadDataset(source=self._reference))
        return self._do_get(command)
