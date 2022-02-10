from __future__ import annotations

from abc import abstractmethod
from typing import List, Union

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
    CommandReadDataset,
    CommandWriteIntoDataset,
    FlightDoGetRequest,
    FlightDoPutRequest,
    ResultActionStatus,
    ResultDoPutUpdate,
    SaveMode,
)


class DatasetClient:
    def __init__(self, client: AreaClient, reference: AreaSourceReference):
        self._reference = reference
        self._client = client
        self._schema = None

    @classmethod
    @abstractmethod
    def from_options(cls, name: str, areas: List[str], options: ClientOptions) -> DatasetClient:
        """Create a new DatasetClient instance from service options

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
            raise NotImplementedError
        return self._schema

    def write_into(
        self,
        data: Union[pd.DataFrame, pa.Table],
        save_mode: SaveMode = SaveMode.SAVE_MODE_OVERWRITE,
    ) -> ResultDoPutUpdate:
        if isinstance(data, pd.DataFrame):
            data = pa.Table.from_pandas(data)
        data = data.replace_schema_metadata({})
        command = FlightDoPutRequest(
            storage=CommandWriteIntoDataset(source=self._reference, save_mode=save_mode)
        )
        response = self._client.client._do_put(table=data, command=command)
        return ResultDoPutUpdate().parse(response)

    def load(self) -> pa.Table:
        command = FlightDoGetRequest(read=CommandReadDataset(source=self._reference))
        return self._client.client._do_get(command)

    def query(self, query: str) -> pa.Table:
        command = FlightDoGetRequest(query=CommandExecuteQuery(query=query, source=self._reference))
        return self._client.client._do_get(command)

    def drop(self) -> ResultActionStatus:
        raise NotImplementedError

    def get_metadata(self) -> AreaSourceMetadata:
        raise NotImplementedError

    def set_metadata(self, metadata: AreaSourceMetadata = None) -> None:
        raise NotImplementedError


class TableClient(DatasetClient):
    def __init__(self, client: AreaClient, reference: AreaSourceReference) -> None:
        super().__init__(client, reference)

    @classmethod
    def from_options(cls, name: str, areas: List[str], options: ClientOptions) -> TableClient:
        return cls(
            client=AreaClient.from_options(areas=areas, options=options),
            reference=AreaSourceReference(location=AreaTableLocation(name=name, areas=areas)),
        )
