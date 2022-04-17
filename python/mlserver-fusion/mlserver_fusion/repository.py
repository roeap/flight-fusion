from __future__ import annotations

from contextlib import asynccontextmanager
from itertools import chain

import httpx
from flight_fusion.ipc.mlflow import ListRegisteredModelsResponse, RegisteredModel
from mlserver.errors import ModelNotFound
from mlserver.repository import ModelRepository
from mlserver.settings import ModelParameters, ModelSettings


def _to_settings(model: RegisteredModel) -> list[ModelSettings]:
    items = []
    for version in model.latest_versions:
        extra = {"mlflow_run_id": version.run_id, "current_stage": version.current_stage}
        parameters = ModelParameters(uri=version.source, version=version.version, extra=extra)
        items.append(
            ModelSettings(
                name=version.name,
                versions=[version.version],
                implementation="mlserver_fusion.MLfusionRuntime",  # type: ignore
                parameters=parameters,
            )
        )
    return items


class MlFlowRepository(ModelRepository):
    """Model repository which fetches model meta data directly from MlFlow.

    The original implementation relied on a file written to disk to discover models.
    """

    def __init__(self, service_url: str):
        self._base_url = f"{service_url}/api/2.0"

    @asynccontextmanager
    async def _client(self):
        async with httpx.AsyncClient(base_url=self._base_url) as client:
            yield client

    async def list(self) -> list[ModelSettings]:
        async with self._client() as client:
            response = await client.get("/preview/mlflow/registered-models/list")

        data = ListRegisteredModelsResponse().from_dict(response.json())
        return list(chain(*[_to_settings(m) for m in data.registered_models]))

    async def find(self, name: str) -> list[ModelSettings]:
        # TODO use the mlflow search endpoint for this
        all_settings = await self.list()
        selected = []
        for model_settings in all_settings:
            # TODO: Implement other version policies (e.g. "Last N")
            if model_settings.name == name:
                selected.append(model_settings)

        if len(selected) == 0:
            raise ModelNotFound(name)

        return selected
