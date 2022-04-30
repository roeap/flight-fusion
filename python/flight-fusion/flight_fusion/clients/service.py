from __future__ import annotations

from typing import TYPE_CHECKING, Iterable, List, Tuple

from flight_fusion.clients import DatasetClient, VersionedDatasetClient
from flight_fusion.ipc.v1alpha1 import AreaSourceReference, AreaTableLocation

from ._base import BaseClient, ClientOptions

if TYPE_CHECKING:
    from flight_fusion.clients import BaseDatasetClient, ContextClient


class FlightActions:
    REGISTER_TABLE = b"register-table"
    REGISTER_DELTA_TABLE = b"register-delta-table"


class FusionServiceClient(BaseClient):
    def __init__(
        self,
        options: ClientOptions,
    ) -> None:
        super().__init__(options=options)

    def get_source_reference(self, name: str, areas: List[str]) -> AreaSourceReference:
        return AreaSourceReference(location=AreaTableLocation(name=name, areas=areas))

    def get_context(self, refs: Iterable[Tuple[str, List[str]]]) -> ContextClient:
        from flight_fusion.clients.data.context import ContextClient

        return ContextClient(
            sources=[self.get_source_reference(name, areas) for name, areas in refs],
            client=self._flight,
        )

    def get_dataset_client(self, name: str, areas: List[str]) -> BaseDatasetClient:
        return DatasetClient(
            client=self._flight,
            reference=AreaSourceReference(location=AreaTableLocation(name=name, areas=areas)),
        )

    def get_versioned_dataset_client(self, name: str, areas: List[str]) -> BaseDatasetClient:
        """_summary_

        Args:
            name (str): _description_
            areas (List[str]): _description_

        Returns:
            BaseDatasetClient: _description_
        """
        return VersionedDatasetClient(
            client=self._flight,
            reference=AreaSourceReference(location=AreaTableLocation(name=name, areas=areas)),
        )
