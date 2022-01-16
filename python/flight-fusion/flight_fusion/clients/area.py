from __future__ import annotations

from typing import TYPE_CHECKING, List

from flight_fusion._internal import FusionClient as RawFusionClient
from flight_fusion.clients.service import (
    AreaSourceReference,
    AreaTableLocation,
    ClientOptions,
    FlightFusionClient,
)
from flight_fusion.ipc.v1alpha1 import AreaSourceMetadata

if TYPE_CHECKING:
    from .dataset import DatasetClient


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

    def get_dataset_client(self, name: str) -> DatasetClient:
        from flight_fusion.clients import DatasetClient

        return DatasetClient(
            client=self,
            reference=AreaSourceReference(
                location=AreaTableLocation(name=name, areas=self.areas)
            ),
        )

    def list_sources(self, recursive: bool = False) -> List[AreaSourceMetadata]:
        ...
