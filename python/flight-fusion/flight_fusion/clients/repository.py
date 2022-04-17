from flight_fusion.ipc.inference.model_repository import (
    RepositoryIndexResponse,
    RepositoryModelLoadResponse,
    RepositoryModelUnloadResponse,
)

from .aio import AsyncGrpcModelRepositoryServiceClient, run_async


class GrpcModelRepositoryServiceClient:
    def __init__(
        self,
        host: str = "localhost",
        port: int = 8081,
        use_ssl: bool = False,
        # credential: Union[bool, TokenCredential] = True,
    ) -> None:
        """_summary_

        Args:
            host (str, optional): _description_. Defaults to "localhost".
            port (int, optional): _description_. Defaults to 8081.
            use_ssl (bool, optional): _description_. Defaults to True.
        """
        self._client = AsyncGrpcModelRepositoryServiceClient(
            host=host,
            port=port,
            use_ssl=use_ssl,  # credential=credential
        )

    def repository_index(
        self, *, repository_name: str = "", ready: bool = False
    ) -> RepositoryIndexResponse:
        return run_async(
            self._client.repository_index, repository_name=repository_name, ready=ready
        )

    def repository_model_load(
        self, *, repository_name: str = "", model_name: str = ""
    ) -> RepositoryModelLoadResponse:
        return run_async(
            self._client.repository_model_load,
            repository_name=repository_name,
            model_name=model_name,
        )

    def repository_model_unload(
        self, *, repository_name: str = "", model_name: str = ""
    ) -> RepositoryModelUnloadResponse:
        return run_async(
            self._client.repository_model_unload,
            repository_name=repository_name,
            model_name=model_name,
        )
