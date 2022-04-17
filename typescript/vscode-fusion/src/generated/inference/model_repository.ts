/* eslint-disable */
import * as Long from "long";
import * as _m0 from "protobufjs/minimal";

export const protobufPackage = "inference.model_repository";

export interface RepositoryIndexRequest {
  /**
   * The name of the repository. If empty the index is returned
   * for all repositories.
   */
  repositoryName: string;
  /** If true return only models currently ready for inferencing. */
  ready: boolean;
}

export interface RepositoryIndexResponse {
  /** An index entry for each model. */
  models: RepositoryIndexResponse_ModelIndex[];
}

/** Index entry for a model. */
export interface RepositoryIndexResponse_ModelIndex {
  /** The name of the model. */
  name: string;
  /** The version of the model. */
  version: string;
  /** The state of the model. */
  state: string;
  /** The reason, if any, that the model is in the given state. */
  reason: string;
}

export interface RepositoryModelLoadRequest {
  /**
   * The name of the repository to load from. If empty the model
   * is loaded from any repository.
   */
  repositoryName: string;
  /** The name of the model to load, or reload. */
  modelName: string;
}

export interface RepositoryModelLoadResponse {}

export interface RepositoryModelUnloadRequest {
  /**
   * The name of the repository from which the model was originally
   * loaded. If empty the repository is not considered.
   */
  repositoryName: string;
  /** The name of the model to unload. */
  modelName: string;
}

export interface RepositoryModelUnloadResponse {}

function createBaseRepositoryIndexRequest(): RepositoryIndexRequest {
  return { repositoryName: "", ready: false };
}

export const RepositoryIndexRequest = {
  encode(
    message: RepositoryIndexRequest,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.repositoryName !== "") {
      writer.uint32(10).string(message.repositoryName);
    }
    if (message.ready === true) {
      writer.uint32(16).bool(message.ready);
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): RepositoryIndexRequest {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRepositoryIndexRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.repositoryName = reader.string();
          break;
        case 2:
          message.ready = reader.bool();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): RepositoryIndexRequest {
    return {
      repositoryName: isSet(object.repositoryName)
        ? String(object.repositoryName)
        : "",
      ready: isSet(object.ready) ? Boolean(object.ready) : false,
    };
  },

  toJSON(message: RepositoryIndexRequest): unknown {
    const obj: any = {};
    message.repositoryName !== undefined &&
      (obj.repositoryName = message.repositoryName);
    message.ready !== undefined && (obj.ready = message.ready);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<RepositoryIndexRequest>, I>>(
    object: I
  ): RepositoryIndexRequest {
    const message = createBaseRepositoryIndexRequest();
    message.repositoryName = object.repositoryName ?? "";
    message.ready = object.ready ?? false;
    return message;
  },
};

function createBaseRepositoryIndexResponse(): RepositoryIndexResponse {
  return { models: [] };
}

export const RepositoryIndexResponse = {
  encode(
    message: RepositoryIndexResponse,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    for (const v of message.models) {
      RepositoryIndexResponse_ModelIndex.encode(
        v!,
        writer.uint32(10).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): RepositoryIndexResponse {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRepositoryIndexResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.models.push(
            RepositoryIndexResponse_ModelIndex.decode(reader, reader.uint32())
          );
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): RepositoryIndexResponse {
    return {
      models: Array.isArray(object?.models)
        ? object.models.map((e: any) =>
            RepositoryIndexResponse_ModelIndex.fromJSON(e)
          )
        : [],
    };
  },

  toJSON(message: RepositoryIndexResponse): unknown {
    const obj: any = {};
    if (message.models) {
      obj.models = message.models.map((e) =>
        e ? RepositoryIndexResponse_ModelIndex.toJSON(e) : undefined
      );
    } else {
      obj.models = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<RepositoryIndexResponse>, I>>(
    object: I
  ): RepositoryIndexResponse {
    const message = createBaseRepositoryIndexResponse();
    message.models =
      object.models?.map((e) =>
        RepositoryIndexResponse_ModelIndex.fromPartial(e)
      ) || [];
    return message;
  },
};

function createBaseRepositoryIndexResponse_ModelIndex(): RepositoryIndexResponse_ModelIndex {
  return { name: "", version: "", state: "", reason: "" };
}

export const RepositoryIndexResponse_ModelIndex = {
  encode(
    message: RepositoryIndexResponse_ModelIndex,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    if (message.version !== "") {
      writer.uint32(18).string(message.version);
    }
    if (message.state !== "") {
      writer.uint32(26).string(message.state);
    }
    if (message.reason !== "") {
      writer.uint32(34).string(message.reason);
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): RepositoryIndexResponse_ModelIndex {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRepositoryIndexResponse_ModelIndex();
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
          message.state = reader.string();
          break;
        case 4:
          message.reason = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): RepositoryIndexResponse_ModelIndex {
    return {
      name: isSet(object.name) ? String(object.name) : "",
      version: isSet(object.version) ? String(object.version) : "",
      state: isSet(object.state) ? String(object.state) : "",
      reason: isSet(object.reason) ? String(object.reason) : "",
    };
  },

  toJSON(message: RepositoryIndexResponse_ModelIndex): unknown {
    const obj: any = {};
    message.name !== undefined && (obj.name = message.name);
    message.version !== undefined && (obj.version = message.version);
    message.state !== undefined && (obj.state = message.state);
    message.reason !== undefined && (obj.reason = message.reason);
    return obj;
  },

  fromPartial<
    I extends Exact<DeepPartial<RepositoryIndexResponse_ModelIndex>, I>
  >(object: I): RepositoryIndexResponse_ModelIndex {
    const message = createBaseRepositoryIndexResponse_ModelIndex();
    message.name = object.name ?? "";
    message.version = object.version ?? "";
    message.state = object.state ?? "";
    message.reason = object.reason ?? "";
    return message;
  },
};

function createBaseRepositoryModelLoadRequest(): RepositoryModelLoadRequest {
  return { repositoryName: "", modelName: "" };
}

export const RepositoryModelLoadRequest = {
  encode(
    message: RepositoryModelLoadRequest,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.repositoryName !== "") {
      writer.uint32(10).string(message.repositoryName);
    }
    if (message.modelName !== "") {
      writer.uint32(18).string(message.modelName);
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): RepositoryModelLoadRequest {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRepositoryModelLoadRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.repositoryName = reader.string();
          break;
        case 2:
          message.modelName = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): RepositoryModelLoadRequest {
    return {
      repositoryName: isSet(object.repositoryName)
        ? String(object.repositoryName)
        : "",
      modelName: isSet(object.modelName) ? String(object.modelName) : "",
    };
  },

  toJSON(message: RepositoryModelLoadRequest): unknown {
    const obj: any = {};
    message.repositoryName !== undefined &&
      (obj.repositoryName = message.repositoryName);
    message.modelName !== undefined && (obj.modelName = message.modelName);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<RepositoryModelLoadRequest>, I>>(
    object: I
  ): RepositoryModelLoadRequest {
    const message = createBaseRepositoryModelLoadRequest();
    message.repositoryName = object.repositoryName ?? "";
    message.modelName = object.modelName ?? "";
    return message;
  },
};

function createBaseRepositoryModelLoadResponse(): RepositoryModelLoadResponse {
  return {};
}

export const RepositoryModelLoadResponse = {
  encode(
    _: RepositoryModelLoadResponse,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): RepositoryModelLoadResponse {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRepositoryModelLoadResponse();
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

  fromJSON(_: any): RepositoryModelLoadResponse {
    return {};
  },

  toJSON(_: RepositoryModelLoadResponse): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<RepositoryModelLoadResponse>, I>>(
    _: I
  ): RepositoryModelLoadResponse {
    const message = createBaseRepositoryModelLoadResponse();
    return message;
  },
};

function createBaseRepositoryModelUnloadRequest(): RepositoryModelUnloadRequest {
  return { repositoryName: "", modelName: "" };
}

export const RepositoryModelUnloadRequest = {
  encode(
    message: RepositoryModelUnloadRequest,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.repositoryName !== "") {
      writer.uint32(10).string(message.repositoryName);
    }
    if (message.modelName !== "") {
      writer.uint32(18).string(message.modelName);
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): RepositoryModelUnloadRequest {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRepositoryModelUnloadRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.repositoryName = reader.string();
          break;
        case 2:
          message.modelName = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): RepositoryModelUnloadRequest {
    return {
      repositoryName: isSet(object.repositoryName)
        ? String(object.repositoryName)
        : "",
      modelName: isSet(object.modelName) ? String(object.modelName) : "",
    };
  },

  toJSON(message: RepositoryModelUnloadRequest): unknown {
    const obj: any = {};
    message.repositoryName !== undefined &&
      (obj.repositoryName = message.repositoryName);
    message.modelName !== undefined && (obj.modelName = message.modelName);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<RepositoryModelUnloadRequest>, I>>(
    object: I
  ): RepositoryModelUnloadRequest {
    const message = createBaseRepositoryModelUnloadRequest();
    message.repositoryName = object.repositoryName ?? "";
    message.modelName = object.modelName ?? "";
    return message;
  },
};

function createBaseRepositoryModelUnloadResponse(): RepositoryModelUnloadResponse {
  return {};
}

export const RepositoryModelUnloadResponse = {
  encode(
    _: RepositoryModelUnloadResponse,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): RepositoryModelUnloadResponse {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRepositoryModelUnloadResponse();
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

  fromJSON(_: any): RepositoryModelUnloadResponse {
    return {};
  },

  toJSON(_: RepositoryModelUnloadResponse): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<RepositoryModelUnloadResponse>, I>>(
    _: I
  ): RepositoryModelUnloadResponse {
    const message = createBaseRepositoryModelUnloadResponse();
    return message;
  },
};

export interface ModelRepositoryService {
  /** Get the index of model repository contents. */
  RepositoryIndex(
    request: RepositoryIndexRequest
  ): Promise<RepositoryIndexResponse>;
  /** Load or reload a model from a repository. */
  RepositoryModelLoad(
    request: RepositoryModelLoadRequest
  ): Promise<RepositoryModelLoadResponse>;
  /** Unload a model. */
  RepositoryModelUnload(
    request: RepositoryModelUnloadRequest
  ): Promise<RepositoryModelUnloadResponse>;
}

export class ModelRepositoryServiceClientImpl
  implements ModelRepositoryService
{
  private readonly rpc: Rpc;
  constructor(rpc: Rpc) {
    this.rpc = rpc;
    this.RepositoryIndex = this.RepositoryIndex.bind(this);
    this.RepositoryModelLoad = this.RepositoryModelLoad.bind(this);
    this.RepositoryModelUnload = this.RepositoryModelUnload.bind(this);
  }
  RepositoryIndex(
    request: RepositoryIndexRequest
  ): Promise<RepositoryIndexResponse> {
    const data = RepositoryIndexRequest.encode(request).finish();
    const promise = this.rpc.request(
      "inference.model_repository.ModelRepositoryService",
      "RepositoryIndex",
      data
    );
    return promise.then((data) =>
      RepositoryIndexResponse.decode(new _m0.Reader(data))
    );
  }

  RepositoryModelLoad(
    request: RepositoryModelLoadRequest
  ): Promise<RepositoryModelLoadResponse> {
    const data = RepositoryModelLoadRequest.encode(request).finish();
    const promise = this.rpc.request(
      "inference.model_repository.ModelRepositoryService",
      "RepositoryModelLoad",
      data
    );
    return promise.then((data) =>
      RepositoryModelLoadResponse.decode(new _m0.Reader(data))
    );
  }

  RepositoryModelUnload(
    request: RepositoryModelUnloadRequest
  ): Promise<RepositoryModelUnloadResponse> {
    const data = RepositoryModelUnloadRequest.encode(request).finish();
    const promise = this.rpc.request(
      "inference.model_repository.ModelRepositoryService",
      "RepositoryModelUnload",
      data
    );
    return promise.then((data) =>
      RepositoryModelUnloadResponse.decode(new _m0.Reader(data))
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

// If you get a compile-error about 'Constructor<Long> and ... have no overlap',
// add '--ts_proto_opt=esModuleInterop=true' as a flag when calling 'protoc'.
if (_m0.util.Long !== Long) {
  _m0.util.Long = Long as any;
  _m0.configure();
}

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}
