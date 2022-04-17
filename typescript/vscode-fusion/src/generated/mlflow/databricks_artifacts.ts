/* eslint-disable */
import * as Long from "long";
import * as _m0 from "protobufjs/minimal";

export const protobufPackage = "mlflow";

/** The type of a given artifact access credential */
export enum ArtifactCredentialType {
  /**
   * AZURE_SAS_URI - The credential is an Azure Shared Access Signature URI. For more information, see
   * https://docs.microsoft.com/en-us/azure/storage/common/storage-sas-overview
   */
  AZURE_SAS_URI = 1,
  /**
   * AWS_PRESIGNED_URL - The credential is an AWS Presigned URL. For more information, see
   * https://docs.aws.amazon.com/AmazonS3/latest/dev/ShareObjectPreSignedURL.html
   */
  AWS_PRESIGNED_URL = 2,
  /**
   * GCP_SIGNED_URL - The credential is a GCP Signed URL. For more information, see
   * https://cloud.google.com/storage/docs/access-control/signed-urls
   */
  GCP_SIGNED_URL = 3,
  UNRECOGNIZED = -1,
}

export function artifactCredentialTypeFromJSON(
  object: any
): ArtifactCredentialType {
  switch (object) {
    case 1:
    case "AZURE_SAS_URI":
      return ArtifactCredentialType.AZURE_SAS_URI;
    case 2:
    case "AWS_PRESIGNED_URL":
      return ArtifactCredentialType.AWS_PRESIGNED_URL;
    case 3:
    case "GCP_SIGNED_URL":
      return ArtifactCredentialType.GCP_SIGNED_URL;
    case -1:
    case "UNRECOGNIZED":
    default:
      return ArtifactCredentialType.UNRECOGNIZED;
  }
}

export function artifactCredentialTypeToJSON(
  object: ArtifactCredentialType
): string {
  switch (object) {
    case ArtifactCredentialType.AZURE_SAS_URI:
      return "AZURE_SAS_URI";
    case ArtifactCredentialType.AWS_PRESIGNED_URL:
      return "AWS_PRESIGNED_URL";
    case ArtifactCredentialType.GCP_SIGNED_URL:
      return "GCP_SIGNED_URL";
    default:
      return "UNKNOWN";
  }
}

export interface ArtifactCredentialInfo {
  /**
   * The ID of the MLflow Run containing the artifact that can be accessed
   * with the credential
   */
  runId: string;
  /**
   * The path, relative to the Run's artifact root location, of the artifact
   * that can be accessed with the credential
   */
  path: string;
  /** The signed URI credential that provides access to the artifact */
  signedUri: string;
  /**
   * A collection of HTTP headers that should be specified when uploading to
   * or downloading from the specified `signed_uri`
   */
  headers: ArtifactCredentialInfo_HttpHeader[];
  /**
   * The type of the signed credential URI (e.g., an AWS presigned URL
   * or an Azure Shared Access Signature URI)
   */
  type: ArtifactCredentialType;
}

export interface ArtifactCredentialInfo_HttpHeader {
  /** The HTTP header name */
  name: string;
  /** The HTTP header value */
  value: string;
}

export interface GetCredentialsForRead {
  /** The ID of the MLflow Run for which to fetch artifact read credentials */
  runId: string;
  /**
   * The artifact paths, relative to the Run's artifact root location, for which to
   * fetch artifact read credentials. Must not be empty.
   */
  path: string[];
  /** Token specifying the page of credentials to fetch for large requests that require pagination */
  pageToken: string;
}

export interface GetCredentialsForRead_Response {
  /** Credentials for reading from the specified artifact locations */
  credentialInfos: ArtifactCredentialInfo[];
  /** Token used to fetch the next page of credentials for large requests that require pagination */
  nextPageToken: string;
}

export interface GetCredentialsForWrite {
  /** The ID of the MLflow Run for which to fetch artifact write credentials */
  runId: string;
  /**
   * The artifact paths, relative to the Run's artifact root location, for which to
   * fetch artifact write credentials. Must not be empty.
   */
  path: string[];
  /** Token specifying the page of credentials to fetch for large requests that require pagination */
  pageToken: string;
}

export interface GetCredentialsForWrite_Response {
  /** Credentials for writing to the specified artifact locations */
  credentialInfos: ArtifactCredentialInfo[];
  /** Token used to fetch the next page of credentials for large requests that require pagination */
  nextPageToken: string;
}

function createBaseArtifactCredentialInfo(): ArtifactCredentialInfo {
  return { runId: "", path: "", signedUri: "", headers: [], type: 1 };
}

export const ArtifactCredentialInfo = {
  encode(
    message: ArtifactCredentialInfo,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.runId !== "") {
      writer.uint32(10).string(message.runId);
    }
    if (message.path !== "") {
      writer.uint32(18).string(message.path);
    }
    if (message.signedUri !== "") {
      writer.uint32(26).string(message.signedUri);
    }
    for (const v of message.headers) {
      ArtifactCredentialInfo_HttpHeader.encode(
        v!,
        writer.uint32(34).fork()
      ).ldelim();
    }
    if (message.type !== 1) {
      writer.uint32(40).int32(message.type);
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ArtifactCredentialInfo {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseArtifactCredentialInfo();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.runId = reader.string();
          break;
        case 2:
          message.path = reader.string();
          break;
        case 3:
          message.signedUri = reader.string();
          break;
        case 4:
          message.headers.push(
            ArtifactCredentialInfo_HttpHeader.decode(reader, reader.uint32())
          );
          break;
        case 5:
          message.type = reader.int32() as any;
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ArtifactCredentialInfo {
    return {
      runId: isSet(object.runId) ? String(object.runId) : "",
      path: isSet(object.path) ? String(object.path) : "",
      signedUri: isSet(object.signedUri) ? String(object.signedUri) : "",
      headers: Array.isArray(object?.headers)
        ? object.headers.map((e: any) =>
            ArtifactCredentialInfo_HttpHeader.fromJSON(e)
          )
        : [],
      type: isSet(object.type)
        ? artifactCredentialTypeFromJSON(object.type)
        : 1,
    };
  },

  toJSON(message: ArtifactCredentialInfo): unknown {
    const obj: any = {};
    message.runId !== undefined && (obj.runId = message.runId);
    message.path !== undefined && (obj.path = message.path);
    message.signedUri !== undefined && (obj.signedUri = message.signedUri);
    if (message.headers) {
      obj.headers = message.headers.map((e) =>
        e ? ArtifactCredentialInfo_HttpHeader.toJSON(e) : undefined
      );
    } else {
      obj.headers = [];
    }
    message.type !== undefined &&
      (obj.type = artifactCredentialTypeToJSON(message.type));
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ArtifactCredentialInfo>, I>>(
    object: I
  ): ArtifactCredentialInfo {
    const message = createBaseArtifactCredentialInfo();
    message.runId = object.runId ?? "";
    message.path = object.path ?? "";
    message.signedUri = object.signedUri ?? "";
    message.headers =
      object.headers?.map((e) =>
        ArtifactCredentialInfo_HttpHeader.fromPartial(e)
      ) || [];
    message.type = object.type ?? 1;
    return message;
  },
};

function createBaseArtifactCredentialInfo_HttpHeader(): ArtifactCredentialInfo_HttpHeader {
  return { name: "", value: "" };
}

export const ArtifactCredentialInfo_HttpHeader = {
  encode(
    message: ArtifactCredentialInfo_HttpHeader,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    if (message.value !== "") {
      writer.uint32(18).string(message.value);
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ArtifactCredentialInfo_HttpHeader {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseArtifactCredentialInfo_HttpHeader();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.name = reader.string();
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

  fromJSON(object: any): ArtifactCredentialInfo_HttpHeader {
    return {
      name: isSet(object.name) ? String(object.name) : "",
      value: isSet(object.value) ? String(object.value) : "",
    };
  },

  toJSON(message: ArtifactCredentialInfo_HttpHeader): unknown {
    const obj: any = {};
    message.name !== undefined && (obj.name = message.name);
    message.value !== undefined && (obj.value = message.value);
    return obj;
  },

  fromPartial<
    I extends Exact<DeepPartial<ArtifactCredentialInfo_HttpHeader>, I>
  >(object: I): ArtifactCredentialInfo_HttpHeader {
    const message = createBaseArtifactCredentialInfo_HttpHeader();
    message.name = object.name ?? "";
    message.value = object.value ?? "";
    return message;
  },
};

function createBaseGetCredentialsForRead(): GetCredentialsForRead {
  return { runId: "", path: [], pageToken: "" };
}

export const GetCredentialsForRead = {
  encode(
    message: GetCredentialsForRead,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.runId !== "") {
      writer.uint32(10).string(message.runId);
    }
    for (const v of message.path) {
      writer.uint32(18).string(v!);
    }
    if (message.pageToken !== "") {
      writer.uint32(26).string(message.pageToken);
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): GetCredentialsForRead {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseGetCredentialsForRead();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.runId = reader.string();
          break;
        case 2:
          message.path.push(reader.string());
          break;
        case 3:
          message.pageToken = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): GetCredentialsForRead {
    return {
      runId: isSet(object.runId) ? String(object.runId) : "",
      path: Array.isArray(object?.path)
        ? object.path.map((e: any) => String(e))
        : [],
      pageToken: isSet(object.pageToken) ? String(object.pageToken) : "",
    };
  },

  toJSON(message: GetCredentialsForRead): unknown {
    const obj: any = {};
    message.runId !== undefined && (obj.runId = message.runId);
    if (message.path) {
      obj.path = message.path.map((e) => e);
    } else {
      obj.path = [];
    }
    message.pageToken !== undefined && (obj.pageToken = message.pageToken);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<GetCredentialsForRead>, I>>(
    object: I
  ): GetCredentialsForRead {
    const message = createBaseGetCredentialsForRead();
    message.runId = object.runId ?? "";
    message.path = object.path?.map((e) => e) || [];
    message.pageToken = object.pageToken ?? "";
    return message;
  },
};

function createBaseGetCredentialsForRead_Response(): GetCredentialsForRead_Response {
  return { credentialInfos: [], nextPageToken: "" };
}

export const GetCredentialsForRead_Response = {
  encode(
    message: GetCredentialsForRead_Response,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    for (const v of message.credentialInfos) {
      ArtifactCredentialInfo.encode(v!, writer.uint32(18).fork()).ldelim();
    }
    if (message.nextPageToken !== "") {
      writer.uint32(26).string(message.nextPageToken);
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): GetCredentialsForRead_Response {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseGetCredentialsForRead_Response();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 2:
          message.credentialInfos.push(
            ArtifactCredentialInfo.decode(reader, reader.uint32())
          );
          break;
        case 3:
          message.nextPageToken = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): GetCredentialsForRead_Response {
    return {
      credentialInfos: Array.isArray(object?.credentialInfos)
        ? object.credentialInfos.map((e: any) =>
            ArtifactCredentialInfo.fromJSON(e)
          )
        : [],
      nextPageToken: isSet(object.nextPageToken)
        ? String(object.nextPageToken)
        : "",
    };
  },

  toJSON(message: GetCredentialsForRead_Response): unknown {
    const obj: any = {};
    if (message.credentialInfos) {
      obj.credentialInfos = message.credentialInfos.map((e) =>
        e ? ArtifactCredentialInfo.toJSON(e) : undefined
      );
    } else {
      obj.credentialInfos = [];
    }
    message.nextPageToken !== undefined &&
      (obj.nextPageToken = message.nextPageToken);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<GetCredentialsForRead_Response>, I>>(
    object: I
  ): GetCredentialsForRead_Response {
    const message = createBaseGetCredentialsForRead_Response();
    message.credentialInfos =
      object.credentialInfos?.map((e) =>
        ArtifactCredentialInfo.fromPartial(e)
      ) || [];
    message.nextPageToken = object.nextPageToken ?? "";
    return message;
  },
};

function createBaseGetCredentialsForWrite(): GetCredentialsForWrite {
  return { runId: "", path: [], pageToken: "" };
}

export const GetCredentialsForWrite = {
  encode(
    message: GetCredentialsForWrite,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.runId !== "") {
      writer.uint32(10).string(message.runId);
    }
    for (const v of message.path) {
      writer.uint32(18).string(v!);
    }
    if (message.pageToken !== "") {
      writer.uint32(26).string(message.pageToken);
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): GetCredentialsForWrite {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseGetCredentialsForWrite();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.runId = reader.string();
          break;
        case 2:
          message.path.push(reader.string());
          break;
        case 3:
          message.pageToken = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): GetCredentialsForWrite {
    return {
      runId: isSet(object.runId) ? String(object.runId) : "",
      path: Array.isArray(object?.path)
        ? object.path.map((e: any) => String(e))
        : [],
      pageToken: isSet(object.pageToken) ? String(object.pageToken) : "",
    };
  },

  toJSON(message: GetCredentialsForWrite): unknown {
    const obj: any = {};
    message.runId !== undefined && (obj.runId = message.runId);
    if (message.path) {
      obj.path = message.path.map((e) => e);
    } else {
      obj.path = [];
    }
    message.pageToken !== undefined && (obj.pageToken = message.pageToken);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<GetCredentialsForWrite>, I>>(
    object: I
  ): GetCredentialsForWrite {
    const message = createBaseGetCredentialsForWrite();
    message.runId = object.runId ?? "";
    message.path = object.path?.map((e) => e) || [];
    message.pageToken = object.pageToken ?? "";
    return message;
  },
};

function createBaseGetCredentialsForWrite_Response(): GetCredentialsForWrite_Response {
  return { credentialInfos: [], nextPageToken: "" };
}

export const GetCredentialsForWrite_Response = {
  encode(
    message: GetCredentialsForWrite_Response,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    for (const v of message.credentialInfos) {
      ArtifactCredentialInfo.encode(v!, writer.uint32(18).fork()).ldelim();
    }
    if (message.nextPageToken !== "") {
      writer.uint32(26).string(message.nextPageToken);
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): GetCredentialsForWrite_Response {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseGetCredentialsForWrite_Response();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 2:
          message.credentialInfos.push(
            ArtifactCredentialInfo.decode(reader, reader.uint32())
          );
          break;
        case 3:
          message.nextPageToken = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): GetCredentialsForWrite_Response {
    return {
      credentialInfos: Array.isArray(object?.credentialInfos)
        ? object.credentialInfos.map((e: any) =>
            ArtifactCredentialInfo.fromJSON(e)
          )
        : [],
      nextPageToken: isSet(object.nextPageToken)
        ? String(object.nextPageToken)
        : "",
    };
  },

  toJSON(message: GetCredentialsForWrite_Response): unknown {
    const obj: any = {};
    if (message.credentialInfos) {
      obj.credentialInfos = message.credentialInfos.map((e) =>
        e ? ArtifactCredentialInfo.toJSON(e) : undefined
      );
    } else {
      obj.credentialInfos = [];
    }
    message.nextPageToken !== undefined &&
      (obj.nextPageToken = message.nextPageToken);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<GetCredentialsForWrite_Response>, I>>(
    object: I
  ): GetCredentialsForWrite_Response {
    const message = createBaseGetCredentialsForWrite_Response();
    message.credentialInfos =
      object.credentialInfos?.map((e) =>
        ArtifactCredentialInfo.fromPartial(e)
      ) || [];
    message.nextPageToken = object.nextPageToken ?? "";
    return message;
  },
};

export interface DatabricksMlflowArtifactsService {
  /**
   * Fetch credentials to read from the specified MLflow artifact location
   *
   * Note: Even if no artifacts exist at the specified artifact location, this API will
   * still provide read credentials as long as the format of the location is valid.
   * Callers must subsequently check for the existence of the artifacts using the appropriate
   * cloud storage APIs (as determined by the `ArtifactCredentialType` property of the response)
   */
  getCredentialsForRead(
    request: GetCredentialsForRead
  ): Promise<GetCredentialsForRead_Response>;
  /** Fetch credentials to write to the specified MLflow artifact location */
  getCredentialsForWrite(
    request: GetCredentialsForWrite
  ): Promise<GetCredentialsForWrite_Response>;
}

export class DatabricksMlflowArtifactsServiceClientImpl
  implements DatabricksMlflowArtifactsService
{
  private readonly rpc: Rpc;
  constructor(rpc: Rpc) {
    this.rpc = rpc;
    this.getCredentialsForRead = this.getCredentialsForRead.bind(this);
    this.getCredentialsForWrite = this.getCredentialsForWrite.bind(this);
  }
  getCredentialsForRead(
    request: GetCredentialsForRead
  ): Promise<GetCredentialsForRead_Response> {
    const data = GetCredentialsForRead.encode(request).finish();
    const promise = this.rpc.request(
      "mlflow.DatabricksMlflowArtifactsService",
      "getCredentialsForRead",
      data
    );
    return promise.then((data) =>
      GetCredentialsForRead_Response.decode(new _m0.Reader(data))
    );
  }

  getCredentialsForWrite(
    request: GetCredentialsForWrite
  ): Promise<GetCredentialsForWrite_Response> {
    const data = GetCredentialsForWrite.encode(request).finish();
    const promise = this.rpc.request(
      "mlflow.DatabricksMlflowArtifactsService",
      "getCredentialsForWrite",
      data
    );
    return promise.then((data) =>
      GetCredentialsForWrite_Response.decode(new _m0.Reader(data))
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
