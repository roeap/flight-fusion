/* eslint-disable */
import * as Long from "long";
import * as _m0 from "protobufjs/minimal";
import { TableReference } from "../../../flight_fusion/ipc/v1alpha1/common";

export const protobufPackage = "flight_fusion.ipc.v1alpha1";

export enum SignalType {
  SIGNAL_TYPE_UNSPECIFIED = 0,
  SIGNAL_TYPE_OBSERVATION = 1,
  SIGNAL_TYPE_CONSTANT = 2,
  SIGNAL_TYPE_EXPRESSION = 3,
  SIGNAL_TYPE_MODEL = 4,
  UNRECOGNIZED = -1,
}

export function signalTypeFromJSON(object: any): SignalType {
  switch (object) {
    case 0:
    case "SIGNAL_TYPE_UNSPECIFIED":
      return SignalType.SIGNAL_TYPE_UNSPECIFIED;
    case 1:
    case "SIGNAL_TYPE_OBSERVATION":
      return SignalType.SIGNAL_TYPE_OBSERVATION;
    case 2:
    case "SIGNAL_TYPE_CONSTANT":
      return SignalType.SIGNAL_TYPE_CONSTANT;
    case 3:
    case "SIGNAL_TYPE_EXPRESSION":
      return SignalType.SIGNAL_TYPE_EXPRESSION;
    case 4:
    case "SIGNAL_TYPE_MODEL":
      return SignalType.SIGNAL_TYPE_MODEL;
    case -1:
    case "UNRECOGNIZED":
    default:
      return SignalType.UNRECOGNIZED;
  }
}

export function signalTypeToJSON(object: SignalType): string {
  switch (object) {
    case SignalType.SIGNAL_TYPE_UNSPECIFIED:
      return "SIGNAL_TYPE_UNSPECIFIED";
    case SignalType.SIGNAL_TYPE_OBSERVATION:
      return "SIGNAL_TYPE_OBSERVATION";
    case SignalType.SIGNAL_TYPE_CONSTANT:
      return "SIGNAL_TYPE_CONSTANT";
    case SignalType.SIGNAL_TYPE_EXPRESSION:
      return "SIGNAL_TYPE_EXPRESSION";
    case SignalType.SIGNAL_TYPE_MODEL:
      return "SIGNAL_TYPE_MODEL";
    default:
      return "UNKNOWN";
  }
}

export interface ExpressionReference {
  uid: string;
  expression: string;
}

export interface ModelReference {
  uri: string;
}

export interface Signal {
  uid: string;
  name: string;
  description: string;
  traits: SignalTrait[];
}

export interface SignalTrait {
  sensitive: SensitiveDataTrait | undefined;
  timeSeries: TimeSeriesTrait | undefined;
  entityReference: EntityReferenceTrait | undefined;
}

export interface SensitiveDataTrait {
  level: string;
}

export interface TimeSeriesTrait {
  level: string;
}

export interface EntityReferenceTrait {
  level: string;
}

export interface SignalProvider {
  uid: string;
  name: string;
  description: string;
  signals: Signal[];
  inputs: Signal[];
  table: TableReference | undefined;
  expression: ExpressionReference | undefined;
  model: ModelReference | undefined;
}

/**
 * A SignalFrame defines the context for a specialized query across
 * multiple data sources
 */
export interface SignalFrame {
  uid: string;
  name: string;
  description: string;
  providers: SignalProvider[];
}

function createBaseExpressionReference(): ExpressionReference {
  return { uid: "", expression: "" };
}

export const ExpressionReference = {
  encode(
    message: ExpressionReference,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.uid !== "") {
      writer.uint32(10).string(message.uid);
    }
    if (message.expression !== "") {
      writer.uint32(18).string(message.expression);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ExpressionReference {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseExpressionReference();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.uid = reader.string();
          break;
        case 2:
          message.expression = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ExpressionReference {
    return {
      uid: isSet(object.uid) ? String(object.uid) : "",
      expression: isSet(object.expression) ? String(object.expression) : "",
    };
  },

  toJSON(message: ExpressionReference): unknown {
    const obj: any = {};
    message.uid !== undefined && (obj.uid = message.uid);
    message.expression !== undefined && (obj.expression = message.expression);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ExpressionReference>, I>>(
    object: I
  ): ExpressionReference {
    const message = createBaseExpressionReference();
    message.uid = object.uid ?? "";
    message.expression = object.expression ?? "";
    return message;
  },
};

function createBaseModelReference(): ModelReference {
  return { uri: "" };
}

export const ModelReference = {
  encode(
    message: ModelReference,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.uri !== "") {
      writer.uint32(10).string(message.uri);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ModelReference {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseModelReference();
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

  fromJSON(object: any): ModelReference {
    return {
      uri: isSet(object.uri) ? String(object.uri) : "",
    };
  },

  toJSON(message: ModelReference): unknown {
    const obj: any = {};
    message.uri !== undefined && (obj.uri = message.uri);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ModelReference>, I>>(
    object: I
  ): ModelReference {
    const message = createBaseModelReference();
    message.uri = object.uri ?? "";
    return message;
  },
};

function createBaseSignal(): Signal {
  return { uid: "", name: "", description: "", traits: [] };
}

export const Signal = {
  encode(
    message: Signal,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.uid !== "") {
      writer.uint32(10).string(message.uid);
    }
    if (message.name !== "") {
      writer.uint32(18).string(message.name);
    }
    if (message.description !== "") {
      writer.uint32(26).string(message.description);
    }
    for (const v of message.traits) {
      SignalTrait.encode(v!, writer.uint32(82).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Signal {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSignal();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.uid = reader.string();
          break;
        case 2:
          message.name = reader.string();
          break;
        case 3:
          message.description = reader.string();
          break;
        case 10:
          message.traits.push(SignalTrait.decode(reader, reader.uint32()));
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Signal {
    return {
      uid: isSet(object.uid) ? String(object.uid) : "",
      name: isSet(object.name) ? String(object.name) : "",
      description: isSet(object.description) ? String(object.description) : "",
      traits: Array.isArray(object?.traits)
        ? object.traits.map((e: any) => SignalTrait.fromJSON(e))
        : [],
    };
  },

  toJSON(message: Signal): unknown {
    const obj: any = {};
    message.uid !== undefined && (obj.uid = message.uid);
    message.name !== undefined && (obj.name = message.name);
    message.description !== undefined &&
      (obj.description = message.description);
    if (message.traits) {
      obj.traits = message.traits.map((e) =>
        e ? SignalTrait.toJSON(e) : undefined
      );
    } else {
      obj.traits = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<Signal>, I>>(object: I): Signal {
    const message = createBaseSignal();
    message.uid = object.uid ?? "";
    message.name = object.name ?? "";
    message.description = object.description ?? "";
    message.traits =
      object.traits?.map((e) => SignalTrait.fromPartial(e)) || [];
    return message;
  },
};

function createBaseSignalTrait(): SignalTrait {
  return {
    sensitive: undefined,
    timeSeries: undefined,
    entityReference: undefined,
  };
}

export const SignalTrait = {
  encode(
    message: SignalTrait,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.sensitive !== undefined) {
      SensitiveDataTrait.encode(
        message.sensitive,
        writer.uint32(10).fork()
      ).ldelim();
    }
    if (message.timeSeries !== undefined) {
      TimeSeriesTrait.encode(
        message.timeSeries,
        writer.uint32(18).fork()
      ).ldelim();
    }
    if (message.entityReference !== undefined) {
      EntityReferenceTrait.encode(
        message.entityReference,
        writer.uint32(26).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): SignalTrait {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSignalTrait();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.sensitive = SensitiveDataTrait.decode(
            reader,
            reader.uint32()
          );
          break;
        case 2:
          message.timeSeries = TimeSeriesTrait.decode(reader, reader.uint32());
          break;
        case 3:
          message.entityReference = EntityReferenceTrait.decode(
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

  fromJSON(object: any): SignalTrait {
    return {
      sensitive: isSet(object.sensitive)
        ? SensitiveDataTrait.fromJSON(object.sensitive)
        : undefined,
      timeSeries: isSet(object.timeSeries)
        ? TimeSeriesTrait.fromJSON(object.timeSeries)
        : undefined,
      entityReference: isSet(object.entityReference)
        ? EntityReferenceTrait.fromJSON(object.entityReference)
        : undefined,
    };
  },

  toJSON(message: SignalTrait): unknown {
    const obj: any = {};
    message.sensitive !== undefined &&
      (obj.sensitive = message.sensitive
        ? SensitiveDataTrait.toJSON(message.sensitive)
        : undefined);
    message.timeSeries !== undefined &&
      (obj.timeSeries = message.timeSeries
        ? TimeSeriesTrait.toJSON(message.timeSeries)
        : undefined);
    message.entityReference !== undefined &&
      (obj.entityReference = message.entityReference
        ? EntityReferenceTrait.toJSON(message.entityReference)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<SignalTrait>, I>>(
    object: I
  ): SignalTrait {
    const message = createBaseSignalTrait();
    message.sensitive =
      object.sensitive !== undefined && object.sensitive !== null
        ? SensitiveDataTrait.fromPartial(object.sensitive)
        : undefined;
    message.timeSeries =
      object.timeSeries !== undefined && object.timeSeries !== null
        ? TimeSeriesTrait.fromPartial(object.timeSeries)
        : undefined;
    message.entityReference =
      object.entityReference !== undefined && object.entityReference !== null
        ? EntityReferenceTrait.fromPartial(object.entityReference)
        : undefined;
    return message;
  },
};

function createBaseSensitiveDataTrait(): SensitiveDataTrait {
  return { level: "" };
}

export const SensitiveDataTrait = {
  encode(
    message: SensitiveDataTrait,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.level !== "") {
      writer.uint32(10).string(message.level);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): SensitiveDataTrait {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSensitiveDataTrait();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.level = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): SensitiveDataTrait {
    return {
      level: isSet(object.level) ? String(object.level) : "",
    };
  },

  toJSON(message: SensitiveDataTrait): unknown {
    const obj: any = {};
    message.level !== undefined && (obj.level = message.level);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<SensitiveDataTrait>, I>>(
    object: I
  ): SensitiveDataTrait {
    const message = createBaseSensitiveDataTrait();
    message.level = object.level ?? "";
    return message;
  },
};

function createBaseTimeSeriesTrait(): TimeSeriesTrait {
  return { level: "" };
}

export const TimeSeriesTrait = {
  encode(
    message: TimeSeriesTrait,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.level !== "") {
      writer.uint32(10).string(message.level);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): TimeSeriesTrait {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseTimeSeriesTrait();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.level = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): TimeSeriesTrait {
    return {
      level: isSet(object.level) ? String(object.level) : "",
    };
  },

  toJSON(message: TimeSeriesTrait): unknown {
    const obj: any = {};
    message.level !== undefined && (obj.level = message.level);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<TimeSeriesTrait>, I>>(
    object: I
  ): TimeSeriesTrait {
    const message = createBaseTimeSeriesTrait();
    message.level = object.level ?? "";
    return message;
  },
};

function createBaseEntityReferenceTrait(): EntityReferenceTrait {
  return { level: "" };
}

export const EntityReferenceTrait = {
  encode(
    message: EntityReferenceTrait,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.level !== "") {
      writer.uint32(10).string(message.level);
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): EntityReferenceTrait {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseEntityReferenceTrait();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.level = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): EntityReferenceTrait {
    return {
      level: isSet(object.level) ? String(object.level) : "",
    };
  },

  toJSON(message: EntityReferenceTrait): unknown {
    const obj: any = {};
    message.level !== undefined && (obj.level = message.level);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<EntityReferenceTrait>, I>>(
    object: I
  ): EntityReferenceTrait {
    const message = createBaseEntityReferenceTrait();
    message.level = object.level ?? "";
    return message;
  },
};

function createBaseSignalProvider(): SignalProvider {
  return {
    uid: "",
    name: "",
    description: "",
    signals: [],
    inputs: [],
    table: undefined,
    expression: undefined,
    model: undefined,
  };
}

export const SignalProvider = {
  encode(
    message: SignalProvider,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.uid !== "") {
      writer.uint32(10).string(message.uid);
    }
    if (message.name !== "") {
      writer.uint32(18).string(message.name);
    }
    if (message.description !== "") {
      writer.uint32(26).string(message.description);
    }
    for (const v of message.signals) {
      Signal.encode(v!, writer.uint32(34).fork()).ldelim();
    }
    for (const v of message.inputs) {
      Signal.encode(v!, writer.uint32(42).fork()).ldelim();
    }
    if (message.table !== undefined) {
      TableReference.encode(message.table, writer.uint32(802).fork()).ldelim();
    }
    if (message.expression !== undefined) {
      ExpressionReference.encode(
        message.expression,
        writer.uint32(810).fork()
      ).ldelim();
    }
    if (message.model !== undefined) {
      ModelReference.encode(message.model, writer.uint32(818).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): SignalProvider {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSignalProvider();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.uid = reader.string();
          break;
        case 2:
          message.name = reader.string();
          break;
        case 3:
          message.description = reader.string();
          break;
        case 4:
          message.signals.push(Signal.decode(reader, reader.uint32()));
          break;
        case 5:
          message.inputs.push(Signal.decode(reader, reader.uint32()));
          break;
        case 100:
          message.table = TableReference.decode(reader, reader.uint32());
          break;
        case 101:
          message.expression = ExpressionReference.decode(
            reader,
            reader.uint32()
          );
          break;
        case 102:
          message.model = ModelReference.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): SignalProvider {
    return {
      uid: isSet(object.uid) ? String(object.uid) : "",
      name: isSet(object.name) ? String(object.name) : "",
      description: isSet(object.description) ? String(object.description) : "",
      signals: Array.isArray(object?.signals)
        ? object.signals.map((e: any) => Signal.fromJSON(e))
        : [],
      inputs: Array.isArray(object?.inputs)
        ? object.inputs.map((e: any) => Signal.fromJSON(e))
        : [],
      table: isSet(object.table)
        ? TableReference.fromJSON(object.table)
        : undefined,
      expression: isSet(object.expression)
        ? ExpressionReference.fromJSON(object.expression)
        : undefined,
      model: isSet(object.model)
        ? ModelReference.fromJSON(object.model)
        : undefined,
    };
  },

  toJSON(message: SignalProvider): unknown {
    const obj: any = {};
    message.uid !== undefined && (obj.uid = message.uid);
    message.name !== undefined && (obj.name = message.name);
    message.description !== undefined &&
      (obj.description = message.description);
    if (message.signals) {
      obj.signals = message.signals.map((e) =>
        e ? Signal.toJSON(e) : undefined
      );
    } else {
      obj.signals = [];
    }
    if (message.inputs) {
      obj.inputs = message.inputs.map((e) =>
        e ? Signal.toJSON(e) : undefined
      );
    } else {
      obj.inputs = [];
    }
    message.table !== undefined &&
      (obj.table = message.table
        ? TableReference.toJSON(message.table)
        : undefined);
    message.expression !== undefined &&
      (obj.expression = message.expression
        ? ExpressionReference.toJSON(message.expression)
        : undefined);
    message.model !== undefined &&
      (obj.model = message.model
        ? ModelReference.toJSON(message.model)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<SignalProvider>, I>>(
    object: I
  ): SignalProvider {
    const message = createBaseSignalProvider();
    message.uid = object.uid ?? "";
    message.name = object.name ?? "";
    message.description = object.description ?? "";
    message.signals = object.signals?.map((e) => Signal.fromPartial(e)) || [];
    message.inputs = object.inputs?.map((e) => Signal.fromPartial(e)) || [];
    message.table =
      object.table !== undefined && object.table !== null
        ? TableReference.fromPartial(object.table)
        : undefined;
    message.expression =
      object.expression !== undefined && object.expression !== null
        ? ExpressionReference.fromPartial(object.expression)
        : undefined;
    message.model =
      object.model !== undefined && object.model !== null
        ? ModelReference.fromPartial(object.model)
        : undefined;
    return message;
  },
};

function createBaseSignalFrame(): SignalFrame {
  return { uid: "", name: "", description: "", providers: [] };
}

export const SignalFrame = {
  encode(
    message: SignalFrame,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.uid !== "") {
      writer.uint32(10).string(message.uid);
    }
    if (message.name !== "") {
      writer.uint32(18).string(message.name);
    }
    if (message.description !== "") {
      writer.uint32(26).string(message.description);
    }
    for (const v of message.providers) {
      SignalProvider.encode(v!, writer.uint32(34).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): SignalFrame {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSignalFrame();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.uid = reader.string();
          break;
        case 2:
          message.name = reader.string();
          break;
        case 3:
          message.description = reader.string();
          break;
        case 4:
          message.providers.push(
            SignalProvider.decode(reader, reader.uint32())
          );
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): SignalFrame {
    return {
      uid: isSet(object.uid) ? String(object.uid) : "",
      name: isSet(object.name) ? String(object.name) : "",
      description: isSet(object.description) ? String(object.description) : "",
      providers: Array.isArray(object?.providers)
        ? object.providers.map((e: any) => SignalProvider.fromJSON(e))
        : [],
    };
  },

  toJSON(message: SignalFrame): unknown {
    const obj: any = {};
    message.uid !== undefined && (obj.uid = message.uid);
    message.name !== undefined && (obj.name = message.name);
    message.description !== undefined &&
      (obj.description = message.description);
    if (message.providers) {
      obj.providers = message.providers.map((e) =>
        e ? SignalProvider.toJSON(e) : undefined
      );
    } else {
      obj.providers = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<SignalFrame>, I>>(
    object: I
  ): SignalFrame {
    const message = createBaseSignalFrame();
    message.uid = object.uid ?? "";
    message.name = object.name ?? "";
    message.description = object.description ?? "";
    message.providers =
      object.providers?.map((e) => SignalProvider.fromPartial(e)) || [];
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
