from __future__ import annotations

import asyncio
import logging
import signal

from mlserver.batching import load_batching
from mlserver.grpc import GRPCServer
from mlserver.handlers import DataPlane, ModelRepositoryHandlers
from mlserver.kafka import KafkaServer
from mlserver.logging import configure_logger
from mlserver.metrics import MetricsServer
from mlserver.model import MLModel
from mlserver.parallel import InferencePool
from mlserver.registry import MultiModelRegistry
from mlserver.rest import RESTServer
from mlserver.settings import ModelSettings, Settings
from mlserver.utils import logger

from mlserver_fusion.repository import MlFlowRepository

HANDLED_SIGNALS = [signal.SIGINT, signal.SIGTERM]


class MLServer:
    def __init__(self, settings: Settings, mlflow_url: str):
        self._settings = settings
        self._inference_pool = None
        on_model_load = [
            self.add_custom_handlers,
            load_batching,
        ]
        on_model_reload = [self.reload_custom_handlers]
        on_model_unload = [self.remove_custom_handlers]

        if self._settings.parallel_workers:
            # Only load inference pool if parallel inference has been enabled
            self._inference_pool = InferencePool(self._settings)
            on_model_load = [
                self.add_custom_handlers,
                self._inference_pool.load_model,
                load_batching,
            ]
            on_model_reload = [
                self.reload_custom_handlers,
                self._inference_pool.reload_model,  # type: ignore
            ]
            on_model_unload = [
                self.remove_custom_handlers,
                self._inference_pool.unload_model,  # type: ignore
            ]

        self._model_registry = MultiModelRegistry(
            on_model_load=on_model_load,  # type: ignore
            on_model_reload=on_model_reload,  # type: ignore
            on_model_unload=on_model_unload,  # type: ignore
        )
        self._model_repository = MlFlowRepository(self._settings.model_repository_root)
        self._data_plane = DataPlane(settings=self._settings, model_registry=self._model_registry)
        self._model_repository_handlers = ModelRepositoryHandlers(
            repository=self._model_repository, model_registry=self._model_registry
        )

        logger.setLevel(logging.INFO)
        if self._settings.debug:
            logger.setLevel(logging.DEBUG)

        self._logger = configure_logger(settings)

        self._rest_server = RESTServer(self._settings, self._data_plane, self._model_repository_handlers)
        self._grpc_server = GRPCServer(self._settings, self._data_plane, self._model_repository_handlers)

        self._kafka_server = None
        if self._settings.kafka_enabled:
            self._kafka_server = KafkaServer(self._settings, self._data_plane)

        self._metrics_server = None
        if self._settings.metrics_endpoint:
            self._metrics_server = MetricsServer(self._settings)

    async def start(self, models_settings: list[ModelSettings] = []):
        self._add_signal_handlers()

        self._rest_server = RESTServer(self._settings, self._data_plane, self._model_repository_handlers)
        self._grpc_server = GRPCServer(self._settings, self._data_plane, self._model_repository_handlers)

        await asyncio.gather(*[self._model_registry.load(model_settings) for model_settings in models_settings])

        await asyncio.gather(self._rest_server.start(), self._grpc_server.start())

    async def add_custom_handlers(self, model: MLModel):
        await self._rest_server.add_custom_handlers(model)
        if self._kafka_server:
            await self._kafka_server.add_custom_handlers(model)

        # TODO: Add support for custom gRPC endpoints
        # self._grpc_server.add_custom_handlers(handlers)

    async def reload_custom_handlers(self, old_model: MLModel, new_model: MLModel):
        await self.add_custom_handlers(new_model)
        await self.remove_custom_handlers(old_model)

        # TODO: Add support for custom gRPC endpoints
        # self._grpc_server.delete_custom_handlers(handlers)

    async def remove_custom_handlers(self, model: MLModel):
        await self._rest_server.delete_custom_handlers(model)
        if self._kafka_server:
            await self._kafka_server.delete_custom_handlers(model)

        # TODO: Add support for custom gRPC endpoints
        # self._grpc_server.delete_custom_handlers(handlers)

    def _add_signal_handlers(self):
        loop = asyncio.get_event_loop()

        for sig in HANDLED_SIGNALS:
            loop.add_signal_handler(sig, lambda s=sig: asyncio.create_task(self.stop(sig=s)))

    async def stop(self, sig: int | None = None):
        if self._inference_pool:
            await self._inference_pool.close()

        if self._kafka_server:
            await self._kafka_server.stop()

        await self._grpc_server.stop(sig)  # type: ignore
        await self._rest_server.stop(sig)  # type: ignore

        if self._metrics_server:
            await self._metrics_server.stop(sig)  # type: ignore
