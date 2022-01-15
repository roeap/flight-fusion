from __future__ import annotations

from typing import TYPE_CHECKING, List

from flight_fusion._internal import FusionClient as RawFusionClient
from flight_fusion.clients.service import ClientOptions, FlightFusionClient

if TYPE_CHECKING:
    from .dataset import DataSetClient


class AreaClient:
    def __init__(self, client: FlightFusionClient, areas: List[str]) -> None:
        self._areas = areas
        self._client = client

    @classmethod
    def from_options(cls, areas: List[str], options: ClientOptions) -> AreaClient:
        """Create a new AreaClient instance from service options

        Args:
            areas (List[str]): area path
            options (ClientOptions): options used for initializing underlying client

        Returns:
            AreaClient: the client instance
        """
        return cls(FlightFusionClient(options), areas)

    @property
    def fusion(self) -> RawFusionClient:
        """Native flight fusion service client"""
        return self._client.fusion

    @property
    def areas(self) -> List[str]:
        return self.areas

    def get_dataset_client(self, name: str) -> DataSetClient:
        ...

    def create_dataset(self, name: str):
        pass

    def drop_dataset(self, name: str):
        pass
