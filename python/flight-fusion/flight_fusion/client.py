from typing import List, Optional, Union

import pandas as pd
import pyarrow as pa
import pyarrow.flight as flight

from flight_fusion._internal import FusionClient as RawFusionClient
from flight_fusion.ipc.v1alpha1 import (
    CommandSqlOperation,
    DeltaOperationRequest,
    DeltaReference,
    DeltaWriteOperation,
    DoPutUpdateResult,
    DropDatasetResponse,
    FlightActionRequest,
    FlightDoGetRequest,
    FlightDoPutRequest,
    PutMemoryTableResponse,
    RegisterDatasetRequest,
    RegisterDatasetResponse,
    SaveMode,
)


class FlightActions:
    REGISTER_TABLE = b"register-table"
    REGISTER_DELTA_TABLE = b"register-delta-table"


class FlightFusionClient:
    def __init__(
        self,
        host: str = "localhost",
        port: Union[int, str] = 50051,
        client: Optional[flight.FlightClient] = None,
    ) -> None:
        self._client = None
        if client is not None:
            self._client = client
        if self._client is None:
            self._client = flight.FlightClient((host, int(port)))

        self._meta = None
        self._raw = RawFusionClient()

    def _get_schema(self):
        pass

    def meta(self):
        return self._meta

    @property
    def flight_client(self) -> flight.FlightClient:
        return self._client

    def drop_table(self, table_ref: str) -> DropDatasetResponse:
        raw_response = self._raw.drop_table(table_ref)
        return DropDatasetResponse().parse(raw_response)

    def put_memory_table(
        self,
        table_ref: str,
        data: Union[pd.DataFrame, pa.Table],
        save_mode: SaveMode = SaveMode.SAVE_MODE_OVERWRITE,
    ) -> PutMemoryTableResponse:
        if isinstance(data, pd.DataFrame):
            data = pa.Table.from_pandas(data)
        batches = data.to_batches()
        raw_response = self._raw.put_memory_table(table_ref, batches)
        return PutMemoryTableResponse().parse(raw_response)

    def write_into_table(
        self,
        table_ref: str,
        data: Union[pd.DataFrame, pa.Table],
        save_mode: SaveMode = SaveMode.SAVE_MODE_OVERWRITE,
    ) -> DoPutUpdateResult:
        if isinstance(data, pd.DataFrame):
            data = pa.Table.from_pandas(data)
        batches = data.to_batches()
        raw_response = self._raw.write_into_table(table_ref, save_mode.value, batches)
        return DoPutUpdateResult().parse(raw_response)

    def read_table(self, table_ref: str) -> List[pa.RecordBatch]:
        return self._raw.read_table(table_ref)

    def write_into_delta(
        self,
        catalog: str,
        schema: str,
        location: str,
        save_mode: SaveMode,
        data: Union[pd.DataFrame, pa.Table],
    ) -> None:
        if isinstance(data, pd.DataFrame):
            # NOTE removing table meta data is required since we validate the
            # schema against the delta table schema on the server side.
            data = pa.Table.from_pandas(data).replace_schema_metadata()

        request = FlightDoPutRequest(
            delta=DeltaOperationRequest(
                table=DeltaReference(location=location),
                write=DeltaWriteOperation(save_mode=save_mode),
            )
        )
        descriptor = flight.FlightDescriptor.for_command(request.SerializeToString())
        writer, _ = self.flight_client.do_put(descriptor, data.schema)
        writer.write_table(data)
        writer.close()

    def register_remote_dataset(
        self, catalog: str, schema: str, table: str, path: str
    ) -> RegisterDatasetResponse:
        action_body = FlightActionRequest(
            register=RegisterDatasetRequest(path=path, name=table)
        )
        action = flight.Action(
            FlightActions.REGISTER_TABLE, action_body.SerializeToString()
        )
        res = list(self.flight_client.do_action(action))
        return RegisterDatasetResponse().parse(res[0].body.to_pybytes())

    def execute_query(self, query: str) -> pa.Table:
        request = FlightDoGetRequest(sql=CommandSqlOperation(query=query))
        ticket = flight.Ticket(ticket=request.SerializeToString())
        reader = self.flight_client.do_get(ticket)
        table = reader.read_all()
        return table
