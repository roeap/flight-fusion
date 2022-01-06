from typing import Optional, Union

import pandas as pd
import pyarrow as pa
import pyarrow.flight as flight

from flight_fusion._internal import FusionClient as RawFusionClient
from flight_fusion.ipc.v1alpha1 import (
    CommandSqlOperation,
    DropDatasetResponse,
    FlightDoGetRequest,
    PutMemoryTableResponse,
)
from flight_fusion.proto.actions_pb2 import (
    RegisterDatasetRequest,
    RegisterDatasetResponse,
)
from flight_fusion.proto.common_pb2 import DeltaReference, SaveMode
from flight_fusion.proto.message_pb2 import FlightActionRequest, FlightDoPutRequest
from flight_fusion.proto.tickets_pb2 import DeltaOperationRequest, DeltaWriteOperation


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

    def drop_table(self, table_name: str) -> DropDatasetResponse:
        raw_response = self._raw.drop_table(table_name)
        return DropDatasetResponse().parse(raw_response)

    def register_memory_table(
        self, table_name: str, data: Union[pd.DataFrame, pa.Table]
    ) -> PutMemoryTableResponse:
        if isinstance(data, pd.DataFrame):
            data = pa.Table.from_pandas(data)
        batches = data.to_batches()
        raw_response = self._raw.register_memory_table(table_name, batches)
        return PutMemoryTableResponse().parse(raw_response)

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

        op = DeltaWriteOperation(save_mode=save_mode)  # type: ignore

        command = DeltaOperationRequest(
            table=DeltaReference(location=location), write=op
        )

        request = FlightDoPutRequest()
        request.delta.CopyFrom(command)

        descriptor = flight.FlightDescriptor.for_command(request.SerializeToString())
        writer, _ = self.flight_client.do_put(descriptor, data.schema)
        writer.write_table(data)
        writer.close()

    def register_remote_dataset(
        self, catalog: str, schema: str, table: str, path: str
    ) -> RegisterDatasetResponse:

        register_table_action = RegisterDatasetRequest()
        register_table_action.path = path
        register_table_action.name = table

        action_body = FlightActionRequest()
        action_body.register.CopyFrom(register_table_action)

        action = flight.Action(
            FlightActions.REGISTER_TABLE, action_body.SerializeToString()
        )
        res = list(self.flight_client.do_action(action))

        response = RegisterDatasetResponse()
        response.ParseFromString(res[0].body.to_pybytes())

        return response

    def execute_query(self, query: str) -> pa.Table:
        request = FlightDoGetRequest(sql=CommandSqlOperation(query=query))
        ticket = flight.Ticket(ticket=request.SerializeToString())
        reader = self.flight_client.do_get(ticket)
        table = reader.read_all()
        return table
