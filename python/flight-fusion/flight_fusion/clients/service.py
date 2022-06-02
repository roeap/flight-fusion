from __future__ import annotations

from dataclasses import dataclass
from functools import lru_cache
from typing import TYPE_CHECKING, Iterable

import pyarrow.flight as pa_flight

from flight_fusion.asset_key import AssetKey
from flight_fusion.clients import ContextClient, DatasetClient, VersionedDatasetClient
from flight_fusion.ipc.v1alpha1 import AreaSourceMetadata, AreaSourceReference

from ..errors import ResourceDoesNotExist
from ._base import BaseClient, ClientOptions, asset_key_to_source

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

    @lru_cache
    def _area_meta(self, asset_key: AssetKey) -> AreaSourceMetadata:
        source = asset_key_to_source(asset_key=asset_key)
        return self._get_metadata(reference=source)

    def get_context(self, refs: Iterable[AssetKey]) -> ContextClient:
        return ContextClient(
            sources=[asset_key_to_source(asset_key=ak) for ak in refs],
            client=self._flight,
        )

    def new_dataset_client(self, asset_key: AssetKey, versioned: bool = True) -> BaseDatasetClient:
        if versioned:
            return VersionedDatasetClient(asset_key=asset_key, client=self._flight)
        return DatasetClient(asset_key=asset_key, client=self._flight)

    def get_dataset_client(self, asset_key: AssetKey) -> BaseDatasetClient:
        try:
            meta = self._area_meta(asset_key)
            versioned = meta.is_versioned
        except ResourceDoesNotExist:
            versioned = True
        if versioned:
            return VersionedDatasetClient(asset_key=asset_key, client=self._flight)
        return DatasetClient(asset_key=asset_key, client=self._flight)

    def list_datasets(self) -> list[AreaInfo]:
        return [
            AreaInfo(source=AreaSourceReference.FromString(fi.descriptor.command), info=fi)
            for fi in self._flight.list_flights()
        ]
