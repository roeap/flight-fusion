# Generated by the protocol buffer compiler.  DO NOT EDIT!
# sources: inference/dataplane.proto
# plugin: python-betterproto
from dataclasses import dataclass
from typing import Dict, List, Optional

import betterproto
import grpclib
from betterproto.grpc.grpclib_server import ServiceBase


@dataclass(eq=False, repr=False)
class ServerLiveRequest(betterproto.Message):
    """ServerLive messages."""

    pass


@dataclass(eq=False, repr=False)
class ServerLiveResponse(betterproto.Message):
    # True if the inference server is live, false if not live.
    live: bool = betterproto.bool_field(1)


@dataclass(eq=False, repr=False)
class ServerReadyRequest(betterproto.Message):
    """ServerReady messages."""

    pass


@dataclass(eq=False, repr=False)
class ServerReadyResponse(betterproto.Message):
    # True if the inference server is ready, false if not ready.
    ready: bool = betterproto.bool_field(1)


@dataclass(eq=False, repr=False)
class ModelReadyRequest(betterproto.Message):
    """ModelReady messages."""

    # The name of the model to check for readiness.
    name: str = betterproto.string_field(1)
    # The version of the model to check for readiness. If not given the server
    # will choose a version based on the model and internal policy.
    version: str = betterproto.string_field(2)


@dataclass(eq=False, repr=False)
class ModelReadyResponse(betterproto.Message):
    # True if the model is ready, false if not ready.
    ready: bool = betterproto.bool_field(1)


@dataclass(eq=False, repr=False)
class ServerMetadataRequest(betterproto.Message):
    """ServerMetadata messages."""

    pass


@dataclass(eq=False, repr=False)
class ServerMetadataResponse(betterproto.Message):
    # The server name.
    name: str = betterproto.string_field(1)
    # The server version.
    version: str = betterproto.string_field(2)
    # The extensions supported by the server.
    extensions: List[str] = betterproto.string_field(3)


@dataclass(eq=False, repr=False)
class ModelMetadataRequest(betterproto.Message):
    """ModelMetadata messages."""

    # The name of the model.
    name: str = betterproto.string_field(1)
    # The version of the model to check for readiness. If not given the server
    # will choose a version based on the model and internal policy.
    version: str = betterproto.string_field(2)


@dataclass(eq=False, repr=False)
class ModelMetadataResponse(betterproto.Message):
    # The model name.
    name: str = betterproto.string_field(1)
    # The versions of the model available on the server.
    versions: List[str] = betterproto.string_field(2)
    # The model's platform. See Platforms.
    platform: str = betterproto.string_field(3)
    # The model's inputs.
    inputs: List["ModelMetadataResponseTensorMetadata"] = betterproto.message_field(4)
    # The model's outputs.
    outputs: List["ModelMetadataResponseTensorMetadata"] = betterproto.message_field(5)
    # Optional default parameters for the request / response. NOTE: This is an
    # extension to the standard
    parameters: Dict[str, "InferParameter"] = betterproto.map_field(
        6, betterproto.TYPE_STRING, betterproto.TYPE_MESSAGE
    )


@dataclass(eq=False, repr=False)
class ModelMetadataResponseTensorMetadata(betterproto.Message):
    """Metadata for a tensor."""

    # The tensor name.
    name: str = betterproto.string_field(1)
    # The tensor data type.
    datatype: str = betterproto.string_field(2)
    # The tensor shape. A variable-size dimension is represented by a -1 value.
    shape: List[int] = betterproto.int64_field(3)
    # Optional default parameters for input. NOTE: This is an extension to the
    # standard
    parameters: Dict[str, "InferParameter"] = betterproto.map_field(
        4, betterproto.TYPE_STRING, betterproto.TYPE_MESSAGE
    )


@dataclass(eq=False, repr=False)
class ModelInferRequest(betterproto.Message):
    """ModelInfer messages."""

    # The name of the model to use for inferencing.
    model_name: str = betterproto.string_field(1)
    # The version of the model to use for inference. If not given the server will
    # choose a version based on the model and internal policy.
    model_version: str = betterproto.string_field(2)
    # Optional identifier for the request. If specified will be returned in the
    # response.
    id: str = betterproto.string_field(3)
    # Optional inference parameters.
    parameters: Dict[str, "InferParameter"] = betterproto.map_field(
        4, betterproto.TYPE_STRING, betterproto.TYPE_MESSAGE
    )
    # The input tensors for the inference.
    inputs: List["ModelInferRequestInferInputTensor"] = betterproto.message_field(5)
    # The requested output tensors for the inference. Optional, if not specified
    # all outputs produced by the model will be returned.
    outputs: List["ModelInferRequestInferRequestedOutputTensor"] = betterproto.message_field(6)


@dataclass(eq=False, repr=False)
class ModelInferRequestInferInputTensor(betterproto.Message):
    """An input tensor for an inference request."""

    # The tensor name.
    name: str = betterproto.string_field(1)
    # The tensor data type.
    datatype: str = betterproto.string_field(2)
    # The tensor shape.
    shape: List[int] = betterproto.int64_field(3)
    # Optional inference input tensor parameters.
    parameters: Dict[str, "InferParameter"] = betterproto.map_field(
        4, betterproto.TYPE_STRING, betterproto.TYPE_MESSAGE
    )
    # The input tensor data.
    contents: "InferTensorContents" = betterproto.message_field(5)


@dataclass(eq=False, repr=False)
class ModelInferRequestInferRequestedOutputTensor(betterproto.Message):
    """An output tensor requested for an inference request."""

    # The tensor name.
    name: str = betterproto.string_field(1)
    # Optional requested output tensor parameters.
    parameters: Dict[str, "InferParameter"] = betterproto.map_field(
        2, betterproto.TYPE_STRING, betterproto.TYPE_MESSAGE
    )


@dataclass(eq=False, repr=False)
class ModelInferResponse(betterproto.Message):
    # The name of the model used for inference.
    model_name: str = betterproto.string_field(1)
    # The version of the model used for inference.
    model_version: str = betterproto.string_field(2)
    # The id of the inference request if one was specified.
    id: str = betterproto.string_field(3)
    # Optional inference response parameters.
    parameters: Dict[str, "InferParameter"] = betterproto.map_field(
        4, betterproto.TYPE_STRING, betterproto.TYPE_MESSAGE
    )
    # The output tensors holding inference results.
    outputs: List["ModelInferResponseInferOutputTensor"] = betterproto.message_field(5)


@dataclass(eq=False, repr=False)
class ModelInferResponseInferOutputTensor(betterproto.Message):
    """An output tensor returned for an inference request."""

    # The tensor name.
    name: str = betterproto.string_field(1)
    # The tensor data type.
    datatype: str = betterproto.string_field(2)
    # The tensor shape.
    shape: List[int] = betterproto.int64_field(3)
    # Optional output tensor parameters.
    parameters: Dict[str, "InferParameter"] = betterproto.map_field(
        4, betterproto.TYPE_STRING, betterproto.TYPE_MESSAGE
    )
    # The output tensor data.
    contents: "InferTensorContents" = betterproto.message_field(5)


@dataclass(eq=False, repr=False)
class InferParameter(betterproto.Message):
    """An inference parameter value."""

    # A boolean parameter value.
    bool_param: bool = betterproto.bool_field(1, group="parameter_choice")
    # An int64 parameter value.
    int64_param: int = betterproto.int64_field(2, group="parameter_choice")
    # A string parameter value.
    string_param: str = betterproto.string_field(3, group="parameter_choice")


@dataclass(eq=False, repr=False)
class InferTensorContents(betterproto.Message):
    """
    The data contained in a tensor. For a given data type the tensor contents
    can be represented in "raw" bytes form or in the repeated type that matches
    the tensor's data type. Protobuf oneof is not used because oneofs cannot
    contain repeated fields.
    """

    # Representation for BOOL data type. The size must match what is expected by
    # the tensor's shape. The contents must be the flattened, one-dimensional,
    # row-major order of the tensor elements.
    bool_contents: List[bool] = betterproto.bool_field(1)
    # Representation for INT8, INT16, and INT32 data types. The size must match
    # what is expected by the tensor's shape. The contents must be the flattened,
    # one-dimensional, row-major order of the tensor elements.
    int_contents: List[int] = betterproto.int32_field(2)
    # Representation for INT64 data types. The size must match what is expected
    # by the tensor's shape. The contents must be the flattened, one-dimensional,
    # row-major order of the tensor elements.
    int64_contents: List[int] = betterproto.int64_field(3)
    # Representation for UINT8, UINT16, and UINT32 data types. The size must
    # match what is expected by the tensor's shape. The contents must be the
    # flattened, one-dimensional, row-major order of the tensor elements.
    uint_contents: List[int] = betterproto.uint32_field(4)
    # Representation for UINT64 data types. The size must match what is expected
    # by the tensor's shape. The contents must be the flattened, one-dimensional,
    # row-major order of the tensor elements.
    uint64_contents: List[int] = betterproto.uint64_field(5)
    # Representation for FP32 data type. The size must match what is expected by
    # the tensor's shape. The contents must be the flattened, one-dimensional,
    # row-major order of the tensor elements.
    fp32_contents: List[float] = betterproto.float_field(6)
    # Representation for FP64 data type. The size must match what is expected by
    # the tensor's shape. The contents must be the flattened, one-dimensional,
    # row-major order of the tensor elements.
    fp64_contents: List[float] = betterproto.double_field(7)
    # Representation for BYTES data type. The size must match what is expected by
    # the tensor's shape. The contents must be the flattened, one-dimensional,
    # row-major order of the tensor elements.
    bytes_contents: List[bytes] = betterproto.bytes_field(8)


class GrpcInferenceServiceStub(betterproto.ServiceStub):
    async def server_live(self) -> "ServerLiveResponse":

        request = ServerLiveRequest()

        return await self._unary_unary("/inference.GRPCInferenceService/ServerLive", request, ServerLiveResponse)

    async def server_ready(self) -> "ServerReadyResponse":

        request = ServerReadyRequest()

        return await self._unary_unary("/inference.GRPCInferenceService/ServerReady", request, ServerReadyResponse)

    async def model_ready(self, *, name: str = "", version: str = "") -> "ModelReadyResponse":

        request = ModelReadyRequest()
        request.name = name
        request.version = version

        return await self._unary_unary("/inference.GRPCInferenceService/ModelReady", request, ModelReadyResponse)

    async def server_metadata(self) -> "ServerMetadataResponse":

        request = ServerMetadataRequest()

        return await self._unary_unary(
            "/inference.GRPCInferenceService/ServerMetadata",
            request,
            ServerMetadataResponse,
        )

    async def model_metadata(self, *, name: str = "", version: str = "") -> "ModelMetadataResponse":

        request = ModelMetadataRequest()
        request.name = name
        request.version = version

        return await self._unary_unary(
            "/inference.GRPCInferenceService/ModelMetadata",
            request,
            ModelMetadataResponse,
        )

    async def model_infer(
        self,
        *,
        model_name: str = "",
        model_version: str = "",
        id: str = "",
        parameters: Dict[str, "InferParameter"] = None,
        inputs: Optional[List["ModelInferRequestInferInputTensor"]] = None,
        outputs: Optional[List["ModelInferRequestInferRequestedOutputTensor"]] = None,
    ) -> "ModelInferResponse":
        inputs = inputs or []
        outputs = outputs or []

        request = ModelInferRequest()
        request.model_name = model_name
        request.model_version = model_version
        request.id = id
        request.parameters = parameters
        if inputs is not None:
            request.inputs = inputs
        if outputs is not None:
            request.outputs = outputs

        return await self._unary_unary("/inference.GRPCInferenceService/ModelInfer", request, ModelInferResponse)


class GrpcInferenceServiceBase(ServiceBase):
    async def server_live(self) -> "ServerLiveResponse":
        raise grpclib.GRPCError(grpclib.const.Status.UNIMPLEMENTED)

    async def server_ready(self) -> "ServerReadyResponse":
        raise grpclib.GRPCError(grpclib.const.Status.UNIMPLEMENTED)

    async def model_ready(self, name: str, version: str) -> "ModelReadyResponse":
        raise grpclib.GRPCError(grpclib.const.Status.UNIMPLEMENTED)

    async def server_metadata(self) -> "ServerMetadataResponse":
        raise grpclib.GRPCError(grpclib.const.Status.UNIMPLEMENTED)

    async def model_metadata(self, name: str, version: str) -> "ModelMetadataResponse":
        raise grpclib.GRPCError(grpclib.const.Status.UNIMPLEMENTED)

    async def model_infer(
        self,
        model_name: str,
        model_version: str,
        id: str,
        parameters: Dict[str, "InferParameter"],
        inputs: Optional[List["ModelInferRequestInferInputTensor"]],
        outputs: Optional[List["ModelInferRequestInferRequestedOutputTensor"]],
    ) -> "ModelInferResponse":
        raise grpclib.GRPCError(grpclib.const.Status.UNIMPLEMENTED)

    async def __rpc_server_live(self, stream: grpclib.server.Stream) -> None:
        request = await stream.recv_message()  # noqa

        request_kwargs = {}

        response = await self.server_live(**request_kwargs)
        await stream.send_message(response)

    async def __rpc_server_ready(self, stream: grpclib.server.Stream) -> None:
        request = await stream.recv_message()  # noqa

        request_kwargs = {}

        response = await self.server_ready(**request_kwargs)
        await stream.send_message(response)

    async def __rpc_model_ready(self, stream: grpclib.server.Stream) -> None:
        request = await stream.recv_message()  # noqa

        request_kwargs = {
            "name": request.name,
            "version": request.version,
        }

        response = await self.model_ready(**request_kwargs)
        await stream.send_message(response)

    async def __rpc_server_metadata(self, stream: grpclib.server.Stream) -> None:
        request = await stream.recv_message()  # noqa

        request_kwargs = {}

        response = await self.server_metadata(**request_kwargs)
        await stream.send_message(response)

    async def __rpc_model_metadata(self, stream: grpclib.server.Stream) -> None:
        request = await stream.recv_message()

        request_kwargs = {
            "name": request.name,
            "version": request.version,
        }

        response = await self.model_metadata(**request_kwargs)
        await stream.send_message(response)

    async def __rpc_model_infer(self, stream: grpclib.server.Stream) -> None:
        request = await stream.recv_message()

        request_kwargs = {
            "model_name": request.model_name,
            "model_version": request.model_version,
            "id": request.id,
            "parameters": request.parameters,
            "inputs": request.inputs,
            "outputs": request.outputs,
        }

        response = await self.model_infer(**request_kwargs)
        await stream.send_message(response)

    def __mapping__(self) -> Dict[str, grpclib.const.Handler]:
        return {
            "/inference.GRPCInferenceService/ServerLive": grpclib.const.Handler(
                self.__rpc_server_live,
                grpclib.const.Cardinality.UNARY_UNARY,
                ServerLiveRequest,
                ServerLiveResponse,
            ),
            "/inference.GRPCInferenceService/ServerReady": grpclib.const.Handler(
                self.__rpc_server_ready,
                grpclib.const.Cardinality.UNARY_UNARY,
                ServerReadyRequest,
                ServerReadyResponse,
            ),
            "/inference.GRPCInferenceService/ModelReady": grpclib.const.Handler(
                self.__rpc_model_ready,
                grpclib.const.Cardinality.UNARY_UNARY,
                ModelReadyRequest,
                ModelReadyResponse,
            ),
            "/inference.GRPCInferenceService/ServerMetadata": grpclib.const.Handler(
                self.__rpc_server_metadata,
                grpclib.const.Cardinality.UNARY_UNARY,
                ServerMetadataRequest,
                ServerMetadataResponse,
            ),
            "/inference.GRPCInferenceService/ModelMetadata": grpclib.const.Handler(
                self.__rpc_model_metadata,
                grpclib.const.Cardinality.UNARY_UNARY,
                ModelMetadataRequest,
                ModelMetadataResponse,
            ),
            "/inference.GRPCInferenceService/ModelInfer": grpclib.const.Handler(
                self.__rpc_model_infer,
                grpclib.const.Cardinality.UNARY_UNARY,
                ModelInferRequest,
                ModelInferResponse,
            ),
        }
