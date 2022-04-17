from __future__ import annotations

import os

from flight_fusion.ipc.inference import (
    InferParameter,
    ModelInferRequestInferInputTensor,
    ModelInferRequestInferRequestedOutputTensor,
    ModelInferResponse,
    ModelMetadataResponse,
    ServerMetadataResponse,
)

from .aio import AsyncGrpcInferenceServiceClient, run_async

IS_WINDOWS = os.name == "nt"


class GrpcInferenceServiceClient:
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
        self._client = AsyncGrpcInferenceServiceClient(
            host=host,
            port=port,
            use_ssl=use_ssl,  # credential=credential
        )

    def server_live(self) -> bool:
        return run_async(self._client.server_live)

    def server_ready(self) -> bool:
        return run_async(self._client.server_ready)

    def model_ready(self, name: str, version: str = "") -> bool:
        return run_async(self._client.model_ready, name=name, version=version)

    def server_metadata(self) -> ServerMetadataResponse:
        return run_async(self._client.server_metadata)

    def model_metadata(self, name: str, version: str = "") -> ModelMetadataResponse:
        return run_async(self._client.model_metadata, name=name, version=version)

    def model_infer(
        self,
        *,
        model_name: str,
        model_version: str = "",
        id: str = "",
        parameters: dict[str, InferParameter] | None = None,
        inputs: list[ModelInferRequestInferInputTensor] | None = None,
        outputs: list[ModelInferRequestInferRequestedOutputTensor] | None = None,
    ) -> ModelInferResponse:
        return run_async(
            self._client.model_infer,
            model_name=model_name,
            model_version=model_version,
            id=id,
            parameters=parameters,
            inputs=inputs,
            outputs=outputs,
        )
