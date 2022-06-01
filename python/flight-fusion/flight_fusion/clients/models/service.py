from __future__ import annotations

from flight_fusion.ipc.inference.model_repository import (
    RepositoryIndexResponse,
    RepositoryModelLoadResponse,
    RepositoryModelUnloadResponse,
)

from .inference import GrpcInferenceServiceClient, GrpcModelRepositoryServiceClient
from .model import GrpcModelClient, ModelClient


class ModelServiceClient:
    def __init__(
        self,
        host: str = "localhost",
        port: int = 8081,
        use_ssl: bool = False,
    ) -> None:
        self._client = GrpcInferenceServiceClient(host=host, port=port, use_ssl=use_ssl)
        self._repo_client = GrpcModelRepositoryServiceClient(host=host, port=port, use_ssl=use_ssl)

    def list_models(self, repository_name: str = "", ready: bool = False) -> RepositoryIndexResponse:
        return self._repo_client.repository_index(repository_name=repository_name, ready=ready)

    def load_model(self, repository_name: str = "", model_name: str = "") -> RepositoryModelLoadResponse:
        return self._repo_client.repository_model_load(repository_name=repository_name, model_name=model_name)

    def unload_model(self, repository_name: str = "", model_name: str = "") -> RepositoryModelUnloadResponse:
        return self._repo_client.repository_model_unload(repository_name=repository_name, model_name=model_name)

    def get_model_client(self, name: str, version: str | None = None) -> ModelClient:
        return GrpcModelClient(client=self._client, name=name, version=version)
