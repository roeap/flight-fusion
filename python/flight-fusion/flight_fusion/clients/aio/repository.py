from __future__ import annotations

from contextlib import asynccontextmanager

from grpclib.client import Channel
from grpclib.events import SendRequest, listen

from flight_fusion.ipc.inference.model_repository import (
    ModelRepositoryServiceStub,
    RepositoryIndexResponse,
    RepositoryModelLoadResponse,
    RepositoryModelUnloadResponse,
)


class AsyncGrpcModelRepositoryServiceClient:
    def __init__(
        self,
        host: str = "localhost",
        port: int = 8081,
        use_ssl: bool = False,
        # credential: Union[bool, TokenCredential] = True,
    ) -> None:
        """Asynchronous client for interacting with the Verbund services.

        Args:
            host (str, optional): server host name. Defaults to "localhost".
            port (int, optional): server port number. Defaults to 8081.
            use_ssl (bool, optional): use a secure channe for connection. Defaults to True.
        """
        self._host = host
        self._port = port
        self._use_ssl = use_ssl
        self._auth = None
        # if isinstance(credential, bool):
        #     if credential:
        #         self._auth = SendrequestAuth()
        # elif credential is not None:
        #     self._auth = SendrequestAuth(credential=credential)

    @asynccontextmanager
    async def _service(self):
        async with Channel(host=self._host, port=self._port, ssl=self._use_ssl) as channel:
            if self._auth is not None:
                listen(channel, SendRequest, self._auth)
            yield ModelRepositoryServiceStub(channel)

    async def repository_index(
        self, *, repository_name: str = "", ready: bool = False
    ) -> RepositoryIndexResponse:
        async with self._service() as service:
            return await service.repository_index(repository_name=repository_name, ready=ready)

    async def repository_model_load(
        self, *, repository_name: str = "", model_name: str = ""
    ) -> RepositoryModelLoadResponse:
        async with self._service() as service:
            return await service.repository_model_load(
                repository_name=repository_name, model_name=model_name
            )

    async def repository_model_unload(
        self, *, repository_name: str = "", model_name: str = ""
    ) -> RepositoryModelUnloadResponse:
        async with self._service() as service:
            return await service.repository_model_unload(
                repository_name=repository_name, model_name=model_name
            )
