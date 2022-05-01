from __future__ import annotations

import pandas as pd
import pyarrow as pa

from flight_fusion.ipc.v1alpha1 import (
    CommandReadDataset,
    CommandWriteIntoDataset,
    FlightDoGetRequest,
    FlightDoPutRequest,
    ResultDoPutUpdate,
    SaveMode,
)

from ._dataset import BaseDatasetClient


class DatasetClient(BaseDatasetClient):
    def write_into(
        self,
        data: pd.DataFrame | pa.Table,
        save_mode: SaveMode = SaveMode.SAVE_MODE_APPEND,
        partition_by: list[str] | None = None,
        predicate: str | None = None,
    ) -> ResultDoPutUpdate:
        if predicate is not None:
            raise NotImplementedError("Write predicates not yet implemented.")
        if partition_by is not None:
            raise NotImplementedError("Column partitioning not yet implemented.")
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
