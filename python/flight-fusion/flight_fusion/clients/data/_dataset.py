from __future__ import annotations

from abc import abstractmethod
from typing import List, Optional

import pandas as pd
import pyarrow as pa
from betterproto import which_one_of

import flight_fusion.errors as errors
from flight_fusion.clients.area import AreaClient
from flight_fusion.clients.service import ClientOptions
from flight_fusion.ipc.v1alpha1 import (
    AreaSourceMetadata,
    AreaSourceReference,
    AreaTableLocation,
    CommandExecuteQuery,
    FlightDoGetRequest,
    FlightGetFlightInfoRequest,
    ResultActionStatus,
    ResultDoPutUpdate,
    SaveMode,
)


class BaseDatasetClient:
    def __init__(self, client: AreaClient, reference: AreaSourceReference):
        self._reference = reference
        self._client = client
        self._schema = None

    @classmethod
    @abstractmethod
    def from_options(cls, name: str, areas: List[str], options: ClientOptions) -> BaseDatasetClient:
        """Create a new BaseDatasetClient instance from service options

        Args:
            name (str): name of the dataset
            areas (List[str]): area path where dataset is defined
            options (ClientOptions): options used for initializing underlying client

        Returns:
            DataSetClient: the client instance
        """
        ...

    @property
    def name(self) -> str:
        field, value = which_one_of(self._reference, "table")
        if isinstance(value, AreaTableLocation):
            return value.name
        raise errors.TableSource(f"Variant {field} not yet supported.")

    @property
    def areas(self) -> List[str]:
        field, value = which_one_of(self._reference, "table")
        if isinstance(value, AreaTableLocation):
            return value.areas
        raise errors.TableSource(f"Variant {field} not yet supported.")

    def schema(self) -> pa.Schema:
        if self._schema is None:
            request = pa.flight.FlightDescriptor.for_command(
                FlightGetFlightInfoRequest(source=self._reference).SerializeToString()
            )
            response = self._client.client._client.get_schema(request)
            self._schema = response.schema
        return self._schema

    @abstractmethod
    def write_into(
        self,
        data: pd.DataFrame | pa.Table,
        save_mode: SaveMode = SaveMode.SAVE_MODE_APPEND,
    ) -> ResultDoPutUpdate:
        raise NotImplementedError

    @abstractmethod
    def load(self) -> pa.Table:
        raise NotImplementedError

    def query(self, query: str) -> pa.Table:
        command = FlightDoGetRequest(query=CommandExecuteQuery(query=query, source=self._reference))
        return self._client.client._do_get(command)

    def drop(self) -> ResultActionStatus:
        raise NotImplementedError

    def get_metadata(self) -> AreaSourceMetadata:
        request = pa.flight.FlightDescriptor.for_command(
            FlightGetFlightInfoRequest(source=self._reference).SerializeToString()
        )
        return self._client.client._client.get_flight_info(request)

    def set_metadata(self, metadata: Optional[AreaSourceMetadata] = None) -> None:
        raise NotImplementedError
