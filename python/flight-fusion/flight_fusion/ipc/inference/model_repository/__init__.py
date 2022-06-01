# Generated by the protocol buffer compiler.  DO NOT EDIT!
# sources: inference/model_repository.proto
# plugin: python-betterproto
from dataclasses import dataclass
from typing import Dict, List

import betterproto
import grpclib
from betterproto.grpc.grpclib_server import ServiceBase


@dataclass(eq=False, repr=False)
class RepositoryIndexRequest(betterproto.Message):
    # The name of the repository. If empty the index is returned for all
    # repositories.
    repository_name: str = betterproto.string_field(1)
    # If true return only models currently ready for inferencing.
    ready: bool = betterproto.bool_field(2)


@dataclass(eq=False, repr=False)
class RepositoryIndexResponse(betterproto.Message):
    # An index entry for each model.
    models: List["RepositoryIndexResponseModelIndex"] = betterproto.message_field(1)


@dataclass(eq=False, repr=False)
class RepositoryIndexResponseModelIndex(betterproto.Message):
    """Index entry for a model."""

    # The name of the model.
    name: str = betterproto.string_field(1)
    # The version of the model.
    version: str = betterproto.string_field(2)
    # The state of the model.
    state: str = betterproto.string_field(3)
    # The reason, if any, that the model is in the given state.
    reason: str = betterproto.string_field(4)


@dataclass(eq=False, repr=False)
class RepositoryModelLoadRequest(betterproto.Message):
    # The name of the repository to load from. If empty the model is loaded from
    # any repository.
    repository_name: str = betterproto.string_field(1)
    # The name of the model to load, or reload.
    model_name: str = betterproto.string_field(2)


@dataclass(eq=False, repr=False)
class RepositoryModelLoadResponse(betterproto.Message):
    pass


@dataclass(eq=False, repr=False)
class RepositoryModelUnloadRequest(betterproto.Message):
    # The name of the repository from which the model was originally loaded. If
    # empty the repository is not considered.
    repository_name: str = betterproto.string_field(1)
    # The name of the model to unload.
    model_name: str = betterproto.string_field(2)


@dataclass(eq=False, repr=False)
class RepositoryModelUnloadResponse(betterproto.Message):
    pass


class ModelRepositoryServiceStub(betterproto.ServiceStub):
    async def repository_index(self, *, repository_name: str = "", ready: bool = False) -> "RepositoryIndexResponse":

        request = RepositoryIndexRequest()
        request.repository_name = repository_name
        request.ready = ready

        return await self._unary_unary(
            "/inference.model_repository.ModelRepositoryService/RepositoryIndex",
            request,
            RepositoryIndexResponse,
        )

    async def repository_model_load(
        self, *, repository_name: str = "", model_name: str = ""
    ) -> "RepositoryModelLoadResponse":

        request = RepositoryModelLoadRequest()
        request.repository_name = repository_name
        request.model_name = model_name

        return await self._unary_unary(
            "/inference.model_repository.ModelRepositoryService/RepositoryModelLoad",
            request,
            RepositoryModelLoadResponse,
        )

    async def repository_model_unload(
        self, *, repository_name: str = "", model_name: str = ""
    ) -> "RepositoryModelUnloadResponse":

        request = RepositoryModelUnloadRequest()
        request.repository_name = repository_name
        request.model_name = model_name

        return await self._unary_unary(
            "/inference.model_repository.ModelRepositoryService/RepositoryModelUnload",
            request,
            RepositoryModelUnloadResponse,
        )


class ModelRepositoryServiceBase(ServiceBase):
    async def repository_index(self, repository_name: str, ready: bool) -> "RepositoryIndexResponse":
        raise grpclib.GRPCError(grpclib.const.Status.UNIMPLEMENTED)

    async def repository_model_load(self, repository_name: str, model_name: str) -> "RepositoryModelLoadResponse":
        raise grpclib.GRPCError(grpclib.const.Status.UNIMPLEMENTED)

    async def repository_model_unload(self, repository_name: str, model_name: str) -> "RepositoryModelUnloadResponse":
        raise grpclib.GRPCError(grpclib.const.Status.UNIMPLEMENTED)

    async def __rpc_repository_index(self, stream: grpclib.server.Stream) -> None:
        request = await stream.recv_message()

        request_kwargs = {
            "repository_name": request.repository_name,
            "ready": request.ready,
        }

        response = await self.repository_index(**request_kwargs)
        await stream.send_message(response)

    async def __rpc_repository_model_load(self, stream: grpclib.server.Stream) -> None:
        request = await stream.recv_message()

        request_kwargs = {
            "repository_name": request.repository_name,
            "model_name": request.model_name,
        }

        response = await self.repository_model_load(**request_kwargs)
        await stream.send_message(response)

    async def __rpc_repository_model_unload(self, stream: grpclib.server.Stream) -> None:
        request = await stream.recv_message()

        request_kwargs = {
            "repository_name": request.repository_name,
            "model_name": request.model_name,
        }

        response = await self.repository_model_unload(**request_kwargs)
        await stream.send_message(response)

    def __mapping__(self) -> Dict[str, grpclib.const.Handler]:
        return {
            "/inference.model_repository.ModelRepositoryService/RepositoryIndex": grpclib.const.Handler(
                self.__rpc_repository_index,
                grpclib.const.Cardinality.UNARY_UNARY,
                RepositoryIndexRequest,
                RepositoryIndexResponse,
            ),
            "/inference.model_repository.ModelRepositoryService/RepositoryModelLoad": grpclib.const.Handler(
                self.__rpc_repository_model_load,
                grpclib.const.Cardinality.UNARY_UNARY,
                RepositoryModelLoadRequest,
                RepositoryModelLoadResponse,
            ),
            "/inference.model_repository.ModelRepositoryService/RepositoryModelUnload": grpclib.const.Handler(
                self.__rpc_repository_model_unload,
                grpclib.const.Cardinality.UNARY_UNARY,
                RepositoryModelUnloadRequest,
                RepositoryModelUnloadResponse,
            ),
        }
