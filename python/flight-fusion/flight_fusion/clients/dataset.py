from __future__ import annotations

from abc import abstractmethod
from typing import List, Union

import pandas as pd
import pyarrow as pa

from flight_fusion.ipc.v1alpha1 import SaveMode

from .area import AreaClient
from .service import ClientOptions


class DataSetClient:
    def __init__(self, client: AreaClient, name: str) -> None:
        self._name = name
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
    def load(self):
        pass

    @abstractmethod
    def drop(self):
        pass

    def get_metadata(self):
        # TODO implement ...
        pass


class TableClient(DataSetClient):
    def __init__(self, client: AreaClient, name: str) -> None:
        super().__init__(client, name)

    @classmethod
    def from_options(
        cls, name: str, areas: List[str], options: ClientOptions
    ) -> TableClient:
        return cls(AreaClient.from_options(areas, options), name)

    def write_into(
        self,
        data: Union[pd.DataFrame, pa.Table],
        save_mode: SaveMode = SaveMode.SAVE_MODE_OVERWRITE,
    ):
        pass

    def create(self):
        pass
