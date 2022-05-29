from __future__ import annotations

from abc import abstractmethod

import pandas as pd
import pyarrow as pa
import pyarrow.flight as pa_flight
from betterproto import which_one_of

import flight_fusion.errors as errors
from flight_fusion.asset_key import AssetKey
from flight_fusion.ipc.v1alpha1 import (
    AreaSourceMetadata,
    AreaTableLocation,
    CommandExecuteQuery,
    FlightDoGetRequest,
    FlightGetFlightInfoRequest,
    ResultActionStatus,
    ResultDoPutUpdate,
    SaveMode,
)

from .._base import BaseClient, ClientOptions, asset_key_to_source


class BaseDatasetClient(BaseClient):
    def __init__(
        self,
        asset_key: AssetKey,
        client: pa_flight.FlightClient | None = None,
        options: ClientOptions | None = None,
    ):
        super().__init__(client=client, options=options)

        self._reference = asset_key_to_source(asset_key=asset_key)
        self._client = client
        self._options = options
        self._schema = None

    @property
    def name(self) -> str:
        field, value = which_one_of(self._reference, "table")
        if isinstance(value, AreaTableLocation):
            return value.name
        raise errors.TableSource(f"Variant {field} not yet supported.")

    @property
    def namespace(self) -> list[str]:
        field, value = which_one_of(self._reference, "table")
        if isinstance(value, AreaTableLocation):
            return value.areas
        raise errors.TableSource(f"Variant {field} not yet supported.")

    def schema(self) -> pa.Schema:
        if self._schema is None:
            request = pa_flight.FlightDescriptor.for_command(
                FlightGetFlightInfoRequest(source=self._reference).SerializeToString()
            )
            response = self._flight.get_schema(request)
            self._schema = response.schema
        return self._schema

    @abstractmethod
    def write_into(
        self,
        data: pd.DataFrame | pa.Table,
        save_mode: SaveMode = SaveMode.SAVE_MODE_APPEND,
        partition_by: list[str] | None = None,
        predicate: str | None = None,
    ) -> ResultDoPutUpdate:
        raise NotImplementedError

    @abstractmethod
    def load(self, columns: list[str] | None = None) -> pa.Table:
        raise NotImplementedError

    def query(self, query: str) -> pa.Table:
        command = FlightDoGetRequest(
            query=CommandExecuteQuery(query=query, source=self._reference)
        )
        return self._do_get(command)

    def drop(self) -> ResultActionStatus:
        raise NotImplementedError

    def get_metadata(self) -> AreaSourceMetadata:
        request = pa_flight.FlightDescriptor.for_command(
            FlightGetFlightInfoRequest(source=self._reference).SerializeToString()
        )
        return self._flight.get_flight_info(request)

    def set_metadata(self, metadata: AreaSourceMetadata | None = None) -> None:
        raise NotImplementedError
