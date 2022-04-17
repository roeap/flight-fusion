/* eslint-disable */
import * as Long from "long";
import * as _m0 from "protobufjs/minimal";

export const protobufPackage = "scalapb";

export interface ScalaPbOptions {
  /** If set then it overrides the java_package and package. */
  packageName: string;
  /**
   * If true, the compiler does not append the proto base file name
   * into the generated package name. If false (the default), the
   * generated scala package name is the package_name.basename where
   * basename is the proto file name without the .proto extension.
   */
  flatPackage: boolean;
  /**
   * Adds the following imports at the top of the file (this is meant
   * to provide implicit TypeMappers)
   */
  import: string[];
}

export interface MessageOptions {
  /** additional classes and traits to mix in to the case class. */
  extends: string[];
}

export interface FieldOptions {
  type: string;
}

function createBaseScalaPbOptions(): ScalaPbOptions {
  return { packageName: "", flatPackage: false, import: [] };
}

export const ScalaPbOptions = {
  encode(
    message: ScalaPbOptions,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.packageName !== "") {
      writer.uint32(10).string(message.packageName);
    }
    if (message.flatPackage === true) {
      writer.uint32(16).bool(message.flatPackage);
    }
    for (const v of message.import) {
      writer.uint32(26).string(v!);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ScalaPbOptions {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseScalaPbOptions();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.packageName = reader.string();
          break;
        case 2:
          message.flatPackage = reader.bool();
          break;
        case 3:
          message.import.push(reader.string());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ScalaPbOptions {
    return {
      packageName: isSet(object.packageName) ? String(object.packageName) : "",
      flatPackage: isSet(object.flatPackage)
        ? Boolean(object.flatPackage)
        : false,
      import: Array.isArray(object?.import)
        ? object.import.map((e: any) => String(e))
        : [],
    };
  },

  toJSON(message: ScalaPbOptions): unknown {
    const obj: any = {};
    message.packageName !== undefined &&
      (obj.packageName = message.packageName);
    message.flatPackage !== undefined &&
      (obj.flatPackage = message.flatPackage);
    if (message.import) {
      obj.import = message.import.map((e) => e);
    } else {
      obj.import = [];
    }
    return obj;
  },

  fromPartial(object: DeepPartial<ScalaPbOptions>): ScalaPbOptions {
    const message = createBaseScalaPbOptions();
    message.packageName = object.packageName ?? "";
    message.flatPackage = object.flatPackage ?? false;
    message.import = object.import?.map((e) => e) || [];
    return message;
  },
};

function createBaseMessageOptions(): MessageOptions {
  return { extends: [] };
}

export const MessageOptions = {
  encode(
    message: MessageOptions,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    for (const v of message.extends) {
      writer.uint32(10).string(v!);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): MessageOptions {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseMessageOptions();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.extends.push(reader.string());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): MessageOptions {
    return {
      extends: Array.isArray(object?.extends)
        ? object.extends.map((e: any) => String(e))
        : [],
    };
  },

  toJSON(message: MessageOptions): unknown {
    const obj: any = {};
    if (message.extends) {
      obj.extends = message.extends.map((e) => e);
    } else {
      obj.extends = [];
    }
    return obj;
  },

  fromPartial(object: DeepPartial<MessageOptions>): MessageOptions {
    const message = createBaseMessageOptions();
    message.extends = object.extends?.map((e) => e) || [];
    return message;
  },
};

function createBaseFieldOptions(): FieldOptions {
  return { type: "" };
}

export const FieldOptions = {
  encode(
    message: FieldOptions,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.type !== "") {
      writer.uint32(10).string(message.type);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): FieldOptions {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseFieldOptions();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.type = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): FieldOptions {
    return {
      type: isSet(object.type) ? String(object.type) : "",
    };
  },

  toJSON(message: FieldOptions): unknown {
    const obj: any = {};
    message.type !== undefined && (obj.type = message.type);
    return obj;
  },

  fromPartial(object: DeepPartial<FieldOptions>): FieldOptions {
    const message = createBaseFieldOptions();
    message.type = object.type ?? "";
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
