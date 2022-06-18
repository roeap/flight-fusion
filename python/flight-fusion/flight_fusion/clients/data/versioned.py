from __future__ import annotations

import datetime as dt

import pandas as pd
import pyarrow as pa

from flight_fusion.ipc.v1alpha1 import (
    DeltaOperationRequest,
    DeltaReadOperation,
    DeltaWriteOperation,
    FlightDoGetRequest,
    FlightDoPutRequest,
    ResultDoPutUpdate,
    SaveMode,
)

from ._dataset import BaseDatasetClient


class VersionedDatasetClient(BaseDatasetClient):
    def write_into(
        self,
        data: pd.DataFrame | pa.Table,
        save_mode: SaveMode = SaveMode.SAVE_MODE_APPEND,
        partition_by: list[str] | None = None,
        predicate: str | None = None,
    ) -> ResultDoPutUpdate:
        if isinstance(data, pd.DataFrame):
            data = pa.Table.from_pandas(data)
        data = data.replace_schema_metadata({})
        command = FlightDoPutRequest(
            delta=DeltaOperationRequest(
                source=self._reference,
                write=DeltaWriteOperation(
                    save_mode=save_mode,
                    partition_by=partition_by or [],
                    predicate=predicate,
                ),
            )
        )
        response = self._do_put(table=data, command=command)
        self.log.debug(str(response))
        return ResultDoPutUpdate().parse(response)

    def load(self, columns: list[str] | None = None) -> pa.Table:
        command = FlightDoGetRequest(
            delta=DeltaOperationRequest(
                source=self._reference,
                read=DeltaReadOperation(column_names=columns or []),
            )
        )
        return self._do_get(command)

    def load_version(self, version: int, columns: list[str] | None = None) -> pa.Table:
        command = FlightDoGetRequest(
            delta=DeltaOperationRequest(
                source=self._reference,
                read=DeltaReadOperation(version=version, column_names=columns or []),
            )
        )
        return self._do_get(command)

    def load_timetravel(self, before_or_at: dt.datetime | pd.Timestamp) -> pa.Table:
        raise NotImplementedError
