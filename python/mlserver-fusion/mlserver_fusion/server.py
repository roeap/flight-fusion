import asyncio
import signal
from typing import List

from mlserver.batching import load_batching
from mlserver.grpc import GRPCServer
from mlserver.handlers import DataPlane, ModelRepositoryHandlers
from mlserver.logging import configure_logger
from mlserver.model import MLModel
from mlserver.parallel import load_inference_pool, unload_inference_pool
from mlserver.registry import MultiModelRegistry

# from mlserver.repository import ModelRepository
from mlserver.rest import RESTServer
from mlserver.settings import ModelSettings, Settings

from mlserver_fusion.repository import MlFlowRepository

HANDLED_SIGNALS = [signal.SIGINT, signal.SIGTERM]


class MLServer:
    def __init__(self, settings: Settings, mlflow_url: str):
        self._settings = settings
        self._model_registry = MultiModelRegistry(
            on_model_load=[  # type: ignore
                self.add_custom_handlers,
                load_inference_pool,
                load_batching,
            ],
            on_model_unload=[self.remove_custom_handlers, unload_inference_pool],
        )

        self._model_repository = MlFlowRepository(mlflow_url)
        self._data_plane = DataPlane(settings=self._settings, model_registry=self._model_registry)
        self._model_repository_handlers = ModelRepositoryHandlers(
            repository=self._model_repository, model_registry=self._model_registry
        )

        self._logger = configure_logger(settings)

    async def start(self, models_settings: List[ModelSettings] = []):
        self._add_signal_handlers()

        self._rest_server = RESTServer(self._settings, self._data_plane, self._model_repository_handlers)
        self._grpc_server = GRPCServer(self._settings, self._data_plane, self._model_repository_handlers)

        await asyncio.gather(*[self._model_registry.load(model_settings) for model_settings in models_settings])

        await asyncio.gather(self._rest_server.start(), self._grpc_server.start())

    async def add_custom_handlers(self, model: MLModel):
        await self._rest_server.add_custom_handlers(model)

        # TODO: Add support for custom gRPC endpoints
        # self._grpc_server.add_custom_handlers(handlers)

    async def remove_custom_handlers(self, model: MLModel):
        await self._rest_server.delete_custom_handlers(model)

        # TODO: Add support for custom gRPC endpoints
        # self._grpc_server.delete_custom_handlers(handlers)

    def _add_signal_handlers(self):
        loop = asyncio.get_event_loop()

        for sign in HANDLED_SIGNALS:
            loop.add_signal_handler(sign, lambda: asyncio.ensure_future(self.stop()))

    async def stop(self):
        await self._rest_server.stop()
        await self._grpc_server.stop()
