from __future__ import annotations

import os
from typing import Iterator

from flight_fusion.ipc.mlflow import ListArtifactsResponse
from flight_fusion.ipc.mlflow.artifacts import DownloadArtifactResponse

from .._aio import run_async
from .aio import AsyncMlflowArtifactsClient

IS_WINDOWS = os.name == "nt"


class MlflowArtifactsClient:
    def __init__(
        self,
        host: str = "localhost",
        port: int = 50051,
        use_ssl: bool = True,
        # credential: Union[bool, TokenCredential] = True,
    ) -> None:
        """_summary_

        Args:
            host (str, optional): _description_. Defaults to "localhost".
            port (int, optional): _description_. Defaults to 50051.
            use_ssl (bool, optional): _description_. Defaults to True.
        """
        self._client = AsyncMlflowArtifactsClient(host=host, port=port, use_ssl=use_ssl)

    def list_artifacts(self, *, path: str | None = None) -> ListArtifactsResponse:
        return run_async(self._client.list_artifacts, path=path)  # type: ignore

    def download_artifact(self, *, path: str = "") -> Iterator[DownloadArtifactResponse]:
        return run_async(self._client.download_artifact, path=path)  # type: ignore
