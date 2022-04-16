import os

from .aio.inference import AsyncServiceClient, run_async

IS_WINDOWS = os.name == "nt"


class ServiceClient:
    def __init__(
        self,
        host: str = "localhost",
        port: int = 8081,
        use_ssl: bool = True,
        # credential: Union[bool, TokenCredential] = True,
    ) -> None:
        """_summary_

        Args:
            host (str, optional): _description_. Defaults to "localhost".
            port (int, optional): _description_. Defaults to 8081.
            use_ssl (bool, optional): _description_. Defaults to True.
        """
        self._client = AsyncServiceClient(
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
