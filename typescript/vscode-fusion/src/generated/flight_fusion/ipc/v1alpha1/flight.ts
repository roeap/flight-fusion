/* eslint-disable */
import * as Long from "long";
import * as _m0 from "protobufjs/minimal";
import { AreaSourceReference } from "../../../flight_fusion/ipc/v1alpha1/common";
import {
  CommandSqlOperation,
  CommandKqlOperation,
  SignalFrameOperation,
  CommandReadDataset,
  CommandExecuteQuery,
  DeltaOperationRequest,
  CommandWriteIntoDataset,
  ResultDoPutUpdate,
  CommandRegisterSource,
  CommandDropSource,
  CommandSetMetadata,
  ResultActionStatus,
} from "../../../flight_fusion/ipc/v1alpha1/message";

export const protobufPackage = "flight_fusion.ipc.v1alpha1";

/** Wrappers around to commands and actions tha get passed to the Flight service. */

export interface FlightGetFlightInfoRequest {
  /** source identifier */
  source: AreaSourceReference | undefined;
}

/** Requests submitted against the `do_get` endpoint */
export interface FlightDoGetRequest {
  sql: CommandSqlOperation | undefined;
  kql: CommandKqlOperation | undefined;
  frame: SignalFrameOperation | undefined;
  /** Read data from a registered source */
  read: CommandReadDataset | undefined;
  /** Execute a query against a pre-defined context */
  query: CommandExecuteQuery | undefined;
  /** Perform a read operation against Delta table */
  delta: DeltaOperationRequest | undefined;
}

/** Requests submitted against the `do_put` endpoint */
export interface FlightDoPutRequest {
  /** Write data into a registered source */
  storage: CommandWriteIntoDataset | undefined;
  delta: DeltaOperationRequest | undefined;
}

/** Response recieved from `do_put` operations` */
export interface FlightDoPutResponse {
  /** statistics for data written to source */
  update: ResultDoPutUpdate | undefined;
}

/** Requests submitted against the `do_action` endpoint */
export interface FlightActionRequest {
  /** Register a new data source to service */
  register: CommandRegisterSource | undefined;
  /** command to remove a dataset from the area store */
  drop: CommandDropSource | undefined;
  /** Set the metadata for a data source */
  setMeta: CommandSetMetadata | undefined;
}

export interface FlightActionResponse {
  /** Result when actions reports its execution status */
  status: ResultActionStatus | undefined;
}

function createBaseFlightGetFlightInfoRequest(): FlightGetFlightInfoRequest {
  return { source: undefined };
}

export const FlightGetFlightInfoRequest = {
  encode(
    message: FlightGetFlightInfoRequest,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.source !== undefined) {
      AreaSourceReference.encode(
        message.source,
        writer.uint32(10).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): FlightGetFlightInfoRequest {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseFlightGetFlightInfoRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.source = AreaSourceReference.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): FlightGetFlightInfoRequest {
    return {
      source: isSet(object.source)
        ? AreaSourceReference.fromJSON(object.source)
        : undefined,
    };
  },

  toJSON(message: FlightGetFlightInfoRequest): unknown {
    const obj: any = {};
    message.source !== undefined &&
      (obj.source = message.source
        ? AreaSourceReference.toJSON(message.source)
        : undefined);
    return obj;
  },

  fromPartial(
    object: DeepPartial<FlightGetFlightInfoRequest>
  ): FlightGetFlightInfoRequest {
    const message = createBaseFlightGetFlightInfoRequest();
    message.source =
      object.source !== undefined && object.source !== null
        ? AreaSourceReference.fromPartial(object.source)
        : undefined;
    return message;
  },
};

function createBaseFlightDoGetRequest(): FlightDoGetRequest {
  return {
    sql: undefined,
    kql: undefined,
    frame: undefined,
    read: undefined,
    query: undefined,
    delta: undefined,
  };
}

export const FlightDoGetRequest = {
  encode(
    message: FlightDoGetRequest,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.sql !== undefined) {
      CommandSqlOperation.encode(
        message.sql,
        writer.uint32(10).fork()
      ).ldelim();
    }
    if (message.kql !== undefined) {
      CommandKqlOperation.encode(
        message.kql,
        writer.uint32(18).fork()
      ).ldelim();
    }
    if (message.frame !== undefined) {
      SignalFrameOperation.encode(
        message.frame,
        writer.uint32(26).fork()
      ).ldelim();
    }
    if (message.read !== undefined) {
      CommandReadDataset.encode(
        message.read,
        writer.uint32(34).fork()
      ).ldelim();
    }
    if (message.query !== undefined) {
      CommandExecuteQuery.encode(
        message.query,
        writer.uint32(42).fork()
      ).ldelim();
    }
    if (message.delta !== undefined) {
      DeltaOperationRequest.encode(
        message.delta,
        writer.uint32(50).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): FlightDoGetRequest {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseFlightDoGetRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.sql = CommandSqlOperation.decode(reader, reader.uint32());
          break;
        case 2:
          message.kql = CommandKqlOperation.decode(reader, reader.uint32());
          break;
        case 3:
          message.frame = SignalFrameOperation.decode(reader, reader.uint32());
          break;
        case 4:
          message.read = CommandReadDataset.decode(reader, reader.uint32());
          break;
        case 5:
          message.query = CommandExecuteQuery.decode(reader, reader.uint32());
          break;
        case 6:
          message.delta = DeltaOperationRequest.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): FlightDoGetRequest {
    return {
      sql: isSet(object.sql)
        ? CommandSqlOperation.fromJSON(object.sql)
        : undefined,
      kql: isSet(object.kql)
        ? CommandKqlOperation.fromJSON(object.kql)
        : undefined,
      frame: isSet(object.frame)
        ? SignalFrameOperation.fromJSON(object.frame)
        : undefined,
      read: isSet(object.read)
        ? CommandReadDataset.fromJSON(object.read)
        : undefined,
      query: isSet(object.query)
        ? CommandExecuteQuery.fromJSON(object.query)
        : undefined,
      delta: isSet(object.delta)
        ? DeltaOperationRequest.fromJSON(object.delta)
        : undefined,
    };
  },

  toJSON(message: FlightDoGetRequest): unknown {
    const obj: any = {};
    message.sql !== undefined &&
      (obj.sql = message.sql
        ? CommandSqlOperation.toJSON(message.sql)
        : undefined);
    message.kql !== undefined &&
      (obj.kql = message.kql
        ? CommandKqlOperation.toJSON(message.kql)
        : undefined);
    message.frame !== undefined &&
      (obj.frame = message.frame
        ? SignalFrameOperation.toJSON(message.frame)
        : undefined);
    message.read !== undefined &&
      (obj.read = message.read
        ? CommandReadDataset.toJSON(message.read)
        : undefined);
    message.query !== undefined &&
      (obj.query = message.query
        ? CommandExecuteQuery.toJSON(message.query)
        : undefined);
    message.delta !== undefined &&
      (obj.delta = message.delta
        ? DeltaOperationRequest.toJSON(message.delta)
        : undefined);
    return obj;
  },

  fromPartial(object: DeepPartial<FlightDoGetRequest>): FlightDoGetRequest {
    const message = createBaseFlightDoGetRequest();
    message.sql =
      object.sql !== undefined && object.sql !== null
        ? CommandSqlOperation.fromPartial(object.sql)
        : undefined;
    message.kql =
      object.kql !== undefined && object.kql !== null
        ? CommandKqlOperation.fromPartial(object.kql)
        : undefined;
    message.frame =
      object.frame !== undefined && object.frame !== null
        ? SignalFrameOperation.fromPartial(object.frame)
        : undefined;
    message.read =
      object.read !== undefined && object.read !== null
        ? CommandReadDataset.fromPartial(object.read)
        : undefined;
    message.query =
      object.query !== undefined && object.query !== null
        ? CommandExecuteQuery.fromPartial(object.query)
        : undefined;
    message.delta =
      object.delta !== undefined && object.delta !== null
        ? DeltaOperationRequest.fromPartial(object.delta)
        : undefined;
    return message;
  },
};

function createBaseFlightDoPutRequest(): FlightDoPutRequest {
  return { storage: undefined, delta: undefined };
}

export const FlightDoPutRequest = {
  encode(
    message: FlightDoPutRequest,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.storage !== undefined) {
      CommandWriteIntoDataset.encode(
        message.storage,
        writer.uint32(18).fork()
      ).ldelim();
    }
    if (message.delta !== undefined) {
      DeltaOperationRequest.encode(
        message.delta,
        writer.uint32(26).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): FlightDoPutRequest {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseFlightDoPutRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 2:
          message.storage = CommandWriteIntoDataset.decode(
            reader,
            reader.uint32()
          );
          break;
        case 3:
          message.delta = DeltaOperationRequest.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): FlightDoPutRequest {
    return {
      storage: isSet(object.storage)
        ? CommandWriteIntoDataset.fromJSON(object.storage)
        : undefined,
      delta: isSet(object.delta)
        ? DeltaOperationRequest.fromJSON(object.delta)
        : undefined,
    };
  },

  toJSON(message: FlightDoPutRequest): unknown {
    const obj: any = {};
    message.storage !== undefined &&
      (obj.storage = message.storage
        ? CommandWriteIntoDataset.toJSON(message.storage)
        : undefined);
    message.delta !== undefined &&
      (obj.delta = message.delta
        ? DeltaOperationRequest.toJSON(message.delta)
        : undefined);
    return obj;
  },

  fromPartial(object: DeepPartial<FlightDoPutRequest>): FlightDoPutRequest {
    const message = createBaseFlightDoPutRequest();
    message.storage =
      object.storage !== undefined && object.storage !== null
        ? CommandWriteIntoDataset.fromPartial(object.storage)
        : undefined;
    message.delta =
      object.delta !== undefined && object.delta !== null
        ? DeltaOperationRequest.fromPartial(object.delta)
        : undefined;
    return message;
  },
};

function createBaseFlightDoPutResponse(): FlightDoPutResponse {
  return { update: undefined };
}

export const FlightDoPutResponse = {
  encode(
    message: FlightDoPutResponse,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.update !== undefined) {
      ResultDoPutUpdate.encode(
        message.update,
        writer.uint32(10).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): FlightDoPutResponse {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseFlightDoPutResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.update = ResultDoPutUpdate.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): FlightDoPutResponse {
    return {
      update: isSet(object.update)
        ? ResultDoPutUpdate.fromJSON(object.update)
        : undefined,
    };
  },

  toJSON(message: FlightDoPutResponse): unknown {
    const obj: any = {};
    message.update !== undefined &&
      (obj.update = message.update
        ? ResultDoPutUpdate.toJSON(message.update)
        : undefined);
    return obj;
  },

  fromPartial(object: DeepPartial<FlightDoPutResponse>): FlightDoPutResponse {
    const message = createBaseFlightDoPutResponse();
    message.update =
      object.update !== undefined && object.update !== null
        ? ResultDoPutUpdate.fromPartial(object.update)
        : undefined;
    return message;
  },
};

function createBaseFlightActionRequest(): FlightActionRequest {
  return { register: undefined, drop: undefined, setMeta: undefined };
}

export const FlightActionRequest = {
  encode(
    message: FlightActionRequest,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.register !== undefined) {
      CommandRegisterSource.encode(
        message.register,
        writer.uint32(10).fork()
      ).ldelim();
    }
    if (message.drop !== undefined) {
      CommandDropSource.encode(message.drop, writer.uint32(18).fork()).ldelim();
    }
    if (message.setMeta !== undefined) {
      CommandSetMetadata.encode(
        message.setMeta,
        writer.uint32(26).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): FlightActionRequest {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseFlightActionRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.register = CommandRegisterSource.decode(
            reader,
            reader.uint32()
          );
          break;
        case 2:
          message.drop = CommandDropSource.decode(reader, reader.uint32());
          break;
        case 3:
          message.setMeta = CommandSetMetadata.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): FlightActionRequest {
    return {
      register: isSet(object.register)
        ? CommandRegisterSource.fromJSON(object.register)
        : undefined,
      drop: isSet(object.drop)
        ? CommandDropSource.fromJSON(object.drop)
        : undefined,
      setMeta: isSet(object.setMeta)
        ? CommandSetMetadata.fromJSON(object.setMeta)
        : undefined,
    };
  },

  toJSON(message: FlightActionRequest): unknown {
    const obj: any = {};
    message.register !== undefined &&
      (obj.register = message.register
        ? CommandRegisterSource.toJSON(message.register)
        : undefined);
    message.drop !== undefined &&
      (obj.drop = message.drop
        ? CommandDropSource.toJSON(message.drop)
        : undefined);
    message.setMeta !== undefined &&
      (obj.setMeta = message.setMeta
        ? CommandSetMetadata.toJSON(message.setMeta)
        : undefined);
    return obj;
  },

  fromPartial(object: DeepPartial<FlightActionRequest>): FlightActionRequest {
    const message = createBaseFlightActionRequest();
    message.register =
      object.register !== undefined && object.register !== null
        ? CommandRegisterSource.fromPartial(object.register)
        : undefined;
    message.drop =
      object.drop !== undefined && object.drop !== null
        ? CommandDropSource.fromPartial(object.drop)
        : undefined;
    message.setMeta =
      object.setMeta !== undefined && object.setMeta !== null
        ? CommandSetMetadata.fromPartial(object.setMeta)
        : undefined;
    return message;
  },
};

function createBaseFlightActionResponse(): FlightActionResponse {
  return { status: undefined };
}

export const FlightActionResponse = {
  encode(
    message: FlightActionResponse,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.status !== undefined) {
      ResultActionStatus.encode(
        message.status,
        writer.uint32(10).fork()
      ).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): FlightActionResponse {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseFlightActionResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.status = ResultActionStatus.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): FlightActionResponse {
    return {
      status: isSet(object.status)
        ? ResultActionStatus.fromJSON(object.status)
        : undefined,
    };
  },

  toJSON(message: FlightActionResponse): unknown {
    const obj: any = {};
    message.status !== undefined &&
      (obj.status = message.status
        ? ResultActionStatus.toJSON(message.status)
        : undefined);
    return obj;
  },

  fromPartial(object: DeepPartial<FlightActionResponse>): FlightActionResponse {
    const message = createBaseFlightActionResponse();
    message.status =
      object.status !== undefined && object.status !== null
        ? ResultActionStatus.fromPartial(object.status)
        : undefined;
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
