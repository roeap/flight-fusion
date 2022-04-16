from __future__ import annotations

from contextlib import asynccontextmanager

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


class AsyncGrpcInferenceServiceClient:
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
            yield GrpcInferenceServiceStub(channel)

    async def server_live(self) -> bool:
        async with self._service() as service:
            response = await service.server_live()
        return response.ready

    async def server_ready(self) -> bool:
        async with self._service() as service:
            response = await service.server_ready()
        return response.ready

    async def model_ready(self, name: str, version: str = "") -> bool:
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
