from __future__ import annotations

from dataclasses import dataclass
from typing import TYPE_CHECKING, Iterable

import pyarrow.flight as pa_flight

from flight_fusion.asset_key import AssetKey
from flight_fusion.clients import ContextClient, DatasetClient, VersionedDatasetClient
from flight_fusion.ipc.v1alpha1 import AreaSourceReference, AreaTableLocation

from ._base import BaseClient, ClientOptions

if TYPE_CHECKING:
    from flight_fusion.clients import BaseDatasetClient


class FlightActions:
    REGISTER_TABLE = b"register-table"
    REGISTER_DELTA_TABLE = b"register-delta-table"


@dataclass
class AreaInfo:
    source: AreaSourceReference
    info: pa_flight.FlightInfo  # type: ignore

    @property
    def asset_key(self) -> AssetKey:
        return AssetKey(self.source.location.areas + [self.source.location.name])


class FusionServiceClient(BaseClient):
    def __init__(
        self,
        options: ClientOptions,
    ) -> None:
        super().__init__(options=options)

    def get_source_reference(self, name: str, areas: list[str]) -> AreaSourceReference:
        return AreaSourceReference(location=AreaTableLocation(name=name, areas=areas))

    def get_context(self, refs: Iterable[tuple[str, list[str]]]) -> ContextClient:
        return ContextClient(
            sources=[self.get_source_reference(name, areas) for name, areas in refs],
            client=self._flight,
        )

    def get_dataset_client(self, asset_key: AssetKey) -> BaseDatasetClient:
        return DatasetClient(asset_key=asset_key, client=self._flight)

    def get_versioned_dataset_client(self, asset_key: AssetKey) -> BaseDatasetClient:
        return VersionedDatasetClient(asset_key=asset_key, client=self._flight)

    def list_datasets(self) -> list[AreaInfo]:
        return [
            AreaInfo(
                source=AreaSourceReference.FromString(fi.descriptor.command), info=fi
            )
            for fi in self._flight.list_flights()
        ]
