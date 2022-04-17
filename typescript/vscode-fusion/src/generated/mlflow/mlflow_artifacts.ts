/* eslint-disable */
import * as Long from "long";
import * as _m0 from "protobufjs/minimal";

export const protobufPackage = "mlflow.artifacts";

export interface DownloadArtifact {}

export interface DownloadArtifact_Response {}

export interface UploadArtifact {}

export interface UploadArtifact_Response {}

export interface ListArtifacts {
  /** Filter artifacts matching this path (a relative path from the root artifact directory). */
  path: string;
}

export interface ListArtifacts_Response {
  /** File location and metadata for artifacts. */
  files: FileInfo[];
}

export interface FileInfo {
  /** Path relative to the root artifact directory run. */
  path: string;
  /** Whether the path is a directory. */
  isDir: boolean;
  /** Size in bytes. Unset for directories. */
  fileSize: number;
}

function createBaseDownloadArtifact(): DownloadArtifact {
  return {};
}

export const DownloadArtifact = {
  encode(
    _: DownloadArtifact,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): DownloadArtifact {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDownloadArtifact();
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

  fromJSON(_: any): DownloadArtifact {
    return {};
  },

  toJSON(_: DownloadArtifact): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<DownloadArtifact>, I>>(
    _: I
  ): DownloadArtifact {
    const message = createBaseDownloadArtifact();
    return message;
  },
};

function createBaseDownloadArtifact_Response(): DownloadArtifact_Response {
  return {};
}

export const DownloadArtifact_Response = {
  encode(
    _: DownloadArtifact_Response,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): DownloadArtifact_Response {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDownloadArtifact_Response();
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

  fromJSON(_: any): DownloadArtifact_Response {
    return {};
  },

  toJSON(_: DownloadArtifact_Response): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<DownloadArtifact_Response>, I>>(
    _: I
  ): DownloadArtifact_Response {
    const message = createBaseDownloadArtifact_Response();
    return message;
  },
};

function createBaseUploadArtifact(): UploadArtifact {
  return {};
}

export const UploadArtifact = {
  encode(
    _: UploadArtifact,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UploadArtifact {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUploadArtifact();
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

  fromJSON(_: any): UploadArtifact {
    return {};
  },

  toJSON(_: UploadArtifact): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<UploadArtifact>, I>>(
    _: I
  ): UploadArtifact {
    const message = createBaseUploadArtifact();
    return message;
  },
};

function createBaseUploadArtifact_Response(): UploadArtifact_Response {
  return {};
}

export const UploadArtifact_Response = {
  encode(
    _: UploadArtifact_Response,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): UploadArtifact_Response {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUploadArtifact_Response();
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

  fromJSON(_: any): UploadArtifact_Response {
    return {};
  },

  toJSON(_: UploadArtifact_Response): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<UploadArtifact_Response>, I>>(
    _: I
  ): UploadArtifact_Response {
    const message = createBaseUploadArtifact_Response();
    return message;
  },
};

function createBaseListArtifacts(): ListArtifacts {
  return { path: "" };
}

export const ListArtifacts = {
  encode(
    message: ListArtifacts,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.path !== "") {
      writer.uint32(10).string(message.path);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ListArtifacts {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseListArtifacts();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.path = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ListArtifacts {
    return {
      path: isSet(object.path) ? String(object.path) : "",
    };
  },

  toJSON(message: ListArtifacts): unknown {
    const obj: any = {};
    message.path !== undefined && (obj.path = message.path);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ListArtifacts>, I>>(
    object: I
  ): ListArtifacts {
    const message = createBaseListArtifacts();
    message.path = object.path ?? "";
    return message;
  },
};

function createBaseListArtifacts_Response(): ListArtifacts_Response {
  return { files: [] };
}

export const ListArtifacts_Response = {
  encode(
    message: ListArtifacts_Response,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    for (const v of message.files) {
      FileInfo.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ListArtifacts_Response {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseListArtifacts_Response();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.files.push(FileInfo.decode(reader, reader.uint32()));
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ListArtifacts_Response {
    return {
      files: Array.isArray(object?.files)
        ? object.files.map((e: any) => FileInfo.fromJSON(e))
        : [],
    };
  },

  toJSON(message: ListArtifacts_Response): unknown {
    const obj: any = {};
    if (message.files) {
      obj.files = message.files.map((e) =>
        e ? FileInfo.toJSON(e) : undefined
      );
    } else {
      obj.files = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ListArtifacts_Response>, I>>(
    object: I
  ): ListArtifacts_Response {
    const message = createBaseListArtifacts_Response();
    message.files = object.files?.map((e) => FileInfo.fromPartial(e)) || [];
    return message;
  },
};

function createBaseFileInfo(): FileInfo {
  return { path: "", isDir: false, fileSize: 0 };
}

export const FileInfo = {
  encode(
    message: FileInfo,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.path !== "") {
      writer.uint32(10).string(message.path);
    }
    if (message.isDir === true) {
      writer.uint32(16).bool(message.isDir);
    }
    if (message.fileSize !== 0) {
      writer.uint32(24).int64(message.fileSize);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): FileInfo {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseFileInfo();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.path = reader.string();
          break;
        case 2:
          message.isDir = reader.bool();
          break;
        case 3:
          message.fileSize = longToNumber(reader.int64() as Long);
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): FileInfo {
    return {
      path: isSet(object.path) ? String(object.path) : "",
      isDir: isSet(object.isDir) ? Boolean(object.isDir) : false,
      fileSize: isSet(object.fileSize) ? Number(object.fileSize) : 0,
    };
  },

  toJSON(message: FileInfo): unknown {
    const obj: any = {};
    message.path !== undefined && (obj.path = message.path);
    message.isDir !== undefined && (obj.isDir = message.isDir);
    message.fileSize !== undefined &&
      (obj.fileSize = Math.round(message.fileSize));
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<FileInfo>, I>>(object: I): FileInfo {
    const message = createBaseFileInfo();
    message.path = object.path ?? "";
    message.isDir = object.isDir ?? false;
    message.fileSize = object.fileSize ?? 0;
    return message;
  },
};

export interface MlflowArtifactsService {
  downloadArtifact(
    request: DownloadArtifact
  ): Promise<DownloadArtifact_Response>;
  uploadArtifact(request: UploadArtifact): Promise<UploadArtifact_Response>;
  listArtifacts(request: ListArtifacts): Promise<ListArtifacts_Response>;
}

export class MlflowArtifactsServiceClientImpl
  implements MlflowArtifactsService
{
  private readonly rpc: Rpc;
  constructor(rpc: Rpc) {
    this.rpc = rpc;
    this.downloadArtifact = this.downloadArtifact.bind(this);
    this.uploadArtifact = this.uploadArtifact.bind(this);
    this.listArtifacts = this.listArtifacts.bind(this);
  }
  downloadArtifact(
    request: DownloadArtifact
  ): Promise<DownloadArtifact_Response> {
    const data = DownloadArtifact.encode(request).finish();
    const promise = this.rpc.request(
      "mlflow.artifacts.MlflowArtifactsService",
      "downloadArtifact",
      data
    );
    return promise.then((data) =>
      DownloadArtifact_Response.decode(new _m0.Reader(data))
    );
  }

  uploadArtifact(request: UploadArtifact): Promise<UploadArtifact_Response> {
    const data = UploadArtifact.encode(request).finish();
    const promise = this.rpc.request(
      "mlflow.artifacts.MlflowArtifactsService",
      "uploadArtifact",
      data
    );
    return promise.then((data) =>
      UploadArtifact_Response.decode(new _m0.Reader(data))
    );
  }

  listArtifacts(request: ListArtifacts): Promise<ListArtifacts_Response> {
    const data = ListArtifacts.encode(request).finish();
    const promise = this.rpc.request(
      "mlflow.artifacts.MlflowArtifactsService",
      "listArtifacts",
      data
    );
    return promise.then((data) =>
      ListArtifacts_Response.decode(new _m0.Reader(data))
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

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}
