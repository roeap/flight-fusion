from __future__ import annotations

from contextlib import asynccontextmanager

import httpx
from mlserver.errors import ModelNotFound
from mlserver.repository import ModelRepository
from mlserver.settings import ModelParameters, ModelSettings

from mlserver_fusion.types import ListRegisteredModelResponse, RegisteredModel


def _to_settings(model: RegisteredModel) -> ModelSettings:
    latest = model.latest_versions[0]
    versions = [v.version for v in model.latest_versions]
    parameters = ModelParameters(uri=latest.source, version=latest.version)
    return ModelSettings(
        name=model.name,
        versions=versions,
        implementation="mlserver_mlflow.MLflowRuntime",  # type: ignore
        parameters=parameters,
    )


class MlFlowRepository(ModelRepository):
    def __init__(self, service_url: str):
        self._base_url = f"{service_url}/api/2.0"

    @asynccontextmanager
    async def _client(self):
        async with httpx.AsyncClient(base_url=self._base_url) as client:
            yield client

    async def list(self) -> list[ModelSettings]:
        async with self._client() as client:
            response = await client.get("/preview/mlflow/registered-models/list")

        data = ListRegisteredModelResponse(**response.json())
        return [_to_settings(m) for m in data.registered_models]

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
