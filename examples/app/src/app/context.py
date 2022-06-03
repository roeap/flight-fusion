from __future__ import annotations

import json
from typing import TYPE_CHECKING

import flask
from dash._callback_context import CallbackContext as _CallbackContext
from dash._callback_context import has_context

from flight_fusion import (
    AssetKey,
    BaseDatasetClient,
    ClientOptions,
    FusionServiceClient,
)

if TYPE_CHECKING:
    import pyarrow as pa


def _assert_asset_key(asset_key: AssetKey | list[str] | str) -> AssetKey:
    if isinstance(asset_key, AssetKey):
        return asset_key
    if isinstance(asset_key, str):
        return AssetKey(json.loads(asset_key))
    return AssetKey(asset_key)


class DashAssetClient:
    def __init__(self, client: BaseDatasetClient) -> None:
        self._client = client

    @property
    def client(self) -> BaseDatasetClient:
        return self._client

    def column_options(self):
        return [{"label": f.name, "value": f.name} for f in self.client.schema()]

    def load_columns(self, columns: list[str]) -> pa.Table:
        return self.client.load(columns=columns)


class CallbackContext(_CallbackContext):
    @property
    @has_context
    def fusion(self) -> FusionServiceClient:
        if not hasattr(flask.g, "_fusion"):
            ffc = FusionServiceClient(ClientOptions(host="localhost", port=50051))
            setattr(flask.g, "_fusion", ffc)
        return getattr(flask.g, "_fusion")

    @has_context
    def get_dataset_client(self, asset_key: AssetKey | list[str] | str) -> DashAssetClient:
        asset_key = _assert_asset_key(asset_key)
        return DashAssetClient(self.fusion.get_dataset_client(asset_key=asset_key))


ctx = CallbackContext()
