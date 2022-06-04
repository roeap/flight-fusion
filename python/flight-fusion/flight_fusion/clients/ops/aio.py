from __future__ import annotations

from contextlib import asynccontextmanager
from typing import AsyncIterable, Iterable, TypeVar

from grpclib.client import Channel

from flight_fusion.clients.events import SendrequestAuth, TokenCredential
from flight_fusion.ipc.mlflow.artifacts import (
    DownloadArtifactResponse,
    ListArtifactsResponse,
    MlflowArtifactsServiceStub,
    UploadArtifact,
    UploadArtifactResponse,
)

T = TypeVar("T")


class AsyncMlflowArtifactsClient:
    def __init__(
        self,
        host: str = "localhost",
        port: int = 50051,
        use_ssl: bool = True,
        credential: TokenCredential | None = None,
        scopes: list[str] | None = None,
    ) -> None:
        """Asynchronous client for interacting with the Verbund services.

        Args:
            host (str, optional): server host name. Defaults to "localhost".
            port (int, optional): server port number. Defaults to 50051.
            use_ssl (bool, optional): use a secure channel for connection. Defaults to True.
        """
        self._host = host
        self._port = port
        self._use_ssl = use_ssl
        if credential is not None:
            self._auth = SendrequestAuth(credential=credential, scopes=scopes or [])

    @asynccontextmanager
    async def _service(self):
        async with Channel(host=self._host, port=self._port, ssl=self._use_ssl) as channel:
            yield MlflowArtifactsServiceStub(channel)

    async def list_artifacts(self, *, path: str | None = None) -> ListArtifactsResponse:
        """List all artifacts under given path.

        Returns:
            ListArtifactsResponse: Collection of all artifacts.
        """
        async with self._service() as service:
            return await service.list_artifacts(path=path)

    async def download_artifact(self, *, path: str = "") -> list[DownloadArtifactResponse]:
        chunks = []
        async with self._service() as service:
            async for chunk in service.download_artifact(path=path):
                chunks.append(chunk)
        return chunks

    async def upload_artifact(
        self,
        request_iterator: AsyncIterable[UploadArtifact] | Iterable[UploadArtifact],
    ) -> UploadArtifactResponse:
        async with self._service() as service:
            return await service.upload_artifact(request_iterator=request_iterator)
