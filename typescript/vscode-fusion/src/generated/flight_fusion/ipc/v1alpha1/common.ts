/* eslint-disable */
import * as Long from "long";
import * as _m0 from "protobufjs/minimal";

export const protobufPackage = "flight_fusion.ipc.v1alpha1";

/** File format for a file stroed on disk */
export enum FileFormat {
  /** FILE_FORMAT_UNSPECIFIED - Undefined file format */
  FILE_FORMAT_UNSPECIFIED = 0,
  /** FILE_FORMAT_PARQUET - Stored in parquet */
  FILE_FORMAT_PARQUET = 1,
  /** FILE_FORMAT_AVRO - Avro */
  FILE_FORMAT_AVRO = 2,
  /** FILE_FORMAT_CSV - Csv */
  FILE_FORMAT_CSV = 3,
  UNRECOGNIZED = -1,
}

export function fileFormatFromJSON(object: any): FileFormat {
  switch (object) {
    case 0:
    case "FILE_FORMAT_UNSPECIFIED":
      return FileFormat.FILE_FORMAT_UNSPECIFIED;
    case 1:
    case "FILE_FORMAT_PARQUET":
      return FileFormat.FILE_FORMAT_PARQUET;
    case 2:
    case "FILE_FORMAT_AVRO":
      return FileFormat.FILE_FORMAT_AVRO;
    case 3:
    case "FILE_FORMAT_CSV":
      return FileFormat.FILE_FORMAT_CSV;
    case -1:
    case "UNRECOGNIZED":
    default:
      return FileFormat.UNRECOGNIZED;
  }
}

export function fileFormatToJSON(object: FileFormat): string {
  switch (object) {
    case FileFormat.FILE_FORMAT_UNSPECIFIED:
      return "FILE_FORMAT_UNSPECIFIED";
    case FileFormat.FILE_FORMAT_PARQUET:
      return "FILE_FORMAT_PARQUET";
    case FileFormat.FILE_FORMAT_AVRO:
      return "FILE_FORMAT_AVRO";
    case FileFormat.FILE_FORMAT_CSV:
      return "FILE_FORMAT_CSV";
    default:
      return "UNKNOWN";
  }
}

export enum SaveMode {
  SAVE_MODE_UNSPECIFIED = 0,
  SAVE_MODE_APPEND = 1,
  SAVE_MODE_OVERWRITE = 2,
  SAVE_MODE_ERROR_IF_EXISTS = 3,
  UNRECOGNIZED = -1,
}

export function saveModeFromJSON(object: any): SaveMode {
  switch (object) {
    case 0:
    case "SAVE_MODE_UNSPECIFIED":
      return SaveMode.SAVE_MODE_UNSPECIFIED;
    case 1:
    case "SAVE_MODE_APPEND":
      return SaveMode.SAVE_MODE_APPEND;
    case 2:
    case "SAVE_MODE_OVERWRITE":
      return SaveMode.SAVE_MODE_OVERWRITE;
    case 3:
    case "SAVE_MODE_ERROR_IF_EXISTS":
      return SaveMode.SAVE_MODE_ERROR_IF_EXISTS;
    case -1:
    case "UNRECOGNIZED":
    default:
      return SaveMode.UNRECOGNIZED;
  }
}

export function saveModeToJSON(object: SaveMode): string {
  switch (object) {
    case SaveMode.SAVE_MODE_UNSPECIFIED:
      return "SAVE_MODE_UNSPECIFIED";
    case SaveMode.SAVE_MODE_APPEND:
      return "SAVE_MODE_APPEND";
    case SaveMode.SAVE_MODE_OVERWRITE:
      return "SAVE_MODE_OVERWRITE";
    case SaveMode.SAVE_MODE_ERROR_IF_EXISTS:
      return "SAVE_MODE_ERROR_IF_EXISTS";
    default:
      return "UNKNOWN";
  }
}

export interface FileReference {
  path: string;
  format: FileFormat;
}

export interface TableReference {
  file: FileReference | undefined;
}

export interface EntityUri {
  uri: string;
}

export interface EntityId {
  id: string;
}

export interface EntityPath {
  path: string[];
}

export interface AreaTableLocation {
  name: string;
  areas: string[];
}

export interface AreaTableId {
  id: string;
}

export interface AreaTableUri {
  uri: string;
}

export interface AreaSourceReference {
  location: AreaTableLocation | undefined;
  id: AreaTableId | undefined;
  uri: AreaTableUri | undefined;
}

export interface SourceCollection {
  sources: AreaSourceReference[];
}

export interface Tag {
  key: string;
  value: string;
}

function createBaseFileReference(): FileReference {
  return { path: "", format: 0 };
}

export const FileReference = {
  encode(
    message: FileReference,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.path !== "") {
      writer.uint32(10).string(message.path);
    }
    if (message.format !== 0) {
      writer.uint32(16).int32(message.format);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): FileReference {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseFileReference();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.path = reader.string();
          break;
        case 2:
          message.format = reader.int32() as any;
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): FileReference {
    return {
      path: isSet(object.path) ? String(object.path) : "",
      format: isSet(object.format) ? fileFormatFromJSON(object.format) : 0,
    };
  },

  toJSON(message: FileReference): unknown {
    const obj: any = {};
    message.path !== undefined && (obj.path = message.path);
    message.format !== undefined &&
      (obj.format = fileFormatToJSON(message.format));
    return obj;
  },

  fromPartial(object: DeepPartial<FileReference>): FileReference {
    const message = createBaseFileReference();
    message.path = object.path ?? "";
    message.format = object.format ?? 0;
    return message;
  },
};

function createBaseTableReference(): TableReference {
  return { file: undefined };
}

export const TableReference = {
  encode(
    message: TableReference,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.file !== undefined) {
      FileReference.encode(message.file, writer.uint32(26).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): TableReference {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseTableReference();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 3:
          message.file = FileReference.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): TableReference {
    return {
      file: isSet(object.file)
        ? FileReference.fromJSON(object.file)
        : undefined,
    };
  },

  toJSON(message: TableReference): unknown {
    const obj: any = {};
    message.file !== undefined &&
      (obj.file = message.file
        ? FileReference.toJSON(message.file)
        : undefined);
    return obj;
  },

  fromPartial(object: DeepPartial<TableReference>): TableReference {
    const message = createBaseTableReference();
    message.file =
      object.file !== undefined && object.file !== null
        ? FileReference.fromPartial(object.file)
        : undefined;
    return message;
  },
};

function createBaseEntityUri(): EntityUri {
  return { uri: "" };
}

export const EntityUri = {
  encode(
    message: EntityUri,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.uri !== "") {
      writer.uint32(10).string(message.uri);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): EntityUri {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseEntityUri();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.uri = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): EntityUri {
    return {
      uri: isSet(object.uri) ? String(object.uri) : "",
    };
  },

  toJSON(message: EntityUri): unknown {
    const obj: any = {};
    message.uri !== undefined && (obj.uri = message.uri);
    return obj;
  },

  fromPartial(object: DeepPartial<EntityUri>): EntityUri {
    const message = createBaseEntityUri();
    message.uri = object.uri ?? "";
    return message;
  },
};

function createBaseEntityId(): EntityId {
  return { id: "" };
}

export const EntityId = {
  encode(
    message: EntityId,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.id !== "") {
      writer.uint32(10).string(message.id);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): EntityId {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseEntityId();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.id = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): EntityId {
    return {
      id: isSet(object.id) ? String(object.id) : "",
    };
  },

  toJSON(message: EntityId): unknown {
    const obj: any = {};
    message.id !== undefined && (obj.id = message.id);
    return obj;
  },

  fromPartial(object: DeepPartial<EntityId>): EntityId {
    const message = createBaseEntityId();
    message.id = object.id ?? "";
    return message;
  },
};

function createBaseEntityPath(): EntityPath {
  return { path: [] };
}

export const EntityPath = {
  encode(
    message: EntityPath,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    for (const v of message.path) {
      writer.uint32(10).string(v!);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): EntityPath {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseEntityPath();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.path.push(reader.string());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): EntityPath {
    return {
      path: Array.isArray(object?.path)
        ? object.path.map((e: any) => String(e))
        : [],
    };
  },

  toJSON(message: EntityPath): unknown {
    const obj: any = {};
    if (message.path) {
      obj.path = message.path.map((e) => e);
    } else {
      obj.path = [];
    }
    return obj;
  },

  fromPartial(object: DeepPartial<EntityPath>): EntityPath {
    const message = createBaseEntityPath();
    message.path = object.path?.map((e) => e) || [];
    return message;
  },
};

function createBaseAreaTableLocation(): AreaTableLocation {
  return { name: "", areas: [] };
}

export const AreaTableLocation = {
  encode(
    message: AreaTableLocation,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    for (const v of message.areas) {
      writer.uint32(18).string(v!);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): AreaTableLocation {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseAreaTableLocation();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.name = reader.string();
          break;
        case 2:
          message.areas.push(reader.string());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): AreaTableLocation {
    return {
      name: isSet(object.name) ? String(object.name) : "",
      areas: Array.isArray(object?.areas)
        ? object.areas.map((e: any) => String(e))
        : [],
    };
  },

  toJSON(message: AreaTableLocation): unknown {
    const obj: any = {};
    message.name !== undefined && (obj.name = message.name);
    if (message.areas) {
      obj.areas = message.areas.map((e) => e);
    } else {
      obj.areas = [];
    }
    return obj;
  },

  fromPartial(object: DeepPartial<AreaTableLocation>): AreaTableLocation {
    const message = createBaseAreaTableLocation();
    message.name = object.name ?? "";
    message.areas = object.areas?.map((e) => e) || [];
    return message;
  },
};

function createBaseAreaTableId(): AreaTableId {
  return { id: "" };
}

export const AreaTableId = {
  encode(
    message: AreaTableId,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.id !== "") {
      writer.uint32(10).string(message.id);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): AreaTableId {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseAreaTableId();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.id = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): AreaTableId {
    return {
      id: isSet(object.id) ? String(object.id) : "",
    };
  },

  toJSON(message: AreaTableId): unknown {
    const obj: any = {};
    message.id !== undefined && (obj.id = message.id);
    return obj;
  },

  fromPartial(object: DeepPartial<AreaTableId>): AreaTableId {
    const message = createBaseAreaTableId();
    message.id = object.id ?? "";
    return message;
  },
};

function createBaseAreaTableUri(): AreaTableUri {
  return { uri: "" };
}

export const AreaTableUri = {
  encode(
    message: AreaTableUri,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.uri !== "") {
      writer.uint32(10).string(message.uri);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): AreaTableUri {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseAreaTableUri();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.uri = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): AreaTableUri {
    return {
      uri: isSet(object.uri) ? String(object.uri) : "",
    };
  },

  toJSON(message: AreaTableUri): unknown {
    const obj: any = {};
    message.uri !== undefined && (obj.uri = message.uri);
    return obj;
  },

  fromPartial(object: DeepPartial<AreaTableUri>): AreaTableUri {
    const message = createBaseAreaTableUri();
    message.uri = object.uri ?? "";
    return message;
  },
};

function createBaseAreaSourceReference(): AreaSourceReference {
  return { location: undefined, id: undefined, uri: undefined };
}

export const AreaSourceReference = {
  encode(
    message: AreaSourceReference,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.location !== undefined) {
      AreaTableLocation.encode(
        message.location,
        writer.uint32(10).fork()
      ).ldelim();
    }
    if (message.id !== undefined) {
      AreaTableId.encode(message.id, writer.uint32(18).fork()).ldelim();
    }
    if (message.uri !== undefined) {
      AreaTableUri.encode(message.uri, writer.uint32(26).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): AreaSourceReference {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseAreaSourceReference();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.location = AreaTableLocation.decode(reader, reader.uint32());
          break;
        case 2:
          message.id = AreaTableId.decode(reader, reader.uint32());
          break;
        case 3:
          message.uri = AreaTableUri.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): AreaSourceReference {
    return {
      location: isSet(object.location)
        ? AreaTableLocation.fromJSON(object.location)
        : undefined,
      id: isSet(object.id) ? AreaTableId.fromJSON(object.id) : undefined,
      uri: isSet(object.uri) ? AreaTableUri.fromJSON(object.uri) : undefined,
    };
  },

  toJSON(message: AreaSourceReference): unknown {
    const obj: any = {};
    message.location !== undefined &&
      (obj.location = message.location
        ? AreaTableLocation.toJSON(message.location)
        : undefined);
    message.id !== undefined &&
      (obj.id = message.id ? AreaTableId.toJSON(message.id) : undefined);
    message.uri !== undefined &&
      (obj.uri = message.uri ? AreaTableUri.toJSON(message.uri) : undefined);
    return obj;
  },

  fromPartial(object: DeepPartial<AreaSourceReference>): AreaSourceReference {
    const message = createBaseAreaSourceReference();
    message.location =
      object.location !== undefined && object.location !== null
        ? AreaTableLocation.fromPartial(object.location)
        : undefined;
    message.id =
      object.id !== undefined && object.id !== null
        ? AreaTableId.fromPartial(object.id)
        : undefined;
    message.uri =
      object.uri !== undefined && object.uri !== null
        ? AreaTableUri.fromPartial(object.uri)
        : undefined;
    return message;
  },
};

function createBaseSourceCollection(): SourceCollection {
  return { sources: [] };
}

export const SourceCollection = {
  encode(
    message: SourceCollection,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    for (const v of message.sources) {
      AreaSourceReference.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): SourceCollection {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSourceCollection();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.sources.push(
            AreaSourceReference.decode(reader, reader.uint32())
          );
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): SourceCollection {
    return {
      sources: Array.isArray(object?.sources)
        ? object.sources.map((e: any) => AreaSourceReference.fromJSON(e))
        : [],
    };
  },

  toJSON(message: SourceCollection): unknown {
    const obj: any = {};
    if (message.sources) {
      obj.sources = message.sources.map((e) =>
        e ? AreaSourceReference.toJSON(e) : undefined
      );
    } else {
      obj.sources = [];
    }
    return obj;
  },

  fromPartial(object: DeepPartial<SourceCollection>): SourceCollection {
    const message = createBaseSourceCollection();
    message.sources =
      object.sources?.map((e) => AreaSourceReference.fromPartial(e)) || [];
    return message;
  },
};

function createBaseTag(): Tag {
  return { key: "", value: "" };
}

export const Tag = {
  encode(message: Tag, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== "") {
      writer.uint32(18).string(message.value);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Tag {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseTag();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.key = reader.string();
          break;
        case 2:
          message.value = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Tag {
    return {
      key: isSet(object.key) ? String(object.key) : "",
      value: isSet(object.value) ? String(object.value) : "",
    };
  },

  toJSON(message: Tag): unknown {
    const obj: any = {};
    message.key !== undefined && (obj.key = message.key);
    message.value !== undefined && (obj.value = message.value);
    return obj;
  },

  fromPartial(object: DeepPartial<Tag>): Tag {
    const message = createBaseTag();
    message.key = object.key ?? "";
    message.value = object.value ?? "";
    return message;
  },
};

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

// If you get a compile-error about 'Constructor<Long> and ... have no overlap',
// add '--ts_proto_opt=esModuleInterop=true' as a flag when calling 'protoc'.
if (_m0.util.Long !== Long) {
  _m0.util.Long = Long as any;
  _m0.configure();
}

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}
