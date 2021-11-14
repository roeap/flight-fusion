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

        # descriptor = flight.FlightDescriptor.for_path(f"{catalog}::{schema}::{table}")
        descriptor = flight.FlightDescriptor.for_path(f"{table}")
        writer, _response = self.flight_client.do_put(descriptor, data.schema)
        writer.write_table(data)
        writer.close()

    def register_remote_dataset(
        self, catalog: str, schema: str, table: str, path: str
    ) -> None:
        # action_body = f"{catalog}::{schema}::{table}::{path}"
        action_body = f"{table}::{path}"
        action = flight.Action(FlightActions.REGISTER_TABLE, action_body.encode())
        res = list(self.flight_client.do_action(action))
        res[0].body.to_pybytes().decode()

    def execute_query(self, query: str) -> pa.Table:
        ticket = flight.Ticket(ticket=query.encode())
        reader = self.flight_client.do_get(ticket)
        table = reader.read_all()
        return table
