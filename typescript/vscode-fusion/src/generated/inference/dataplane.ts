/* eslint-disable */
import * as Long from "long";
import * as _m0 from "protobufjs/minimal";

export const protobufPackage = "inference";

/** ServerLive messages. */
export interface ServerLiveRequest {}

export interface ServerLiveResponse {
  /** True if the inference server is live, false if not live. */
  live: boolean;
}

/** ServerReady messages. */
export interface ServerReadyRequest {}

export interface ServerReadyResponse {
  /** True if the inference server is ready, false if not ready. */
  ready: boolean;
}

/** ModelReady messages. */
export interface ModelReadyRequest {
  /** The name of the model to check for readiness. */
  name: string;
  /**
   * The version of the model to check for readiness. If not given the
   * server will choose a version based on the model and internal policy.
   */
  version: string;
}

export interface ModelReadyResponse {
  /** True if the model is ready, false if not ready. */
  ready: boolean;
}

/** ServerMetadata messages. */
export interface ServerMetadataRequest {}

export interface ServerMetadataResponse {
  /** The server name. */
  name: string;
  /** The server version. */
  version: string;
  /** The extensions supported by the server. */
  extensions: string[];
}

/** ModelMetadata messages. */
export interface ModelMetadataRequest {
  /** The name of the model. */
  name: string;
  /**
   * The version of the model to check for readiness. If not given the
   * server will choose a version based on the model and internal policy.
   */
  version: string;
}

export interface ModelMetadataResponse {
  /** The model name. */
  name: string;
  /** The versions of the model available on the server. */
  versions: string[];
  /** The model's platform. See Platforms. */
  platform: string;
  /** The model's inputs. */
  inputs: ModelMetadataResponse_TensorMetadata[];
  /** The model's outputs. */
  outputs: ModelMetadataResponse_TensorMetadata[];
  /**
   * Optional default parameters for the request / response.
   * NOTE: This is an extension to the standard
   */
  parameters: { [key: string]: InferParameter };
}

/** Metadata for a tensor. */
export interface ModelMetadataResponse_TensorMetadata {
  /** The tensor name. */
  name: string;
  /** The tensor data type. */
  datatype: string;
  /**
   * The tensor shape. A variable-size dimension is represented
   * by a -1 value.
   */
  shape: number[];
  /**
   * Optional default parameters for input.
   * NOTE: This is an extension to the standard
   */
  parameters: { [key: string]: InferParameter };
}

export interface ModelMetadataResponse_TensorMetadata_ParametersEntry {
  key: string;
  value: InferParameter | undefined;
}

export interface ModelMetadataResponse_ParametersEntry {
  key: string;
  value: InferParameter | undefined;
}

/** ModelInfer messages. */
export interface ModelInferRequest {
  /** The name of the model to use for inferencing. */
  modelName: string;
  /**
   * The version of the model to use for inference. If not given the
   * server will choose a version based on the model and internal policy.
   */
  modelVersion: string;
  /**
   * Optional identifier for the request. If specified will be
   * returned in the response.
   */
  id: string;
  /** Optional inference parameters. */
  parameters: { [key: string]: InferParameter };
  /** The input tensors for the inference. */
  inputs: ModelInferRequest_InferInputTensor[];
  /**
   * The requested output tensors for the inference. Optional, if not
   * specified all outputs produced by the model will be returned.
   */
  outputs: ModelInferRequest_InferRequestedOutputTensor[];
}

/** An input tensor for an inference request. */
export interface ModelInferRequest_InferInputTensor {
  /** The tensor name. */
  name: string;
  /** The tensor data type. */
  datatype: string;
  /** The tensor shape. */
  shape: number[];
  /** Optional inference input tensor parameters. */
  parameters: { [key: string]: InferParameter };
  /** The input tensor data. */
  contents: InferTensorContents | undefined;
}

export interface ModelInferRequest_InferInputTensor_ParametersEntry {
  key: string;
  value: InferParameter | undefined;
}

/** An output tensor requested for an inference request. */
export interface ModelInferRequest_InferRequestedOutputTensor {
  /** The tensor name. */
  name: string;
  /** Optional requested output tensor parameters. */
  parameters: { [key: string]: InferParameter };
}

export interface ModelInferRequest_InferRequestedOutputTensor_ParametersEntry {
  key: string;
  value: InferParameter | undefined;
}

export interface ModelInferRequest_ParametersEntry {
  key: string;
  value: InferParameter | undefined;
}

export interface ModelInferResponse {
  /** The name of the model used for inference. */
  modelName: string;
  /** The version of the model used for inference. */
  modelVersion: string;
  /** The id of the inference request if one was specified. */
  id: string;
  /** Optional inference response parameters. */
  parameters: { [key: string]: InferParameter };
  /** The output tensors holding inference results. */
  outputs: ModelInferResponse_InferOutputTensor[];
}

/** An output tensor returned for an inference request. */
export interface ModelInferResponse_InferOutputTensor {
  /** The tensor name. */
  name: string;
  /** The tensor data type. */
  datatype: string;
  /** The tensor shape. */
  shape: number[];
  /** Optional output tensor parameters. */
  parameters: { [key: string]: InferParameter };
  /** The output tensor data. */
  contents: InferTensorContents | undefined;
}

export interface ModelInferResponse_InferOutputTensor_ParametersEntry {
  key: string;
  value: InferParameter | undefined;
}

export interface ModelInferResponse_ParametersEntry {
  key: string;
  value: InferParameter | undefined;
}

/** An inference parameter value. */
export interface InferParameter {
  /** A boolean parameter value. */
  boolParam: boolean | undefined;
  /** An int64 parameter value. */
  int64Param: number | undefined;
  /** A string parameter value. */
  stringParam: string | undefined;
}

/**
 * The data contained in a tensor. For a given data type the
 * tensor contents can be represented in "raw" bytes form or in
 * the repeated type that matches the tensor's data type. Protobuf
 * oneof is not used because oneofs cannot contain repeated fields.
 */
export interface InferTensorContents {
  /**
   * Representation for BOOL data type. The size must match what is
   * expected by the tensor's shape. The contents must be the flattened,
   * one-dimensional, row-major order of the tensor elements.
   */
  boolContents: boolean[];
  /**
   * Representation for INT8, INT16, and INT32 data types. The size
   * must match what is expected by the tensor's shape. The contents
   * must be the flattened, one-dimensional, row-major order of the
   * tensor elements.
   */
  intContents: number[];
  /**
   * Representation for INT64 data types. The size must match what
   * is expected by the tensor's shape. The contents must be the
   * flattened, one-dimensional, row-major order of the tensor elements.
   */
  int64Contents: number[];
  /**
   * Representation for UINT8, UINT16, and UINT32 data types. The size
   * must match what is expected by the tensor's shape. The contents
   * must be the flattened, one-dimensional, row-major order of the
   * tensor elements.
   */
  uintContents: number[];
  /**
   * Representation for UINT64 data types. The size must match what
   * is expected by the tensor's shape. The contents must be the
   * flattened, one-dimensional, row-major order of the tensor elements.
   */
  uint64Contents: number[];
  /**
   * Representation for FP32 data type. The size must match what is
   * expected by the tensor's shape. The contents must be the flattened,
   * one-dimensional, row-major order of the tensor elements.
   */
  fp32Contents: number[];
  /**
   * Representation for FP64 data type. The size must match what is
   * expected by the tensor's shape. The contents must be the flattened,
   * one-dimensional, row-major order of the tensor elements.
   */
  fp64Contents: number[];
  /**
   * Representation for BYTES data type. The size must match what is
   * expected by the tensor's shape. The contents must be the flattened,
   * one-dimensional, row-major order of the tensor elements.
   */
  bytesContents: Uint8Array[];
}

function createBaseServerLiveRequest(): ServerLiveRequest {
  return {};
}

export const ServerLiveRequest = {
  encode(
    _: ServerLiveRequest,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ServerLiveRequest {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseServerLiveRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(_: any): ServerLiveRequest {
    return {};
  },

  toJSON(_: ServerLiveRequest): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ServerLiveRequest>, I>>(
    _: I
  ): ServerLiveRequest {
    const message = createBaseServerLiveRequest();
    return message;
  },
};

function createBaseServerLiveResponse(): ServerLiveResponse {
  return { live: false };
}

export const ServerLiveResponse = {
  encode(
    message: ServerLiveResponse,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.live === true) {
      writer.uint32(8).bool(message.live);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ServerLiveResponse {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseServerLiveResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.live = reader.bool();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ServerLiveResponse {
    return {
      live: isSet(object.live) ? Boolean(object.live) : false,
    };
  },

  toJSON(message: ServerLiveResponse): unknown {
    const obj: any = {};
    message.live !== undefined && (obj.live = message.live);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ServerLiveResponse>, I>>(
    object: I
  ): ServerLiveResponse {
    const message = createBaseServerLiveResponse();
    message.live = object.live ?? false;
    return message;
  },
};

function createBaseServerReadyRequest(): ServerReadyRequest {
  return {};
}

export const ServerReadyRequest = {
  encode(
    _: ServerReadyRequest,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ServerReadyRequest {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseServerReadyRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(_: any): ServerReadyRequest {
    return {};
  },

  toJSON(_: ServerReadyRequest): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ServerReadyRequest>, I>>(
    _: I
  ): ServerReadyRequest {
    const message = createBaseServerReadyRequest();
    return message;
  },
};

function createBaseServerReadyResponse(): ServerReadyResponse {
  return { ready: false };
}

export const ServerReadyResponse = {
  encode(
    message: ServerReadyResponse,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.ready === true) {
      writer.uint32(8).bool(message.ready);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ServerReadyResponse {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseServerReadyResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.ready = reader.bool();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ServerReadyResponse {
    return {
      ready: isSet(object.ready) ? Boolean(object.ready) : false,
    };
  },

  toJSON(message: ServerReadyResponse): unknown {
    const obj: any = {};
    message.ready !== undefined && (obj.ready = message.ready);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ServerReadyResponse>, I>>(
    object: I
  ): ServerReadyResponse {
    const message = createBaseServerReadyResponse();
    message.ready = object.ready ?? false;
    return message;
  },
};

function createBaseModelReadyRequest(): ModelReadyRequest {
  return { name: "", version: "" };
}

export const ModelReadyRequest = {
  encode(
    message: ModelReadyRequest,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    if (message.version !== "") {
      writer.uint32(18).string(message.version);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ModelReadyRequest {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseModelReadyRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.name = reader.string();
          break;
        case 2:
          message.version = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ModelReadyRequest {
    return {
      name: isSet(object.name) ? String(object.name) : "",
      version: isSet(object.version) ? String(object.version) : "",
    };
  },

  toJSON(message: ModelReadyRequest): unknown {
    const obj: any = {};
    message.name !== undefined && (obj.name = message.name);
    message.version !== undefined && (obj.version = message.version);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ModelReadyRequest>, I>>(
    object: I
  ): ModelReadyRequest {
    const message = createBaseModelReadyRequest();
    message.name = object.name ?? "";
    message.version = object.version ?? "";
    return message;
  },
};

function createBaseModelReadyResponse(): ModelReadyResponse {
  return { ready: false };
}

export const ModelReadyResponse = {
  encode(
    message: ModelReadyResponse,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.ready === true) {
      writer.uint32(8).bool(message.ready);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ModelReadyResponse {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseModelReadyResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.ready = reader.bool();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ModelReadyResponse {
    return {
      ready: isSet(object.ready) ? Boolean(object.ready) : false,
    };
  },

  toJSON(message: ModelReadyResponse): unknown {
    const obj: any = {};
    message.ready !== undefined && (obj.ready = message.ready);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ModelReadyResponse>, I>>(
    object: I
  ): ModelReadyResponse {
    const message = createBaseModelReadyResponse();
    message.ready = object.ready ?? false;
    return message;
  },
};

function createBaseServerMetadataRequest(): ServerMetadataRequest {
  return {};
}

export const ServerMetadataRequest = {
  encode(
    _: ServerMetadataRequest,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ServerMetadataRequest {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseServerMetadataRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(_: any): ServerMetadataRequest {
    return {};
  },

  toJSON(_: ServerMetadataRequest): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ServerMetadataRequest>, I>>(
    _: I
  ): ServerMetadataRequest {
    const message = createBaseServerMetadataRequest();
    return message;
  },
};

function createBaseServerMetadataResponse(): ServerMetadataResponse {
  return { name: "", version: "", extensions: [] };
}

export const ServerMetadataResponse = {
  encode(
    message: ServerMetadataResponse,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    if (message.version !== "") {
      writer.uint32(18).string(message.version);
    }
    for (const v of message.extensions) {
      writer.uint32(26).string(v!);
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ServerMetadataResponse {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseServerMetadataResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.name = reader.string();
          break;
        case 2:
          message.version = reader.string();
          break;
        case 3:
          message.extensions.push(reader.string());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ServerMetadataResponse {
    return {
      name: isSet(object.name) ? String(object.name) : "",
      version: isSet(object.version) ? String(object.version) : "",
      extensions: Array.isArray(object?.extensions)
        ? object.extensions.map((e: any) => String(e))
        : [],
    };
  },

  toJSON(message: ServerMetadataResponse): unknown {
    const obj: any = {};
    message.name !== undefined && (obj.name = message.name);
    message.version !== undefined && (obj.version = message.version);
    if (message.extensions) {
      obj.extensions = message.extensions.map((e) => e);
    } else {
      obj.extensions = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ServerMetadataResponse>, I>>(
    object: I
  ): ServerMetadataResponse {
    const message = createBaseServerMetadataResponse();
    message.name = object.name ?? "";
    message.version = object.version ?? "";
    message.extensions = object.extensions?.map((e) => e) || [];
    return message;
  },
};

function createBaseModelMetadataRequest(): ModelMetadataRequest {
  return { name: "", version: "" };
}

export const ModelMetadataRequest = {
  encode(
    message: ModelMetadataRequest,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    if (message.version !== "") {
      writer.uint32(18).string(message.version);
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ModelMetadataRequest {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseModelMetadataRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.name = reader.string();
          break;
        case 2:
          message.version = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ModelMetadataRequest {
    return {
      name: isSet(object.name) ? String(object.name) : "",
      version: isSet(object.version) ? String(object.version) : "",
    };
  },

  toJSON(message: ModelMetadataRequest): unknown {
    const obj: any = {};
    message.name !== undefined && (obj.name = message.name);
    message.version !== undefined && (obj.version = message.version);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ModelMetadataRequest>, I>>(
    object: I
  ): ModelMetadataRequest {
    const message = createBaseModelMetadataRequest();
    message.name = object.name ?? "";
    message.version = object.version ?? "";
    return message;
  },
};

function createBaseModelMetadataResponse(): ModelMetadataResponse {
  return {
    name: "",
    versions: [],
    platform: "",
    inputs: [],
    outputs: [],
    parameters: {},
  };
}

export const ModelMetadataResponse = {
  encode(
    message: ModelMetadataResponse,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    for (const v of message.versions) {
      writer.uint32(18).string(v!);
    }
    if (message.platform !== "") {
      writer.uint32(26).string(message.platform);
    }
    for (const v of message.inputs) {
      ModelMetadataResponse_TensorMetadata.encode(
        v!,
        writer.uint32(34).fork()
      ).ldelim();
    }
    for (const v of message.outputs) {
      ModelMetadataResponse_TensorMetadata.encode(
        v!,
        writer.uint32(42).fork()
      ).ldelim();
    }
    Object.entries(message.parameters).forEach(([key, value]) => {
      ModelMetadataResponse_ParametersEntry.encode(
        { key: key as any, value },
        writer.uint32(50).fork()
      ).ldelim();
    });
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ModelMetadataResponse {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseModelMetadataResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.name = reader.string();
          break;
        case 2:
          message.versions.push(reader.string());
          break;
        case 3:
          message.platform = reader.string();
          break;
        case 4:
          message.inputs.push(
            ModelMetadataResponse_TensorMetadata.decode(reader, reader.uint32())
          );
          break;
        case 5:
          message.outputs.push(
            ModelMetadataResponse_TensorMetadata.decode(reader, reader.uint32())
          );
          break;
        case 6:
          const entry6 = ModelMetadataResponse_ParametersEntry.decode(
            reader,
            reader.uint32()
          );
          if (entry6.value !== undefined) {
            message.parameters[entry6.key] = entry6.value;
          }
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ModelMetadataResponse {
    return {
      name: isSet(object.name) ? String(object.name) : "",
      versions: Array.isArray(object?.versions)
        ? object.versions.map((e: any) => String(e))
        : [],
      platform: isSet(object.platform) ? String(object.platform) : "",
      inputs: Array.isArray(object?.inputs)
        ? object.inputs.map((e: any) =>
            ModelMetadataResponse_TensorMetadata.fromJSON(e)
          )
        : [],
      outputs: Array.isArray(object?.outputs)
        ? object.outputs.map((e: any) =>
            ModelMetadataResponse_TensorMetadata.fromJSON(e)
          )
        : [],
      parameters: isObject(object.parameters)
        ? Object.entries(object.parameters).reduce<{
            [key: string]: InferParameter;
          }>((acc, [key, value]) => {
            acc[key] = InferParameter.fromJSON(value);
            return acc;
          }, {})
        : {},
    };
  },

  toJSON(message: ModelMetadataResponse): unknown {
    const obj: any = {};
    message.name !== undefined && (obj.name = message.name);
    if (message.versions) {
      obj.versions = message.versions.map((e) => e);
    } else {
      obj.versions = [];
    }
    message.platform !== undefined && (obj.platform = message.platform);
    if (message.inputs) {
      obj.inputs = message.inputs.map((e) =>
        e ? ModelMetadataResponse_TensorMetadata.toJSON(e) : undefined
      );
    } else {
      obj.inputs = [];
    }
    if (message.outputs) {
      obj.outputs = message.outputs.map((e) =>
        e ? ModelMetadataResponse_TensorMetadata.toJSON(e) : undefined
      );
    } else {
      obj.outputs = [];
    }
    obj.parameters = {};
    if (message.parameters) {
      Object.entries(message.parameters).forEach(([k, v]) => {
        obj.parameters[k] = InferParameter.toJSON(v);
      });
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ModelMetadataResponse>, I>>(
    object: I
  ): ModelMetadataResponse {
    const message = createBaseModelMetadataResponse();
    message.name = object.name ?? "";
    message.versions = object.versions?.map((e) => e) || [];
    message.platform = object.platform ?? "";
    message.inputs =
      object.inputs?.map((e) =>
        ModelMetadataResponse_TensorMetadata.fromPartial(e)
      ) || [];
    message.outputs =
      object.outputs?.map((e) =>
        ModelMetadataResponse_TensorMetadata.fromPartial(e)
      ) || [];
    message.parameters = Object.entries(object.parameters ?? {}).reduce<{
      [key: string]: InferParameter;
    }>((acc, [key, value]) => {
      if (value !== undefined) {
        acc[key] = InferParameter.fromPartial(value);
      }
      return acc;
    }, {});
    return message;
  },
};

function createBaseModelMetadataResponse_TensorMetadata(): ModelMetadataResponse_TensorMetadata {
  return { name: "", datatype: "", shape: [], parameters: {} };
}

export const ModelMetadataResponse_TensorMetadata = {
  encode(
    message: ModelMetadataResponse_TensorMetadata,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    if (message.datatype !== "") {
      writer.uint32(18).string(message.datatype);
    }
    writer.uint32(26).fork();
    for (const v of message.shape) {
      writer.int64(v);
    }
    writer.ldelim();
    Object.entries(message.parameters).forEach(([key, value]) => {
      ModelMetadataResponse_TensorMetadata_ParametersEntry.encode(
        { key: key as any, value },
        writer.uint32(34).fork()
      ).ldelim();
    });
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ModelMetadataResponse_TensorMetadata {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseModelMetadataResponse_TensorMetadata();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.name = reader.string();
          break;
        case 2:
          message.datatype = reader.string();
          break;
        case 3:
          if ((tag & 7) === 2) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.shape.push(longToNumber(reader.int64() as Long));
            }
          } else {
            message.shape.push(longToNumber(reader.int64() as Long));
          }
          break;
        case 4:
          const entry4 =
            ModelMetadataResponse_TensorMetadata_ParametersEntry.decode(
              reader,
              reader.uint32()
            );
          if (entry4.value !== undefined) {
            message.parameters[entry4.key] = entry4.value;
          }
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ModelMetadataResponse_TensorMetadata {
    return {
      name: isSet(object.name) ? String(object.name) : "",
      datatype: isSet(object.datatype) ? String(object.datatype) : "",
      shape: Array.isArray(object?.shape)
        ? object.shape.map((e: any) => Number(e))
        : [],
      parameters: isObject(object.parameters)
        ? Object.entries(object.parameters).reduce<{
            [key: string]: InferParameter;
          }>((acc, [key, value]) => {
            acc[key] = InferParameter.fromJSON(value);
            return acc;
          }, {})
        : {},
    };
  },

  toJSON(message: ModelMetadataResponse_TensorMetadata): unknown {
    const obj: any = {};
    message.name !== undefined && (obj.name = message.name);
    message.datatype !== undefined && (obj.datatype = message.datatype);
    if (message.shape) {
      obj.shape = message.shape.map((e) => Math.round(e));
    } else {
      obj.shape = [];
    }
    obj.parameters = {};
    if (message.parameters) {
      Object.entries(message.parameters).forEach(([k, v]) => {
        obj.parameters[k] = InferParameter.toJSON(v);
      });
    }
    return obj;
  },

  fromPartial<
    I extends Exact<DeepPartial<ModelMetadataResponse_TensorMetadata>, I>
  >(object: I): ModelMetadataResponse_TensorMetadata {
    const message = createBaseModelMetadataResponse_TensorMetadata();
    message.name = object.name ?? "";
    message.datatype = object.datatype ?? "";
    message.shape = object.shape?.map((e) => e) || [];
    message.parameters = Object.entries(object.parameters ?? {}).reduce<{
      [key: string]: InferParameter;
    }>((acc, [key, value]) => {
      if (value !== undefined) {
        acc[key] = InferParameter.fromPartial(value);
      }
      return acc;
    }, {});
    return message;
  },
};

function createBaseModelMetadataResponse_TensorMetadata_ParametersEntry(): ModelMetadataResponse_TensorMetadata_ParametersEntry {
  return { key: "", value: undefined };
}

export const ModelMetadataResponse_TensorMetadata_ParametersEntry = {
  encode(
    message: ModelMetadataResponse_TensorMetadata_ParametersEntry,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== undefined) {
      InferParameter.encode(message.value, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ModelMetadataResponse_TensorMetadata_ParametersEntry {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message =
      createBaseModelMetadataResponse_TensorMetadata_ParametersEntry();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.key = reader.string();
          break;
        case 2:
          message.value = InferParameter.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ModelMetadataResponse_TensorMetadata_ParametersEntry {
    return {
      key: isSet(object.key) ? String(object.key) : "",
      value: isSet(object.value)
        ? InferParameter.fromJSON(object.value)
        : undefined,
    };
  },

  toJSON(
    message: ModelMetadataResponse_TensorMetadata_ParametersEntry
  ): unknown {
    const obj: any = {};
    message.key !== undefined && (obj.key = message.key);
    message.value !== undefined &&
      (obj.value = message.value
        ? InferParameter.toJSON(message.value)
        : undefined);
    return obj;
  },

  fromPartial<
    I extends Exact<
      DeepPartial<ModelMetadataResponse_TensorMetadata_ParametersEntry>,
      I
    >
  >(object: I): ModelMetadataResponse_TensorMetadata_ParametersEntry {
    const message =
      createBaseModelMetadataResponse_TensorMetadata_ParametersEntry();
    message.key = object.key ?? "";
    message.value =
      object.value !== undefined && object.value !== null
        ? InferParameter.fromPartial(object.value)
        : undefined;
    return message;
  },
};

function createBaseModelMetadataResponse_ParametersEntry(): ModelMetadataResponse_ParametersEntry {
  return { key: "", value: undefined };
}

export const ModelMetadataResponse_ParametersEntry = {
  encode(
    message: ModelMetadataResponse_ParametersEntry,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== undefined) {
      InferParameter.encode(message.value, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ModelMetadataResponse_ParametersEntry {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseModelMetadataResponse_ParametersEntry();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.key = reader.string();
          break;
        case 2:
          message.value = InferParameter.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ModelMetadataResponse_ParametersEntry {
    return {
      key: isSet(object.key) ? String(object.key) : "",
      value: isSet(object.value)
        ? InferParameter.fromJSON(object.value)
        : undefined,
    };
  },

  toJSON(message: ModelMetadataResponse_ParametersEntry): unknown {
    const obj: any = {};
    message.key !== undefined && (obj.key = message.key);
    message.value !== undefined &&
      (obj.value = message.value
        ? InferParameter.toJSON(message.value)
        : undefined);
    return obj;
  },

  fromPartial<
    I extends Exact<DeepPartial<ModelMetadataResponse_ParametersEntry>, I>
  >(object: I): ModelMetadataResponse_ParametersEntry {
    const message = createBaseModelMetadataResponse_ParametersEntry();
    message.key = object.key ?? "";
    message.value =
      object.value !== undefined && object.value !== null
        ? InferParameter.fromPartial(object.value)
        : undefined;
    return message;
  },
};

function createBaseModelInferRequest(): ModelInferRequest {
  return {
    modelName: "",
    modelVersion: "",
    id: "",
    parameters: {},
    inputs: [],
    outputs: [],
  };
}

export const ModelInferRequest = {
  encode(
    message: ModelInferRequest,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.modelName !== "") {
      writer.uint32(10).string(message.modelName);
    }
    if (message.modelVersion !== "") {
      writer.uint32(18).string(message.modelVersion);
    }
    if (message.id !== "") {
      writer.uint32(26).string(message.id);
    }
    Object.entries(message.parameters).forEach(([key, value]) => {
      ModelInferRequest_ParametersEntry.encode(
        { key: key as any, value },
        writer.uint32(34).fork()
      ).ldelim();
    });
    for (const v of message.inputs) {
      ModelInferRequest_InferInputTensor.encode(
        v!,
        writer.uint32(42).fork()
      ).ldelim();
    }
    for (const v of message.outputs) {
      ModelInferRequest_InferRequestedOutputTensor.encode(
        v!,
        writer.uint32(50).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ModelInferRequest {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseModelInferRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.modelName = reader.string();
          break;
        case 2:
          message.modelVersion = reader.string();
          break;
        case 3:
          message.id = reader.string();
          break;
        case 4:
          const entry4 = ModelInferRequest_ParametersEntry.decode(
            reader,
            reader.uint32()
          );
          if (entry4.value !== undefined) {
            message.parameters[entry4.key] = entry4.value;
          }
          break;
        case 5:
          message.inputs.push(
            ModelInferRequest_InferInputTensor.decode(reader, reader.uint32())
          );
          break;
        case 6:
          message.outputs.push(
            ModelInferRequest_InferRequestedOutputTensor.decode(
              reader,
              reader.uint32()
            )
          );
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ModelInferRequest {
    return {
      modelName: isSet(object.modelName) ? String(object.modelName) : "",
      modelVersion: isSet(object.modelVersion)
        ? String(object.modelVersion)
        : "",
      id: isSet(object.id) ? String(object.id) : "",
      parameters: isObject(object.parameters)
        ? Object.entries(object.parameters).reduce<{
            [key: string]: InferParameter;
          }>((acc, [key, value]) => {
            acc[key] = InferParameter.fromJSON(value);
            return acc;
          }, {})
        : {},
      inputs: Array.isArray(object?.inputs)
        ? object.inputs.map((e: any) =>
            ModelInferRequest_InferInputTensor.fromJSON(e)
          )
        : [],
      outputs: Array.isArray(object?.outputs)
        ? object.outputs.map((e: any) =>
            ModelInferRequest_InferRequestedOutputTensor.fromJSON(e)
          )
        : [],
    };
  },

  toJSON(message: ModelInferRequest): unknown {
    const obj: any = {};
    message.modelName !== undefined && (obj.modelName = message.modelName);
    message.modelVersion !== undefined &&
      (obj.modelVersion = message.modelVersion);
    message.id !== undefined && (obj.id = message.id);
    obj.parameters = {};
    if (message.parameters) {
      Object.entries(message.parameters).forEach(([k, v]) => {
        obj.parameters[k] = InferParameter.toJSON(v);
      });
    }
    if (message.inputs) {
      obj.inputs = message.inputs.map((e) =>
        e ? ModelInferRequest_InferInputTensor.toJSON(e) : undefined
      );
    } else {
      obj.inputs = [];
    }
    if (message.outputs) {
      obj.outputs = message.outputs.map((e) =>
        e ? ModelInferRequest_InferRequestedOutputTensor.toJSON(e) : undefined
      );
    } else {
      obj.outputs = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ModelInferRequest>, I>>(
    object: I
  ): ModelInferRequest {
    const message = createBaseModelInferRequest();
    message.modelName = object.modelName ?? "";
    message.modelVersion = object.modelVersion ?? "";
    message.id = object.id ?? "";
    message.parameters = Object.entries(object.parameters ?? {}).reduce<{
      [key: string]: InferParameter;
    }>((acc, [key, value]) => {
      if (value !== undefined) {
        acc[key] = InferParameter.fromPartial(value);
      }
      return acc;
    }, {});
    message.inputs =
      object.inputs?.map((e) =>
        ModelInferRequest_InferInputTensor.fromPartial(e)
      ) || [];
    message.outputs =
      object.outputs?.map((e) =>
        ModelInferRequest_InferRequestedOutputTensor.fromPartial(e)
      ) || [];
    return message;
  },
};

function createBaseModelInferRequest_InferInputTensor(): ModelInferRequest_InferInputTensor {
  return {
    name: "",
    datatype: "",
    shape: [],
    parameters: {},
    contents: undefined,
  };
}

export const ModelInferRequest_InferInputTensor = {
  encode(
    message: ModelInferRequest_InferInputTensor,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    if (message.datatype !== "") {
      writer.uint32(18).string(message.datatype);
    }
    writer.uint32(26).fork();
    for (const v of message.shape) {
      writer.int64(v);
    }
    writer.ldelim();
    Object.entries(message.parameters).forEach(([key, value]) => {
      ModelInferRequest_InferInputTensor_ParametersEntry.encode(
        { key: key as any, value },
        writer.uint32(34).fork()
      ).ldelim();
    });
    if (message.contents !== undefined) {
      InferTensorContents.encode(
        message.contents,
        writer.uint32(42).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ModelInferRequest_InferInputTensor {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseModelInferRequest_InferInputTensor();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.name = reader.string();
          break;
        case 2:
          message.datatype = reader.string();
          break;
        case 3:
          if ((tag & 7) === 2) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.shape.push(longToNumber(reader.int64() as Long));
            }
          } else {
            message.shape.push(longToNumber(reader.int64() as Long));
          }
          break;
        case 4:
          const entry4 =
            ModelInferRequest_InferInputTensor_ParametersEntry.decode(
              reader,
              reader.uint32()
            );
          if (entry4.value !== undefined) {
            message.parameters[entry4.key] = entry4.value;
          }
          break;
        case 5:
          message.contents = InferTensorContents.decode(
            reader,
            reader.uint32()
          );
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ModelInferRequest_InferInputTensor {
    return {
      name: isSet(object.name) ? String(object.name) : "",
      datatype: isSet(object.datatype) ? String(object.datatype) : "",
      shape: Array.isArray(object?.shape)
        ? object.shape.map((e: any) => Number(e))
        : [],
      parameters: isObject(object.parameters)
        ? Object.entries(object.parameters).reduce<{
            [key: string]: InferParameter;
          }>((acc, [key, value]) => {
            acc[key] = InferParameter.fromJSON(value);
            return acc;
          }, {})
        : {},
      contents: isSet(object.contents)
        ? InferTensorContents.fromJSON(object.contents)
        : undefined,
    };
  },

  toJSON(message: ModelInferRequest_InferInputTensor): unknown {
    const obj: any = {};
    message.name !== undefined && (obj.name = message.name);
    message.datatype !== undefined && (obj.datatype = message.datatype);
    if (message.shape) {
      obj.shape = message.shape.map((e) => Math.round(e));
    } else {
      obj.shape = [];
    }
    obj.parameters = {};
    if (message.parameters) {
      Object.entries(message.parameters).forEach(([k, v]) => {
        obj.parameters[k] = InferParameter.toJSON(v);
      });
    }
    message.contents !== undefined &&
      (obj.contents = message.contents
        ? InferTensorContents.toJSON(message.contents)
        : undefined);
    return obj;
  },

  fromPartial<
    I extends Exact<DeepPartial<ModelInferRequest_InferInputTensor>, I>
  >(object: I): ModelInferRequest_InferInputTensor {
    const message = createBaseModelInferRequest_InferInputTensor();
    message.name = object.name ?? "";
    message.datatype = object.datatype ?? "";
    message.shape = object.shape?.map((e) => e) || [];
    message.parameters = Object.entries(object.parameters ?? {}).reduce<{
      [key: string]: InferParameter;
    }>((acc, [key, value]) => {
      if (value !== undefined) {
        acc[key] = InferParameter.fromPartial(value);
      }
      return acc;
    }, {});
    message.contents =
      object.contents !== undefined && object.contents !== null
        ? InferTensorContents.fromPartial(object.contents)
        : undefined;
    return message;
  },
};

function createBaseModelInferRequest_InferInputTensor_ParametersEntry(): ModelInferRequest_InferInputTensor_ParametersEntry {
  return { key: "", value: undefined };
}

export const ModelInferRequest_InferInputTensor_ParametersEntry = {
  encode(
    message: ModelInferRequest_InferInputTensor_ParametersEntry,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== undefined) {
      InferParameter.encode(message.value, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ModelInferRequest_InferInputTensor_ParametersEntry {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message =
      createBaseModelInferRequest_InferInputTensor_ParametersEntry();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.key = reader.string();
          break;
        case 2:
          message.value = InferParameter.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ModelInferRequest_InferInputTensor_ParametersEntry {
    return {
      key: isSet(object.key) ? String(object.key) : "",
      value: isSet(object.value)
        ? InferParameter.fromJSON(object.value)
        : undefined,
    };
  },

  toJSON(message: ModelInferRequest_InferInputTensor_ParametersEntry): unknown {
    const obj: any = {};
    message.key !== undefined && (obj.key = message.key);
    message.value !== undefined &&
      (obj.value = message.value
        ? InferParameter.toJSON(message.value)
        : undefined);
    return obj;
  },

  fromPartial<
    I extends Exact<
      DeepPartial<ModelInferRequest_InferInputTensor_ParametersEntry>,
      I
    >
  >(object: I): ModelInferRequest_InferInputTensor_ParametersEntry {
    const message =
      createBaseModelInferRequest_InferInputTensor_ParametersEntry();
    message.key = object.key ?? "";
    message.value =
      object.value !== undefined && object.value !== null
        ? InferParameter.fromPartial(object.value)
        : undefined;
    return message;
  },
};

function createBaseModelInferRequest_InferRequestedOutputTensor(): ModelInferRequest_InferRequestedOutputTensor {
  return { name: "", parameters: {} };
}

export const ModelInferRequest_InferRequestedOutputTensor = {
  encode(
    message: ModelInferRequest_InferRequestedOutputTensor,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    Object.entries(message.parameters).forEach(([key, value]) => {
      ModelInferRequest_InferRequestedOutputTensor_ParametersEntry.encode(
        { key: key as any, value },
        writer.uint32(18).fork()
      ).ldelim();
    });
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ModelInferRequest_InferRequestedOutputTensor {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseModelInferRequest_InferRequestedOutputTensor();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.name = reader.string();
          break;
        case 2:
          const entry2 =
            ModelInferRequest_InferRequestedOutputTensor_ParametersEntry.decode(
              reader,
              reader.uint32()
            );
          if (entry2.value !== undefined) {
            message.parameters[entry2.key] = entry2.value;
          }
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ModelInferRequest_InferRequestedOutputTensor {
    return {
      name: isSet(object.name) ? String(object.name) : "",
      parameters: isObject(object.parameters)
        ? Object.entries(object.parameters).reduce<{
            [key: string]: InferParameter;
          }>((acc, [key, value]) => {
            acc[key] = InferParameter.fromJSON(value);
            return acc;
          }, {})
        : {},
    };
  },

  toJSON(message: ModelInferRequest_InferRequestedOutputTensor): unknown {
    const obj: any = {};
    message.name !== undefined && (obj.name = message.name);
    obj.parameters = {};
    if (message.parameters) {
      Object.entries(message.parameters).forEach(([k, v]) => {
        obj.parameters[k] = InferParameter.toJSON(v);
      });
    }
    return obj;
  },

  fromPartial<
    I extends Exact<
      DeepPartial<ModelInferRequest_InferRequestedOutputTensor>,
      I
    >
  >(object: I): ModelInferRequest_InferRequestedOutputTensor {
    const message = createBaseModelInferRequest_InferRequestedOutputTensor();
    message.name = object.name ?? "";
    message.parameters = Object.entries(object.parameters ?? {}).reduce<{
      [key: string]: InferParameter;
    }>((acc, [key, value]) => {
      if (value !== undefined) {
        acc[key] = InferParameter.fromPartial(value);
      }
      return acc;
    }, {});
    return message;
  },
};

function createBaseModelInferRequest_InferRequestedOutputTensor_ParametersEntry(): ModelInferRequest_InferRequestedOutputTensor_ParametersEntry {
  return { key: "", value: undefined };
}

export const ModelInferRequest_InferRequestedOutputTensor_ParametersEntry = {
  encode(
    message: ModelInferRequest_InferRequestedOutputTensor_ParametersEntry,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== undefined) {
      InferParameter.encode(message.value, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ModelInferRequest_InferRequestedOutputTensor_ParametersEntry {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message =
      createBaseModelInferRequest_InferRequestedOutputTensor_ParametersEntry();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.key = reader.string();
          break;
        case 2:
          message.value = InferParameter.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(
    object: any
  ): ModelInferRequest_InferRequestedOutputTensor_ParametersEntry {
    return {
      key: isSet(object.key) ? String(object.key) : "",
      value: isSet(object.value)
        ? InferParameter.fromJSON(object.value)
        : undefined,
    };
  },

  toJSON(
    message: ModelInferRequest_InferRequestedOutputTensor_ParametersEntry
  ): unknown {
    const obj: any = {};
    message.key !== undefined && (obj.key = message.key);
    message.value !== undefined &&
      (obj.value = message.value
        ? InferParameter.toJSON(message.value)
        : undefined);
    return obj;
  },

  fromPartial<
    I extends Exact<
      DeepPartial<ModelInferRequest_InferRequestedOutputTensor_ParametersEntry>,
      I
    >
  >(object: I): ModelInferRequest_InferRequestedOutputTensor_ParametersEntry {
    const message =
      createBaseModelInferRequest_InferRequestedOutputTensor_ParametersEntry();
    message.key = object.key ?? "";
    message.value =
      object.value !== undefined && object.value !== null
        ? InferParameter.fromPartial(object.value)
        : undefined;
    return message;
  },
};

function createBaseModelInferRequest_ParametersEntry(): ModelInferRequest_ParametersEntry {
  return { key: "", value: undefined };
}

export const ModelInferRequest_ParametersEntry = {
  encode(
    message: ModelInferRequest_ParametersEntry,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== undefined) {
      InferParameter.encode(message.value, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ModelInferRequest_ParametersEntry {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseModelInferRequest_ParametersEntry();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.key = reader.string();
          break;
        case 2:
          message.value = InferParameter.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ModelInferRequest_ParametersEntry {
    return {
      key: isSet(object.key) ? String(object.key) : "",
      value: isSet(object.value)
        ? InferParameter.fromJSON(object.value)
        : undefined,
    };
  },

  toJSON(message: ModelInferRequest_ParametersEntry): unknown {
    const obj: any = {};
    message.key !== undefined && (obj.key = message.key);
    message.value !== undefined &&
      (obj.value = message.value
        ? InferParameter.toJSON(message.value)
        : undefined);
    return obj;
  },

  fromPartial<
    I extends Exact<DeepPartial<ModelInferRequest_ParametersEntry>, I>
  >(object: I): ModelInferRequest_ParametersEntry {
    const message = createBaseModelInferRequest_ParametersEntry();
    message.key = object.key ?? "";
    message.value =
      object.value !== undefined && object.value !== null
        ? InferParameter.fromPartial(object.value)
        : undefined;
    return message;
  },
};

function createBaseModelInferResponse(): ModelInferResponse {
  return {
    modelName: "",
    modelVersion: "",
    id: "",
    parameters: {},
    outputs: [],
  };
}

export const ModelInferResponse = {
  encode(
    message: ModelInferResponse,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.modelName !== "") {
      writer.uint32(10).string(message.modelName);
    }
    if (message.modelVersion !== "") {
      writer.uint32(18).string(message.modelVersion);
    }
    if (message.id !== "") {
      writer.uint32(26).string(message.id);
    }
    Object.entries(message.parameters).forEach(([key, value]) => {
      ModelInferResponse_ParametersEntry.encode(
        { key: key as any, value },
        writer.uint32(34).fork()
      ).ldelim();
    });
    for (const v of message.outputs) {
      ModelInferResponse_InferOutputTensor.encode(
        v!,
        writer.uint32(42).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ModelInferResponse {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseModelInferResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.modelName = reader.string();
          break;
        case 2:
          message.modelVersion = reader.string();
          break;
        case 3:
          message.id = reader.string();
          break;
        case 4:
          const entry4 = ModelInferResponse_ParametersEntry.decode(
            reader,
            reader.uint32()
          );
          if (entry4.value !== undefined) {
            message.parameters[entry4.key] = entry4.value;
          }
          break;
        case 5:
          message.outputs.push(
            ModelInferResponse_InferOutputTensor.decode(reader, reader.uint32())
          );
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ModelInferResponse {
    return {
      modelName: isSet(object.modelName) ? String(object.modelName) : "",
      modelVersion: isSet(object.modelVersion)
        ? String(object.modelVersion)
        : "",
      id: isSet(object.id) ? String(object.id) : "",
      parameters: isObject(object.parameters)
        ? Object.entries(object.parameters).reduce<{
            [key: string]: InferParameter;
          }>((acc, [key, value]) => {
            acc[key] = InferParameter.fromJSON(value);
            return acc;
          }, {})
        : {},
      outputs: Array.isArray(object?.outputs)
        ? object.outputs.map((e: any) =>
            ModelInferResponse_InferOutputTensor.fromJSON(e)
          )
        : [],
    };
  },

  toJSON(message: ModelInferResponse): unknown {
    const obj: any = {};
    message.modelName !== undefined && (obj.modelName = message.modelName);
    message.modelVersion !== undefined &&
      (obj.modelVersion = message.modelVersion);
    message.id !== undefined && (obj.id = message.id);
    obj.parameters = {};
    if (message.parameters) {
      Object.entries(message.parameters).forEach(([k, v]) => {
        obj.parameters[k] = InferParameter.toJSON(v);
      });
    }
    if (message.outputs) {
      obj.outputs = message.outputs.map((e) =>
        e ? ModelInferResponse_InferOutputTensor.toJSON(e) : undefined
      );
    } else {
      obj.outputs = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ModelInferResponse>, I>>(
    object: I
  ): ModelInferResponse {
    const message = createBaseModelInferResponse();
    message.modelName = object.modelName ?? "";
    message.modelVersion = object.modelVersion ?? "";
    message.id = object.id ?? "";
    message.parameters = Object.entries(object.parameters ?? {}).reduce<{
      [key: string]: InferParameter;
    }>((acc, [key, value]) => {
      if (value !== undefined) {
        acc[key] = InferParameter.fromPartial(value);
      }
      return acc;
    }, {});
    message.outputs =
      object.outputs?.map((e) =>
        ModelInferResponse_InferOutputTensor.fromPartial(e)
      ) || [];
    return message;
  },
};

function createBaseModelInferResponse_InferOutputTensor(): ModelInferResponse_InferOutputTensor {
  return {
    name: "",
    datatype: "",
    shape: [],
    parameters: {},
    contents: undefined,
  };
}

export const ModelInferResponse_InferOutputTensor = {
  encode(
    message: ModelInferResponse_InferOutputTensor,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    if (message.datatype !== "") {
      writer.uint32(18).string(message.datatype);
    }
    writer.uint32(26).fork();
    for (const v of message.shape) {
      writer.int64(v);
    }
    writer.ldelim();
    Object.entries(message.parameters).forEach(([key, value]) => {
      ModelInferResponse_InferOutputTensor_ParametersEntry.encode(
        { key: key as any, value },
        writer.uint32(34).fork()
      ).ldelim();
    });
    if (message.contents !== undefined) {
      InferTensorContents.encode(
        message.contents,
        writer.uint32(42).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ModelInferResponse_InferOutputTensor {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseModelInferResponse_InferOutputTensor();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.name = reader.string();
          break;
        case 2:
          message.datatype = reader.string();
          break;
        case 3:
          if ((tag & 7) === 2) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.shape.push(longToNumber(reader.int64() as Long));
            }
          } else {
            message.shape.push(longToNumber(reader.int64() as Long));
          }
          break;
        case 4:
          const entry4 =
            ModelInferResponse_InferOutputTensor_ParametersEntry.decode(
              reader,
              reader.uint32()
            );
          if (entry4.value !== undefined) {
            message.parameters[entry4.key] = entry4.value;
          }
          break;
        case 5:
          message.contents = InferTensorContents.decode(
            reader,
            reader.uint32()
          );
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ModelInferResponse_InferOutputTensor {
    return {
      name: isSet(object.name) ? String(object.name) : "",
      datatype: isSet(object.datatype) ? String(object.datatype) : "",
      shape: Array.isArray(object?.shape)
        ? object.shape.map((e: any) => Number(e))
        : [],
      parameters: isObject(object.parameters)
        ? Object.entries(object.parameters).reduce<{
            [key: string]: InferParameter;
          }>((acc, [key, value]) => {
            acc[key] = InferParameter.fromJSON(value);
            return acc;
          }, {})
        : {},
      contents: isSet(object.contents)
        ? InferTensorContents.fromJSON(object.contents)
        : undefined,
    };
  },

  toJSON(message: ModelInferResponse_InferOutputTensor): unknown {
    const obj: any = {};
    message.name !== undefined && (obj.name = message.name);
    message.datatype !== undefined && (obj.datatype = message.datatype);
    if (message.shape) {
      obj.shape = message.shape.map((e) => Math.round(e));
    } else {
      obj.shape = [];
    }
    obj.parameters = {};
    if (message.parameters) {
      Object.entries(message.parameters).forEach(([k, v]) => {
        obj.parameters[k] = InferParameter.toJSON(v);
      });
    }
    message.contents !== undefined &&
      (obj.contents = message.contents
        ? InferTensorContents.toJSON(message.contents)
        : undefined);
    return obj;
  },

  fromPartial<
    I extends Exact<DeepPartial<ModelInferResponse_InferOutputTensor>, I>
  >(object: I): ModelInferResponse_InferOutputTensor {
    const message = createBaseModelInferResponse_InferOutputTensor();
    message.name = object.name ?? "";
    message.datatype = object.datatype ?? "";
    message.shape = object.shape?.map((e) => e) || [];
    message.parameters = Object.entries(object.parameters ?? {}).reduce<{
      [key: string]: InferParameter;
    }>((acc, [key, value]) => {
      if (value !== undefined) {
        acc[key] = InferParameter.fromPartial(value);
      }
      return acc;
    }, {});
    message.contents =
      object.contents !== undefined && object.contents !== null
        ? InferTensorContents.fromPartial(object.contents)
        : undefined;
    return message;
  },
};

function createBaseModelInferResponse_InferOutputTensor_ParametersEntry(): ModelInferResponse_InferOutputTensor_ParametersEntry {
  return { key: "", value: undefined };
}

export const ModelInferResponse_InferOutputTensor_ParametersEntry = {
  encode(
    message: ModelInferResponse_InferOutputTensor_ParametersEntry,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== undefined) {
      InferParameter.encode(message.value, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ModelInferResponse_InferOutputTensor_ParametersEntry {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message =
      createBaseModelInferResponse_InferOutputTensor_ParametersEntry();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.key = reader.string();
          break;
        case 2:
          message.value = InferParameter.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ModelInferResponse_InferOutputTensor_ParametersEntry {
    return {
      key: isSet(object.key) ? String(object.key) : "",
      value: isSet(object.value)
        ? InferParameter.fromJSON(object.value)
        : undefined,
    };
  },

  toJSON(
    message: ModelInferResponse_InferOutputTensor_ParametersEntry
  ): unknown {
    const obj: any = {};
    message.key !== undefined && (obj.key = message.key);
    message.value !== undefined &&
      (obj.value = message.value
        ? InferParameter.toJSON(message.value)
        : undefined);
    return obj;
  },

  fromPartial<
    I extends Exact<
      DeepPartial<ModelInferResponse_InferOutputTensor_ParametersEntry>,
      I
    >
  >(object: I): ModelInferResponse_InferOutputTensor_ParametersEntry {
    const message =
      createBaseModelInferResponse_InferOutputTensor_ParametersEntry();
    message.key = object.key ?? "";
    message.value =
      object.value !== undefined && object.value !== null
        ? InferParameter.fromPartial(object.value)
        : undefined;
    return message;
  },
};

function createBaseModelInferResponse_ParametersEntry(): ModelInferResponse_ParametersEntry {
  return { key: "", value: undefined };
}

export const ModelInferResponse_ParametersEntry = {
  encode(
    message: ModelInferResponse_ParametersEntry,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== undefined) {
      InferParameter.encode(message.value, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ModelInferResponse_ParametersEntry {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseModelInferResponse_ParametersEntry();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.key = reader.string();
          break;
        case 2:
          message.value = InferParameter.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ModelInferResponse_ParametersEntry {
    return {
      key: isSet(object.key) ? String(object.key) : "",
      value: isSet(object.value)
        ? InferParameter.fromJSON(object.value)
        : undefined,
    };
  },

  toJSON(message: ModelInferResponse_ParametersEntry): unknown {
    const obj: any = {};
    message.key !== undefined && (obj.key = message.key);
    message.value !== undefined &&
      (obj.value = message.value
        ? InferParameter.toJSON(message.value)
        : undefined);
    return obj;
  },

  fromPartial<
    I extends Exact<DeepPartial<ModelInferResponse_ParametersEntry>, I>
  >(object: I): ModelInferResponse_ParametersEntry {
    const message = createBaseModelInferResponse_ParametersEntry();
    message.key = object.key ?? "";
    message.value =
      object.value !== undefined && object.value !== null
        ? InferParameter.fromPartial(object.value)
        : undefined;
    return message;
  },
};

function createBaseInferParameter(): InferParameter {
  return {
    boolParam: undefined,
    int64Param: undefined,
    stringParam: undefined,
  };
}

export const InferParameter = {
  encode(
    message: InferParameter,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.boolParam !== undefined) {
      writer.uint32(8).bool(message.boolParam);
    }
    if (message.int64Param !== undefined) {
      writer.uint32(16).int64(message.int64Param);
    }
    if (message.stringParam !== undefined) {
      writer.uint32(26).string(message.stringParam);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): InferParameter {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseInferParameter();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.boolParam = reader.bool();
          break;
        case 2:
          message.int64Param = longToNumber(reader.int64() as Long);
          break;
        case 3:
          message.stringParam = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): InferParameter {
    return {
      boolParam: isSet(object.boolParam)
        ? Boolean(object.boolParam)
        : undefined,
      int64Param: isSet(object.int64Param)
        ? Number(object.int64Param)
        : undefined,
      stringParam: isSet(object.stringParam)
        ? String(object.stringParam)
        : undefined,
    };
  },

  toJSON(message: InferParameter): unknown {
    const obj: any = {};
    message.boolParam !== undefined && (obj.boolParam = message.boolParam);
    message.int64Param !== undefined &&
      (obj.int64Param = Math.round(message.int64Param));
    message.stringParam !== undefined &&
      (obj.stringParam = message.stringParam);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<InferParameter>, I>>(
    object: I
  ): InferParameter {
    const message = createBaseInferParameter();
    message.boolParam = object.boolParam ?? undefined;
    message.int64Param = object.int64Param ?? undefined;
    message.stringParam = object.stringParam ?? undefined;
    return message;
  },
};

function createBaseInferTensorContents(): InferTensorContents {
  return {
    boolContents: [],
    intContents: [],
    int64Contents: [],
    uintContents: [],
    uint64Contents: [],
    fp32Contents: [],
    fp64Contents: [],
    bytesContents: [],
  };
}

export const InferTensorContents = {
  encode(
    message: InferTensorContents,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    writer.uint32(10).fork();
    for (const v of message.boolContents) {
      writer.bool(v);
    }
    writer.ldelim();
    writer.uint32(18).fork();
    for (const v of message.intContents) {
      writer.int32(v);
    }
    writer.ldelim();
    writer.uint32(26).fork();
    for (const v of message.int64Contents) {
      writer.int64(v);
    }
    writer.ldelim();
    writer.uint32(34).fork();
    for (const v of message.uintContents) {
      writer.uint32(v);
    }
    writer.ldelim();
    writer.uint32(42).fork();
    for (const v of message.uint64Contents) {
      writer.uint64(v);
    }
    writer.ldelim();
    writer.uint32(50).fork();
    for (const v of message.fp32Contents) {
      writer.float(v);
    }
    writer.ldelim();
    writer.uint32(58).fork();
    for (const v of message.fp64Contents) {
      writer.double(v);
    }
    writer.ldelim();
    for (const v of message.bytesContents) {
      writer.uint32(66).bytes(v!);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): InferTensorContents {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseInferTensorContents();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if ((tag & 7) === 2) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.boolContents.push(reader.bool());
            }
          } else {
            message.boolContents.push(reader.bool());
          }
          break;
        case 2:
          if ((tag & 7) === 2) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.intContents.push(reader.int32());
            }
          } else {
            message.intContents.push(reader.int32());
          }
          break;
        case 3:
          if ((tag & 7) === 2) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.int64Contents.push(longToNumber(reader.int64() as Long));
            }
          } else {
            message.int64Contents.push(longToNumber(reader.int64() as Long));
          }
          break;
        case 4:
          if ((tag & 7) === 2) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.uintContents.push(reader.uint32());
            }
          } else {
            message.uintContents.push(reader.uint32());
          }
          break;
        case 5:
          if ((tag & 7) === 2) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.uint64Contents.push(
                longToNumber(reader.uint64() as Long)
              );
            }
          } else {
            message.uint64Contents.push(longToNumber(reader.uint64() as Long));
          }
          break;
        case 6:
          if ((tag & 7) === 2) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.fp32Contents.push(reader.float());
            }
          } else {
            message.fp32Contents.push(reader.float());
          }
          break;
        case 7:
          if ((tag & 7) === 2) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.fp64Contents.push(reader.double());
            }
          } else {
            message.fp64Contents.push(reader.double());
          }
          break;
        case 8:
          message.bytesContents.push(reader.bytes());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): InferTensorContents {
    return {
      boolContents: Array.isArray(object?.boolContents)
        ? object.boolContents.map((e: any) => Boolean(e))
        : [],
      intContents: Array.isArray(object?.intContents)
        ? object.intContents.map((e: any) => Number(e))
        : [],
      int64Contents: Array.isArray(object?.int64Contents)
        ? object.int64Contents.map((e: any) => Number(e))
        : [],
      uintContents: Array.isArray(object?.uintContents)
        ? object.uintContents.map((e: any) => Number(e))
        : [],
      uint64Contents: Array.isArray(object?.uint64Contents)
        ? object.uint64Contents.map((e: any) => Number(e))
        : [],
      fp32Contents: Array.isArray(object?.fp32Contents)
        ? object.fp32Contents.map((e: any) => Number(e))
        : [],
      fp64Contents: Array.isArray(object?.fp64Contents)
        ? object.fp64Contents.map((e: any) => Number(e))
        : [],
      bytesContents: Array.isArray(object?.bytesContents)
        ? object.bytesContents.map((e: any) => bytesFromBase64(e))
        : [],
    };
  },

  toJSON(message: InferTensorContents): unknown {
    const obj: any = {};
    if (message.boolContents) {
      obj.boolContents = message.boolContents.map((e) => e);
    } else {
      obj.boolContents = [];
    }
    if (message.intContents) {
      obj.intContents = message.intContents.map((e) => Math.round(e));
    } else {
      obj.intContents = [];
    }
    if (message.int64Contents) {
      obj.int64Contents = message.int64Contents.map((e) => Math.round(e));
    } else {
      obj.int64Contents = [];
    }
    if (message.uintContents) {
      obj.uintContents = message.uintContents.map((e) => Math.round(e));
    } else {
      obj.uintContents = [];
    }
    if (message.uint64Contents) {
      obj.uint64Contents = message.uint64Contents.map((e) => Math.round(e));
    } else {
      obj.uint64Contents = [];
    }
    if (message.fp32Contents) {
      obj.fp32Contents = message.fp32Contents.map((e) => e);
    } else {
      obj.fp32Contents = [];
    }
    if (message.fp64Contents) {
      obj.fp64Contents = message.fp64Contents.map((e) => e);
    } else {
      obj.fp64Contents = [];
    }
    if (message.bytesContents) {
      obj.bytesContents = message.bytesContents.map((e) =>
        base64FromBytes(e !== undefined ? e : new Uint8Array())
      );
    } else {
      obj.bytesContents = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<InferTensorContents>, I>>(
    object: I
  ): InferTensorContents {
    const message = createBaseInferTensorContents();
    message.boolContents = object.boolContents?.map((e) => e) || [];
    message.intContents = object.intContents?.map((e) => e) || [];
    message.int64Contents = object.int64Contents?.map((e) => e) || [];
    message.uintContents = object.uintContents?.map((e) => e) || [];
    message.uint64Contents = object.uint64Contents?.map((e) => e) || [];
    message.fp32Contents = object.fp32Contents?.map((e) => e) || [];
    message.fp64Contents = object.fp64Contents?.map((e) => e) || [];
    message.bytesContents = object.bytesContents?.map((e) => e) || [];
    return message;
  },
};

/** Inference Server GRPC endpoints. */
export interface GRPCInferenceService {
  /** Check liveness of the inference server. */
  ServerLive(request: ServerLiveRequest): Promise<ServerLiveResponse>;
  /** Check readiness of the inference server. */
  ServerReady(request: ServerReadyRequest): Promise<ServerReadyResponse>;
  /** Check readiness of a model in the inference server. */
  ModelReady(request: ModelReadyRequest): Promise<ModelReadyResponse>;
  /** Get server metadata. */
  ServerMetadata(
    request: ServerMetadataRequest
  ): Promise<ServerMetadataResponse>;
  /** Get model metadata. */
  ModelMetadata(request: ModelMetadataRequest): Promise<ModelMetadataResponse>;
  /** Perform inference using a specific model. */
  ModelInfer(request: ModelInferRequest): Promise<ModelInferResponse>;
}

export class GRPCInferenceServiceClientImpl implements GRPCInferenceService {
  private readonly rpc: Rpc;
  constructor(rpc: Rpc) {
    this.rpc = rpc;
    this.ServerLive = this.ServerLive.bind(this);
    this.ServerReady = this.ServerReady.bind(this);
    this.ModelReady = this.ModelReady.bind(this);
    this.ServerMetadata = this.ServerMetadata.bind(this);
    this.ModelMetadata = this.ModelMetadata.bind(this);
    this.ModelInfer = this.ModelInfer.bind(this);
  }
  ServerLive(request: ServerLiveRequest): Promise<ServerLiveResponse> {
    const data = ServerLiveRequest.encode(request).finish();
    const promise = this.rpc.request(
      "inference.GRPCInferenceService",
      "ServerLive",
      data
    );
    return promise.then((data) =>
      ServerLiveResponse.decode(new _m0.Reader(data))
    );
  }

  ServerReady(request: ServerReadyRequest): Promise<ServerReadyResponse> {
    const data = ServerReadyRequest.encode(request).finish();
    const promise = this.rpc.request(
      "inference.GRPCInferenceService",
      "ServerReady",
      data
    );
    return promise.then((data) =>
      ServerReadyResponse.decode(new _m0.Reader(data))
    );
  }

  ModelReady(request: ModelReadyRequest): Promise<ModelReadyResponse> {
    const data = ModelReadyRequest.encode(request).finish();
    const promise = this.rpc.request(
      "inference.GRPCInferenceService",
      "ModelReady",
      data
    );
    return promise.then((data) =>
      ModelReadyResponse.decode(new _m0.Reader(data))
    );
  }

  ServerMetadata(
    request: ServerMetadataRequest
  ): Promise<ServerMetadataResponse> {
    const data = ServerMetadataRequest.encode(request).finish();
    const promise = this.rpc.request(
      "inference.GRPCInferenceService",
      "ServerMetadata",
      data
    );
    return promise.then((data) =>
      ServerMetadataResponse.decode(new _m0.Reader(data))
    );
  }

  ModelMetadata(request: ModelMetadataRequest): Promise<ModelMetadataResponse> {
    const data = ModelMetadataRequest.encode(request).finish();
    const promise = this.rpc.request(
      "inference.GRPCInferenceService",
      "ModelMetadata",
      data
    );
    return promise.then((data) =>
      ModelMetadataResponse.decode(new _m0.Reader(data))
    );
  }

  ModelInfer(request: ModelInferRequest): Promise<ModelInferResponse> {
    const data = ModelInferRequest.encode(request).finish();
    const promise = this.rpc.request(
      "inference.GRPCInferenceService",
      "ModelInfer",
      data
    );
    return promise.then((data) =>
      ModelInferResponse.decode(new _m0.Reader(data))
    );
  }
}

interface Rpc {
  request(
    service: string,
    method: string,
    data: Uint8Array
  ): Promise<Uint8Array>;
}

declare var self: any | undefined;
declare var window: any | undefined;
declare var global: any | undefined;
var globalThis: any = (() => {
  if (typeof globalThis !== "undefined") return globalThis;
  if (typeof self !== "undefined") return self;
  if (typeof window !== "undefined") return window;
  if (typeof global !== "undefined") return global;
  throw "Unable to locate global object";
})();

const atob: (b64: string) => string =
  globalThis.atob ||
  ((b64) => globalThis.Buffer.from(b64, "base64").toString("binary"));
function bytesFromBase64(b64: string): Uint8Array {
  const bin = atob(b64);
  const arr = new Uint8Array(bin.length);
  for (let i = 0; i < bin.length; ++i) {
    arr[i] = bin.charCodeAt(i);
  }
  return arr;
}

const btoa: (bin: string) => string =
  globalThis.btoa ||
  ((bin) => globalThis.Buffer.from(bin, "binary").toString("base64"));
function base64FromBytes(arr: Uint8Array): string {
  const bin: string[] = [];
  arr.forEach((byte) => {
    bin.push(String.fromCharCode(byte));
  });
  return btoa(bin.join(""));
}

type Builtin =
  | Date
  | Function
  | Uint8Array
  | string
  | number
  | boolean
  | undefined;

export type DeepPartial<T> = T extends Builtin
  ? T
  : T extends Array<infer U>
  ? Array<DeepPartial<U>>
  : T extends ReadonlyArray<infer U>
  ? ReadonlyArray<DeepPartial<U>>
  : T extends {}
  ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;

type KeysOfUnion<T> = T extends T ? keyof T : never;
export type Exact<P, I extends P> = P extends Builtin
  ? P
  : P & { [K in keyof P]: Exact<P[K], I[K]> } & Record<
        Exclude<keyof I, KeysOfUnion<P>>,
        never
      >;

function longToNumber(long: Long): number {
  if (long.gt(Number.MAX_SAFE_INTEGER)) {
    throw new globalThis.Error("Value is larger than Number.MAX_SAFE_INTEGER");
  }
  return long.toNumber();
}

// If you get a compile-error about 'Constructor<Long> and ... have no overlap',
// add '--ts_proto_opt=esModuleInterop=true' as a flag when calling 'protoc'.
if (_m0.util.Long !== Long) {
  _m0.util.Long = Long as any;
  _m0.configure();
}

function isObject(value: any): boolean {
  return typeof value === "object" && value !== null;
}

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}
