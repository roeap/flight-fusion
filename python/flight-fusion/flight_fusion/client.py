from typing import Optional, Union

import pandas as pd
import pyarrow as pa
import pyarrow.flight as flight
from sqlalchemy.dialects.postgresql import (
    BIGINT,
    BOOLEAN,
    DATE,
    FLOAT,
    INTEGER,
    REAL,
    SMALLINT,
    TIME,
    TIMESTAMP,
    VARCHAR,
)

from flight_fusion.proto.actions_pb2 import (
    RegisterDatasetRequest,
    RegisterDatasetResponse,
)
from flight_fusion.proto.common_pb2 import DeltaReference, SaveMode
from flight_fusion.proto.message_pb2 import (
    FlightActionRequest,
    FlightDoGetRequest,
    FlightDoPutRequest,
)
from flight_fusion.proto.tickets_pb2 import (
    DeltaOperationRequest,
    PutMemoryTableRequest,
    SqlTicket,
)

type_map = {
    "Utf8": VARCHAR,
    "Float32": FLOAT,
    "Int16": SMALLINT,
    "Int32": INTEGER,
    "Int64": BIGINT,
    "Float64": REAL,
    "Boolean": BOOLEAN,
    "Date32": DATE,
    "Time64": TIME,
    "Timestamp": TIMESTAMP,
}


class FlightActions:
    REGISTER_TABLE = b"register-table"
    REGISTER_DELTA_TABLE = b"register-delta-table"


def _validate_segment(segment: str):
    return "." not in segment


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

    def _get_schema(self):
        pass

    def meta(self):
        return self._meta

    @property
    def flight_client(self) -> flight.FlightClient:
        return self._client

    def register_dataset(
        self, catalog: str, schema: str, table: str, data: Union[pd.DataFrame, pa.Table]
    ) -> None:
        if isinstance(data, pd.DataFrame):
            data = pa.Table.from_pandas(data)

        command = PutMemoryTableRequest()
        command.name = table

        request = FlightDoPutRequest()
        request.memory.CopyFrom(command)

        descriptor = flight.FlightDescriptor.for_command(request.SerializeToString())
        writer, _ = self.flight_client.do_put(descriptor, data.schema)
        writer.write_table(data)
        writer.close()

    def write_into_delta(
        self,
        catalog: str,
        schema: str,
        location: str,
        save_mode: SaveMode,
        data: Union[pd.DataFrame, pa.Table],
    ) -> None:
        if isinstance(data, pd.DataFrame):
            data = pa.Table.from_pandas(data)

        command = DeltaOperationRequest(table=DeltaReference(location=location))

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
        sql_ticket = SqlTicket()
        sql_ticket.query = query

        request = FlightDoGetRequest()
        request.sql.CopyFrom(sql_ticket)

        ticket = flight.Ticket(ticket=request.SerializeToString())
        reader = self.flight_client.do_get(ticket)
        table = reader.read_all()
        return table
