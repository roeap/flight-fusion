/* eslint-disable */
import * as Long from "long";
import * as _m0 from "protobufjs/minimal";

export const protobufPackage = "mlflow";

/** Visibility defines who is allowed to use the RPC. */
export enum Visibility {
  /** PUBLIC - Public indicates visible to both external and internal customers. */
  PUBLIC = 1,
  /** INTERNAL - Internal is only available to Databricks-internal clients. */
  INTERNAL = 2,
  /**
   * PUBLIC_UNDOCUMENTED - Public-undocumented are accessible via public endpoints, but not documented. This is useful
   * for internal clients that depend on public endpoints (e.g. workflows running in the driver).
   */
  PUBLIC_UNDOCUMENTED = 3,
  UNRECOGNIZED = -1,
}

export function visibilityFromJSON(object: any): Visibility {
  switch (object) {
    case 1:
    case "PUBLIC":
      return Visibility.PUBLIC;
    case 2:
    case "INTERNAL":
      return Visibility.INTERNAL;
    case 3:
    case "PUBLIC_UNDOCUMENTED":
      return Visibility.PUBLIC_UNDOCUMENTED;
    case -1:
    case "UNRECOGNIZED":
    default:
      return Visibility.UNRECOGNIZED;
  }
}

export function visibilityToJSON(object: Visibility): string {
  switch (object) {
    case Visibility.PUBLIC:
      return "PUBLIC";
    case Visibility.INTERNAL:
      return "INTERNAL";
    case Visibility.PUBLIC_UNDOCUMENTED:
      return "PUBLIC_UNDOCUMENTED";
    default:
      return "UNKNOWN";
  }
}

export enum ErrorCode {
  /**
   * INTERNAL_ERROR - Internal, system-level error codes, which generally cannot be resolved by the user, but
   * instead are due to service issues.
   *
   * Generic internal error occurred.
   */
  INTERNAL_ERROR = 1,
  /** TEMPORARILY_UNAVAILABLE - An internal system could not be contacted due to a period of unavailability. */
  TEMPORARILY_UNAVAILABLE = 2,
  /** IO_ERROR - Indicates that an IOException has been internally thrown. */
  IO_ERROR = 3,
  /** BAD_REQUEST - The request is invalid. */
  BAD_REQUEST = 4,
  /**
   * INVALID_PARAMETER_VALUE - Common application-level error codes, which were caused by the user input but may be returned
   * by multiple services.
   *
   * Supplied value for a parameter was invalid (e.g., giving a number for a string parameter).
   */
  INVALID_PARAMETER_VALUE = 1000,
  /** ENDPOINT_NOT_FOUND - Indicates that the given API endpoint does not exist. */
  ENDPOINT_NOT_FOUND = 1001,
  /** MALFORMED_REQUEST - Indicates that the given API request was malformed. */
  MALFORMED_REQUEST = 1002,
  /** INVALID_STATE - If one or more of the inputs to a given RPC are not in a valid state for the action. */
  INVALID_STATE = 1003,
  /** PERMISSION_DENIED - If a given user/entity doesn't have the required permission(s) to perform an action */
  PERMISSION_DENIED = 1004,
  /** FEATURE_DISABLED - If a given user/entity is trying to use a feature which has been disabled */
  FEATURE_DISABLED = 1005,
  /** CUSTOMER_UNAUTHORIZED - If customer-provided credentials are not authorized to perform an operation */
  CUSTOMER_UNAUTHORIZED = 1006,
  /** REQUEST_LIMIT_EXCEEDED - If the API request is rejected due to throttling */
  REQUEST_LIMIT_EXCEEDED = 1007,
  /** INVALID_STATE_TRANSITION - If the user attempts to perform an invalid state transition on a shard. */
  INVALID_STATE_TRANSITION = 2001,
  /** COULD_NOT_ACQUIRE_LOCK - Unable to perform the operation because the shard was locked by some other operation. */
  COULD_NOT_ACQUIRE_LOCK = 2002,
  /** RESOURCE_ALREADY_EXISTS - Operation was performed on a resource that already exists. */
  RESOURCE_ALREADY_EXISTS = 3001,
  /** RESOURCE_DOES_NOT_EXIST - Operation was performed on a resource that does not exist. */
  RESOURCE_DOES_NOT_EXIST = 3002,
  QUOTA_EXCEEDED = 4001,
  MAX_BLOCK_SIZE_EXCEEDED = 4002,
  MAX_READ_SIZE_EXCEEDED = 4003,
  DRY_RUN_FAILED = 5001,
  /** RESOURCE_LIMIT_EXCEEDED - Cluster request was rejected because it would exceed a resource limit. */
  RESOURCE_LIMIT_EXCEEDED = 5002,
  DIRECTORY_NOT_EMPTY = 6001,
  DIRECTORY_PROTECTED = 6002,
  MAX_NOTEBOOK_SIZE_EXCEEDED = 6003,
  UNRECOGNIZED = -1,
}

export function errorCodeFromJSON(object: any): ErrorCode {
  switch (object) {
    case 1:
    case "INTERNAL_ERROR":
      return ErrorCode.INTERNAL_ERROR;
    case 2:
    case "TEMPORARILY_UNAVAILABLE":
      return ErrorCode.TEMPORARILY_UNAVAILABLE;
    case 3:
    case "IO_ERROR":
      return ErrorCode.IO_ERROR;
    case 4:
    case "BAD_REQUEST":
      return ErrorCode.BAD_REQUEST;
    case 1000:
    case "INVALID_PARAMETER_VALUE":
      return ErrorCode.INVALID_PARAMETER_VALUE;
    case 1001:
    case "ENDPOINT_NOT_FOUND":
      return ErrorCode.ENDPOINT_NOT_FOUND;
    case 1002:
    case "MALFORMED_REQUEST":
      return ErrorCode.MALFORMED_REQUEST;
    case 1003:
    case "INVALID_STATE":
      return ErrorCode.INVALID_STATE;
    case 1004:
    case "PERMISSION_DENIED":
      return ErrorCode.PERMISSION_DENIED;
    case 1005:
    case "FEATURE_DISABLED":
      return ErrorCode.FEATURE_DISABLED;
    case 1006:
    case "CUSTOMER_UNAUTHORIZED":
      return ErrorCode.CUSTOMER_UNAUTHORIZED;
    case 1007:
    case "REQUEST_LIMIT_EXCEEDED":
      return ErrorCode.REQUEST_LIMIT_EXCEEDED;
    case 2001:
    case "INVALID_STATE_TRANSITION":
      return ErrorCode.INVALID_STATE_TRANSITION;
    case 2002:
    case "COULD_NOT_ACQUIRE_LOCK":
      return ErrorCode.COULD_NOT_ACQUIRE_LOCK;
    case 3001:
    case "RESOURCE_ALREADY_EXISTS":
      return ErrorCode.RESOURCE_ALREADY_EXISTS;
    case 3002:
    case "RESOURCE_DOES_NOT_EXIST":
      return ErrorCode.RESOURCE_DOES_NOT_EXIST;
    case 4001:
    case "QUOTA_EXCEEDED":
      return ErrorCode.QUOTA_EXCEEDED;
    case 4002:
    case "MAX_BLOCK_SIZE_EXCEEDED":
      return ErrorCode.MAX_BLOCK_SIZE_EXCEEDED;
    case 4003:
    case "MAX_READ_SIZE_EXCEEDED":
      return ErrorCode.MAX_READ_SIZE_EXCEEDED;
    case 5001:
    case "DRY_RUN_FAILED":
      return ErrorCode.DRY_RUN_FAILED;
    case 5002:
    case "RESOURCE_LIMIT_EXCEEDED":
      return ErrorCode.RESOURCE_LIMIT_EXCEEDED;
    case 6001:
    case "DIRECTORY_NOT_EMPTY":
      return ErrorCode.DIRECTORY_NOT_EMPTY;
    case 6002:
    case "DIRECTORY_PROTECTED":
      return ErrorCode.DIRECTORY_PROTECTED;
    case 6003:
    case "MAX_NOTEBOOK_SIZE_EXCEEDED":
      return ErrorCode.MAX_NOTEBOOK_SIZE_EXCEEDED;
    case -1:
    case "UNRECOGNIZED":
    default:
      return ErrorCode.UNRECOGNIZED;
  }
}

export function errorCodeToJSON(object: ErrorCode): string {
  switch (object) {
    case ErrorCode.INTERNAL_ERROR:
      return "INTERNAL_ERROR";
    case ErrorCode.TEMPORARILY_UNAVAILABLE:
      return "TEMPORARILY_UNAVAILABLE";
    case ErrorCode.IO_ERROR:
      return "IO_ERROR";
    case ErrorCode.BAD_REQUEST:
      return "BAD_REQUEST";
    case ErrorCode.INVALID_PARAMETER_VALUE:
      return "INVALID_PARAMETER_VALUE";
    case ErrorCode.ENDPOINT_NOT_FOUND:
      return "ENDPOINT_NOT_FOUND";
    case ErrorCode.MALFORMED_REQUEST:
      return "MALFORMED_REQUEST";
    case ErrorCode.INVALID_STATE:
      return "INVALID_STATE";
    case ErrorCode.PERMISSION_DENIED:
      return "PERMISSION_DENIED";
    case ErrorCode.FEATURE_DISABLED:
      return "FEATURE_DISABLED";
    case ErrorCode.CUSTOMER_UNAUTHORIZED:
      return "CUSTOMER_UNAUTHORIZED";
    case ErrorCode.REQUEST_LIMIT_EXCEEDED:
      return "REQUEST_LIMIT_EXCEEDED";
    case ErrorCode.INVALID_STATE_TRANSITION:
      return "INVALID_STATE_TRANSITION";
    case ErrorCode.COULD_NOT_ACQUIRE_LOCK:
      return "COULD_NOT_ACQUIRE_LOCK";
    case ErrorCode.RESOURCE_ALREADY_EXISTS:
      return "RESOURCE_ALREADY_EXISTS";
    case ErrorCode.RESOURCE_DOES_NOT_EXIST:
      return "RESOURCE_DOES_NOT_EXIST";
    case ErrorCode.QUOTA_EXCEEDED:
      return "QUOTA_EXCEEDED";
    case ErrorCode.MAX_BLOCK_SIZE_EXCEEDED:
      return "MAX_BLOCK_SIZE_EXCEEDED";
    case ErrorCode.MAX_READ_SIZE_EXCEEDED:
      return "MAX_READ_SIZE_EXCEEDED";
    case ErrorCode.DRY_RUN_FAILED:
      return "DRY_RUN_FAILED";
    case ErrorCode.RESOURCE_LIMIT_EXCEEDED:
      return "RESOURCE_LIMIT_EXCEEDED";
    case ErrorCode.DIRECTORY_NOT_EMPTY:
      return "DIRECTORY_NOT_EMPTY";
    case ErrorCode.DIRECTORY_PROTECTED:
      return "DIRECTORY_PROTECTED";
    case ErrorCode.MAX_NOTEBOOK_SIZE_EXCEEDED:
      return "MAX_NOTEBOOK_SIZE_EXCEEDED";
    default:
      return "UNKNOWN";
  }
}

/**
 * Defines the set of options declared for every service RPC which are used to
 * direct RPCs to endpoints, as well as other metadata about the RPC.
 */
export interface DatabricksRpcOptions {
  endpoints: HttpEndpoint[];
  /** Indicates which users are allowed to initiate this RPC. */
  visibility: Visibility;
  /**
   * Complete definition of all error codes (from a statically defined set) which this method
   * may return.
   */
  errorCodes: ErrorCode[];
  /** If defined, a rate limit will be applied to this RPC for all requests from the API proxy. */
  rateLimit: RateLimit | undefined;
  /**
   * If defined, overrides the default title used for in the API docs. See ProtobufDocGenerator
   * for more info.
   */
  rpcDocTitle: string;
}

export interface HttpEndpoint {
  /** HTTP method like POST or GET. */
  method: string;
  /** Conceptual path of the API, like "/clusters" or "/clusters/create". Should start with a slash. */
  path: string;
  /**
   * A version like 1.1 which is prepended to the URL (e.g., GET /1.1/clusters).
   * Breaking changes to an RPC must use a different version number.
   */
  since: ApiVersion | undefined;
}

export interface ApiVersion {
  major: number;
  minor: number;
}

/**
 * API rate limits applied to RPCs coming from the API Proxy. The rate limits are applied on a
 * per organization basis.
 */
export interface RateLimit {
  /**
   * The maximum burst of API requests allowed for a single endpoint. In the context of the
   * token bucket algorithm, this constant represents the total capacity of the token bucket.
   */
  maxBurst: number;
  /**
   * The maximum sustained request per second limit for a single endpoint. In the context of the,
   * token bucket algorithm, this constant represents the rate at which the token bucket fills.
   */
  maxSustainedPerSecond: number;
}

/** A block of documentation that is added to the AST after parsing the original protocol buffer. */
export interface DocumentationMetadata {
  /** The string of documentation attached to this particular item. */
  docstring: string;
  /**
   * The string of documentation that is *before* this item. This only makes sense for top-level
   * items such as (top-level) messages, (top-level) enumerations, or services. In all other
   * cases, this string is empty.
   */
  leadDoc: string;
  /**
   * The visibility level when the docstring was generated.
   * The documentation extractor builds multiple versions of the documentation, one for each
   * visibility level. The documentation is then generated for each visibility level.
   */
  visibility: Visibility;
  /**
   * The original proto path in the internal representation. This is useful when performing field
   * flattening to figure out what the original field was.
   * One example is ["jobs","Run","original_attempt_run_id"] for jobs.
   * This path is unique.
   */
  originalProtoPath: string[];
  /**
   * The location (line number) of the start of the documentation. This is required to keep the
   * pieces of documentation sorted.
   */
  position: number;
}

/** Serialization format for DatabricksServiceException. */
export interface DatabricksServiceExceptionProto {
  errorCode: ErrorCode;
  message: string;
  stackTrace: string;
}

function createBaseDatabricksRpcOptions(): DatabricksRpcOptions {
  return {
    endpoints: [],
    visibility: 1,
    errorCodes: [],
    rateLimit: undefined,
    rpcDocTitle: "",
  };
}

export const DatabricksRpcOptions = {
  encode(
    message: DatabricksRpcOptions,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    for (const v of message.endpoints) {
      HttpEndpoint.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    if (message.visibility !== 1) {
      writer.uint32(16).int32(message.visibility);
    }
    writer.uint32(26).fork();
    for (const v of message.errorCodes) {
      writer.int32(v);
    }
    writer.ldelim();
    if (message.rateLimit !== undefined) {
      RateLimit.encode(message.rateLimit, writer.uint32(34).fork()).ldelim();
    }
    if (message.rpcDocTitle !== "") {
      writer.uint32(42).string(message.rpcDocTitle);
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): DatabricksRpcOptions {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDatabricksRpcOptions();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.endpoints.push(HttpEndpoint.decode(reader, reader.uint32()));
          break;
        case 2:
          message.visibility = reader.int32() as any;
          break;
        case 3:
          if ((tag & 7) === 2) {
            const end2 = reader.uint32() + reader.pos;
            while (reader.pos < end2) {
              message.errorCodes.push(reader.int32() as any);
            }
          } else {
            message.errorCodes.push(reader.int32() as any);
          }
          break;
        case 4:
          message.rateLimit = RateLimit.decode(reader, reader.uint32());
          break;
        case 5:
          message.rpcDocTitle = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): DatabricksRpcOptions {
    return {
      endpoints: Array.isArray(object?.endpoints)
        ? object.endpoints.map((e: any) => HttpEndpoint.fromJSON(e))
        : [],
      visibility: isSet(object.visibility)
        ? visibilityFromJSON(object.visibility)
        : 1,
      errorCodes: Array.isArray(object?.errorCodes)
        ? object.errorCodes.map((e: any) => errorCodeFromJSON(e))
        : [],
      rateLimit: isSet(object.rateLimit)
        ? RateLimit.fromJSON(object.rateLimit)
        : undefined,
      rpcDocTitle: isSet(object.rpcDocTitle) ? String(object.rpcDocTitle) : "",
    };
  },

  toJSON(message: DatabricksRpcOptions): unknown {
    const obj: any = {};
    if (message.endpoints) {
      obj.endpoints = message.endpoints.map((e) =>
        e ? HttpEndpoint.toJSON(e) : undefined
      );
    } else {
      obj.endpoints = [];
    }
    message.visibility !== undefined &&
      (obj.visibility = visibilityToJSON(message.visibility));
    if (message.errorCodes) {
      obj.errorCodes = message.errorCodes.map((e) => errorCodeToJSON(e));
    } else {
      obj.errorCodes = [];
    }
    message.rateLimit !== undefined &&
      (obj.rateLimit = message.rateLimit
        ? RateLimit.toJSON(message.rateLimit)
        : undefined);
    message.rpcDocTitle !== undefined &&
      (obj.rpcDocTitle = message.rpcDocTitle);
    return obj;
  },

  fromPartial(object: DeepPartial<DatabricksRpcOptions>): DatabricksRpcOptions {
    const message = createBaseDatabricksRpcOptions();
    message.endpoints =
      object.endpoints?.map((e) => HttpEndpoint.fromPartial(e)) || [];
    message.visibility = object.visibility ?? 1;
    message.errorCodes = object.errorCodes?.map((e) => e) || [];
    message.rateLimit =
      object.rateLimit !== undefined && object.rateLimit !== null
        ? RateLimit.fromPartial(object.rateLimit)
        : undefined;
    message.rpcDocTitle = object.rpcDocTitle ?? "";
    return message;
  },
};

function createBaseHttpEndpoint(): HttpEndpoint {
  return { method: "", path: "", since: undefined };
}

export const HttpEndpoint = {
  encode(
    message: HttpEndpoint,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.method !== "") {
      writer.uint32(10).string(message.method);
    }
    if (message.path !== "") {
      writer.uint32(18).string(message.path);
    }
    if (message.since !== undefined) {
      ApiVersion.encode(message.since, writer.uint32(26).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): HttpEndpoint {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseHttpEndpoint();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.method = reader.string();
          break;
        case 2:
          message.path = reader.string();
          break;
        case 3:
          message.since = ApiVersion.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): HttpEndpoint {
    return {
      method: isSet(object.method) ? String(object.method) : "",
      path: isSet(object.path) ? String(object.path) : "",
      since: isSet(object.since)
        ? ApiVersion.fromJSON(object.since)
        : undefined,
    };
  },

  toJSON(message: HttpEndpoint): unknown {
    const obj: any = {};
    message.method !== undefined && (obj.method = message.method);
    message.path !== undefined && (obj.path = message.path);
    message.since !== undefined &&
      (obj.since = message.since
        ? ApiVersion.toJSON(message.since)
        : undefined);
    return obj;
  },

  fromPartial(object: DeepPartial<HttpEndpoint>): HttpEndpoint {
    const message = createBaseHttpEndpoint();
    message.method = object.method ?? "";
    message.path = object.path ?? "";
    message.since =
      object.since !== undefined && object.since !== null
        ? ApiVersion.fromPartial(object.since)
        : undefined;
    return message;
  },
};

function createBaseApiVersion(): ApiVersion {
  return { major: 0, minor: 0 };
}

export const ApiVersion = {
  encode(
    message: ApiVersion,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.major !== 0) {
      writer.uint32(8).int32(message.major);
    }
    if (message.minor !== 0) {
      writer.uint32(16).int32(message.minor);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ApiVersion {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseApiVersion();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.major = reader.int32();
          break;
        case 2:
          message.minor = reader.int32();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ApiVersion {
    return {
      major: isSet(object.major) ? Number(object.major) : 0,
      minor: isSet(object.minor) ? Number(object.minor) : 0,
    };
  },

  toJSON(message: ApiVersion): unknown {
    const obj: any = {};
    message.major !== undefined && (obj.major = Math.round(message.major));
    message.minor !== undefined && (obj.minor = Math.round(message.minor));
    return obj;
  },

  fromPartial(object: DeepPartial<ApiVersion>): ApiVersion {
    const message = createBaseApiVersion();
    message.major = object.major ?? 0;
    message.minor = object.minor ?? 0;
    return message;
  },
};

function createBaseRateLimit(): RateLimit {
  return { maxBurst: 0, maxSustainedPerSecond: 0 };
}

export const RateLimit = {
  encode(
    message: RateLimit,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.maxBurst !== 0) {
      writer.uint32(8).int64(message.maxBurst);
    }
    if (message.maxSustainedPerSecond !== 0) {
      writer.uint32(16).int64(message.maxSustainedPerSecond);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): RateLimit {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRateLimit();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.maxBurst = longToNumber(reader.int64() as Long);
          break;
        case 2:
          message.maxSustainedPerSecond = longToNumber(reader.int64() as Long);
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): RateLimit {
    return {
      maxBurst: isSet(object.maxBurst) ? Number(object.maxBurst) : 0,
      maxSustainedPerSecond: isSet(object.maxSustainedPerSecond)
        ? Number(object.maxSustainedPerSecond)
        : 0,
    };
  },

  toJSON(message: RateLimit): unknown {
    const obj: any = {};
    message.maxBurst !== undefined &&
      (obj.maxBurst = Math.round(message.maxBurst));
    message.maxSustainedPerSecond !== undefined &&
      (obj.maxSustainedPerSecond = Math.round(message.maxSustainedPerSecond));
    return obj;
  },

  fromPartial(object: DeepPartial<RateLimit>): RateLimit {
    const message = createBaseRateLimit();
    message.maxBurst = object.maxBurst ?? 0;
    message.maxSustainedPerSecond = object.maxSustainedPerSecond ?? 0;
    return message;
  },
};

function createBaseDocumentationMetadata(): DocumentationMetadata {
  return {
    docstring: "",
    leadDoc: "",
    visibility: 1,
    originalProtoPath: [],
    position: 0,
  };
}

export const DocumentationMetadata = {
  encode(
    message: DocumentationMetadata,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.docstring !== "") {
      writer.uint32(10).string(message.docstring);
    }
    if (message.leadDoc !== "") {
      writer.uint32(18).string(message.leadDoc);
    }
    if (message.visibility !== 1) {
      writer.uint32(24).int32(message.visibility);
    }
    for (const v of message.originalProtoPath) {
      writer.uint32(34).string(v!);
    }
    if (message.position !== 0) {
      writer.uint32(40).int32(message.position);
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): DocumentationMetadata {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDocumentationMetadata();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.docstring = reader.string();
          break;
        case 2:
          message.leadDoc = reader.string();
          break;
        case 3:
          message.visibility = reader.int32() as any;
          break;
        case 4:
          message.originalProtoPath.push(reader.string());
          break;
        case 5:
          message.position = reader.int32();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): DocumentationMetadata {
    return {
      docstring: isSet(object.docstring) ? String(object.docstring) : "",
      leadDoc: isSet(object.leadDoc) ? String(object.leadDoc) : "",
      visibility: isSet(object.visibility)
        ? visibilityFromJSON(object.visibility)
        : 1,
      originalProtoPath: Array.isArray(object?.originalProtoPath)
        ? object.originalProtoPath.map((e: any) => String(e))
        : [],
      position: isSet(object.position) ? Number(object.position) : 0,
    };
  },

  toJSON(message: DocumentationMetadata): unknown {
    const obj: any = {};
    message.docstring !== undefined && (obj.docstring = message.docstring);
    message.leadDoc !== undefined && (obj.leadDoc = message.leadDoc);
    message.visibility !== undefined &&
      (obj.visibility = visibilityToJSON(message.visibility));
    if (message.originalProtoPath) {
      obj.originalProtoPath = message.originalProtoPath.map((e) => e);
    } else {
      obj.originalProtoPath = [];
    }
    message.position !== undefined &&
      (obj.position = Math.round(message.position));
    return obj;
  },

  fromPartial(
    object: DeepPartial<DocumentationMetadata>
  ): DocumentationMetadata {
    const message = createBaseDocumentationMetadata();
    message.docstring = object.docstring ?? "";
    message.leadDoc = object.leadDoc ?? "";
    message.visibility = object.visibility ?? 1;
    message.originalProtoPath = object.originalProtoPath?.map((e) => e) || [];
    message.position = object.position ?? 0;
    return message;
  },
};

function createBaseDatabricksServiceExceptionProto(): DatabricksServiceExceptionProto {
  return { errorCode: 1, message: "", stackTrace: "" };
}

export const DatabricksServiceExceptionProto = {
  encode(
    message: DatabricksServiceExceptionProto,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.errorCode !== 1) {
      writer.uint32(8).int32(message.errorCode);
    }
    if (message.message !== "") {
      writer.uint32(18).string(message.message);
    }
    if (message.stackTrace !== "") {
      writer.uint32(26).string(message.stackTrace);
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): DatabricksServiceExceptionProto {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDatabricksServiceExceptionProto();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.errorCode = reader.int32() as any;
          break;
        case 2:
          message.message = reader.string();
          break;
        case 3:
          message.stackTrace = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): DatabricksServiceExceptionProto {
    return {
      errorCode: isSet(object.errorCode)
        ? errorCodeFromJSON(object.errorCode)
        : 1,
      message: isSet(object.message) ? String(object.message) : "",
      stackTrace: isSet(object.stackTrace) ? String(object.stackTrace) : "",
    };
  },

  toJSON(message: DatabricksServiceExceptionProto): unknown {
    const obj: any = {};
    message.errorCode !== undefined &&
      (obj.errorCode = errorCodeToJSON(message.errorCode));
    message.message !== undefined && (obj.message = message.message);
    message.stackTrace !== undefined && (obj.stackTrace = message.stackTrace);
    return obj;
  },

  fromPartial(
    object: DeepPartial<DatabricksServiceExceptionProto>
  ): DatabricksServiceExceptionProto {
    const message = createBaseDatabricksServiceExceptionProto();
    message.errorCode = object.errorCode ?? 1;
    message.message = object.message ?? "";
    message.stackTrace = object.stackTrace ?? "";
    return message;
  },
};

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
