from __future__ import annotations

from abc import abstractmethod
from typing import List, Union

import pandas as pd
import pyarrow as pa
from betterproto import which_one_of

import flight_fusion.errors as errors
from flight_fusion.clients.area import AreaClient
from flight_fusion.clients.service import ClientOptions
from flight_fusion.ipc.v1alpha1 import AreaSourceReference, AreaTableLocation, SaveMode


class DataSetClient:
    def __init__(self, client: AreaClient, reference: AreaSourceReference) -> None:
        self._reference = reference
        self._client = client

    @classmethod
    @abstractmethod
    def from_options(
        cls, name: str, areas: List[str], options: ClientOptions
    ) -> DataSetClient:
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

    @abstractmethod
    def write_into(
        self,
        data: Union[pd.DataFrame, pa.Table],
        save_mode: SaveMode = SaveMode.SAVE_MODE_OVERWRITE,
    ):
        pass

    @abstractmethod
    def create(self):
        pass

    @abstractmethod
    def load(self) -> pa.Table:
        pass

    @abstractmethod
    def drop(self):
        pass

    def get_metadata(self):
        # TODO implement ...
        pass


class TableClient(DataSetClient):
    def __init__(self, client: AreaClient, reference: AreaSourceReference) -> None:
        super().__init__(client, reference)

    @classmethod
    def from_options(
        cls, name: str, areas: List[str], options: ClientOptions
    ) -> TableClient:
        return cls(
            client=AreaClient.from_options(areas=areas, options=options),
            reference=AreaSourceReference(
                location=AreaTableLocation(name=name, areas=areas)
            ),
        )

    def write_into(
        self,
        data: Union[pd.DataFrame, pa.Table],
        save_mode: SaveMode = SaveMode.SAVE_MODE_OVERWRITE,
    ):
        pass

    def create(self):
        pass
