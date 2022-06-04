from __future__ import annotations

import asyncio
from contextlib import asynccontextmanager
from threading import Thread
from typing import Any, AsyncIterable, Callable, Coroutine, Iterable, Iterator, TypeVar

from grpclib.client import Channel

from mlflow_fusion.auth import SendrequestAuth, TokenCredential
from mlflow_fusion.ipc.artifacts import (
    DownloadArtifactResponse,
    ListArtifactsResponse,
    MlflowArtifactsServiceStub,
    UploadArtifact,
    UploadArtifactResponse,
)

T = TypeVar("T")


class _RunThread(Thread):
    def __init__(self, func, args, kwargs):
        self.func = func
        self.args = args
        self.kwargs = kwargs
        super().__init__()

    def run(self):
        self.result = asyncio.run(self.func(*self.args, **self.kwargs))


def run_async(func: Callable[..., Coroutine[Any, Any, T]], *args, **kwargs) -> T:
    """Helper function to execute async code.

    Will use current loop if executed inside a running loop, will use asyncio.run otherwise.
    """
    try:
        loop = asyncio.get_running_loop()
    except RuntimeError:
        loop = None
    if loop and loop.is_running():
        thread = _RunThread(func, args, kwargs)
        thread.start()
        thread.join()
        return thread.result
    else:
        return asyncio.run(func(*args, **kwargs))


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
            host: server host name. Defaults to "localhost".
            port: server port number. Defaults to 50051.
            use_ssl: use a secure channel for connection. Defaults to True.
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


class MlflowArtifactsClient:
    def __init__(
        self,
        host: str = "localhost",
        port: int = 50051,
        use_ssl: bool = True,
        credential: TokenCredential | None = None,
        scopes: list[str] | None = None,
    ) -> None:
        """_summary_

        Args:
            host (str, optional): _description_. Defaults to "localhost".
            port (int, optional): _description_. Defaults to 50051.
            use_ssl (bool, optional): _description_. Defaults to True.
        """
        self._client = AsyncMlflowArtifactsClient(
            host=host, port=port, use_ssl=use_ssl, credential=credential, scopes=scopes
        )

    def list_artifacts(self, *, path: str | None = None) -> ListArtifactsResponse:
        return run_async(self._client.list_artifacts, path=path)  # type: ignore

    def download_artifact(self, *, path: str = "") -> Iterator[DownloadArtifactResponse]:
        return run_async(self._client.download_artifact, path=path)  # type: ignore

    def upload_artifact(
        self,
        request_iterator: AsyncIterable[UploadArtifact] | Iterable[UploadArtifact],
    ) -> UploadArtifactResponse:
        return run_async(self._client.upload_artifact, request_iterator=request_iterator)  # type: ignore
