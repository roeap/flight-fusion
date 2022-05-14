from __future__ import annotations

from contextlib import asynccontextmanager
from typing import AsyncGenerator, Generic, Type, TypeVar

from betterproto import ServiceStub
from grpclib.client import Channel
from grpclib.events import SendRequest, listen

from flight_fusion.ipc.inference import (
    GrpcInferenceServiceStub,
    InferParameter,
    ModelInferRequestInferInputTensor,
    ModelInferRequestInferRequestedOutputTensor,
    ModelInferResponse,
    ModelMetadataResponse,
    ServerMetadataResponse,
)
from flight_fusion.ipc.inference.model_repository import (
    ModelRepositoryServiceStub,
    RepositoryIndexResponse,
    RepositoryModelLoadResponse,
    RepositoryModelUnloadResponse,
)

T = TypeVar("T", bound=ServiceStub)


class _BaseGrpcClient(Generic[T]):
    def __init__(
        self,
        stub_type: Type[T],
        host: str = "localhost",
        port: int = 8081,
        use_ssl: bool = False,
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
        self.stub = stub_type

    @asynccontextmanager
    async def _service(self) -> AsyncGenerator[T, None]:
        async with Channel(host=self._host, port=self._port, ssl=self._use_ssl) as channel:
            if self._auth is not None:
                listen(channel, SendRequest, self._auth)
            yield self.stub(channel)


class AsyncGrpcModelRepositoryServiceClient(_BaseGrpcClient[ModelRepositoryServiceStub]):
    def __init__(
        self,
        host: str = "localhost",
        port: int = 8081,
        use_ssl: bool = False,
    ) -> None:
        """Asynchronous client for interacting with the Verbund services.

        Args:
            host (str, optional): server host name. Defaults to "localhost".
            port (int, optional): server port number. Defaults to 8081.
            use_ssl (bool, optional): use a secure channe for connection. Defaults to True.
        """
        super().__init__(
            stub_type=ModelRepositoryServiceStub, host=host, port=port, use_ssl=use_ssl
        )

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


class AsyncGrpcInferenceServiceClient(_BaseGrpcClient[GrpcInferenceServiceStub]):
    def __init__(
        self,
        host: str = "localhost",
        port: int = 8081,
        use_ssl: bool = False,
    ) -> None:
        """Asynchronous client for interacting with the Verbund services.

        Args:
            host (str, optional): server host name. Defaults to "localhost".
            port (int, optional): server port number. Defaults to 8081.
            use_ssl (bool, optional): use a secure channe for connection. Defaults to True.
        """
        super().__init__(stub_type=GrpcInferenceServiceStub, host=host, port=port, use_ssl=use_ssl)

    async def server_live(self) -> bool:
        async with self._service() as service:
            response = await service.server_live()
        return response.ready

    async def server_ready(self) -> bool:
        async with self._service() as service:
            response = await service.server_ready()
        return response.ready

    async def model_ready(self, *, name: str, version: str = "") -> bool:
        async with self._service() as service:
            response = await service.model_ready(name=name, version=version)
        return response.ready

    async def server_metadata(self) -> ServerMetadataResponse:
        async with self._service() as service:
            return await service.server_metadata()

    async def model_metadata(self, *, name: str, version: str = "") -> ModelMetadataResponse:
        async with self._service() as service:
            return await service.model_metadata(name=name, version=version)

    async def model_infer(
        self,
        *,
        model_name: str,
        model_version: str = "",
        id: str = "",
        parameters: dict[str, InferParameter] | None = None,
        inputs: list[ModelInferRequestInferInputTensor] | None = None,
        outputs: list[ModelInferRequestInferRequestedOutputTensor] | None = None,
    ) -> ModelInferResponse:
        async with self._service() as service:
            return await service.model_infer(
                model_name=model_name,
                model_version=model_version,
                id=id,
                parameters=parameters or {},
                inputs=inputs,
                outputs=outputs,
            )
