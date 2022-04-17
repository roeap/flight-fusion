/* eslint-disable */
import * as Long from "long";
import * as _m0 from "protobufjs/minimal";

export const protobufPackage = "mlflow";

/** View type for ListExperiments query. */
export enum ViewType {
  /** ACTIVE_ONLY - Default. Return only active experiments. */
  ACTIVE_ONLY = 1,
  /** DELETED_ONLY - Return only deleted experiments. */
  DELETED_ONLY = 2,
  /** ALL - Get all experiments. */
  ALL = 3,
  UNRECOGNIZED = -1,
}

export function viewTypeFromJSON(object: any): ViewType {
  switch (object) {
    case 1:
    case "ACTIVE_ONLY":
      return ViewType.ACTIVE_ONLY;
    case 2:
    case "DELETED_ONLY":
      return ViewType.DELETED_ONLY;
    case 3:
    case "ALL":
      return ViewType.ALL;
    case -1:
    case "UNRECOGNIZED":
    default:
      return ViewType.UNRECOGNIZED;
  }
}

export function viewTypeToJSON(object: ViewType): string {
  switch (object) {
    case ViewType.ACTIVE_ONLY:
      return "ACTIVE_ONLY";
    case ViewType.DELETED_ONLY:
      return "DELETED_ONLY";
    case ViewType.ALL:
      return "ALL";
    default:
      return "UNKNOWN";
  }
}

/** Source that generated a run. */
export enum SourceType {
  /** NOTEBOOK - Databricks notebook environment. */
  NOTEBOOK = 1,
  /** JOB - Scheduled or Run Now job. */
  JOB = 2,
  /** PROJECT - As a prepackaged project: either a Docker image or GitHub source, etc. */
  PROJECT = 3,
  /** LOCAL - Local run: Using CLI, IDE, or local notebook. */
  LOCAL = 4,
  /** UNKNOWN - Unknown source type. */
  UNKNOWN = 1000,
  UNRECOGNIZED = -1,
}

export function sourceTypeFromJSON(object: any): SourceType {
  switch (object) {
    case 1:
    case "NOTEBOOK":
      return SourceType.NOTEBOOK;
    case 2:
    case "JOB":
      return SourceType.JOB;
    case 3:
    case "PROJECT":
      return SourceType.PROJECT;
    case 4:
    case "LOCAL":
      return SourceType.LOCAL;
    case 1000:
    case "UNKNOWN":
      return SourceType.UNKNOWN;
    case -1:
    case "UNRECOGNIZED":
    default:
      return SourceType.UNRECOGNIZED;
  }
}

export function sourceTypeToJSON(object: SourceType): string {
  switch (object) {
    case SourceType.NOTEBOOK:
      return "NOTEBOOK";
    case SourceType.JOB:
      return "JOB";
    case SourceType.PROJECT:
      return "PROJECT";
    case SourceType.LOCAL:
      return "LOCAL";
    case SourceType.UNKNOWN:
      return "UNKNOWN";
    default:
      return "UNKNOWN";
  }
}

/** Status of a run. */
export enum RunStatus {
  /** RUNNING - Run has been initiated. */
  RUNNING = 1,
  /** SCHEDULED - Run is scheduled to run at a later time. */
  SCHEDULED = 2,
  /** FINISHED - Run has completed. */
  FINISHED = 3,
  /** FAILED - Run execution failed. */
  FAILED = 4,
  /** KILLED - Run killed by user. */
  KILLED = 5,
  UNRECOGNIZED = -1,
}

export function runStatusFromJSON(object: any): RunStatus {
  switch (object) {
    case 1:
    case "RUNNING":
      return RunStatus.RUNNING;
    case 2:
    case "SCHEDULED":
      return RunStatus.SCHEDULED;
    case 3:
    case "FINISHED":
      return RunStatus.FINISHED;
    case 4:
    case "FAILED":
      return RunStatus.FAILED;
    case 5:
    case "KILLED":
      return RunStatus.KILLED;
    case -1:
    case "UNRECOGNIZED":
    default:
      return RunStatus.UNRECOGNIZED;
  }
}

export function runStatusToJSON(object: RunStatus): string {
  switch (object) {
    case RunStatus.RUNNING:
      return "RUNNING";
    case RunStatus.SCHEDULED:
      return "SCHEDULED";
    case RunStatus.FINISHED:
      return "FINISHED";
    case RunStatus.FAILED:
      return "FAILED";
    case RunStatus.KILLED:
      return "KILLED";
    default:
      return "UNKNOWN";
  }
}

/** Metric associated with a run, represented as a key-value pair. */
export interface Metric {
  /** Key identifying this metric. */
  key: string;
  /** Value associated with this metric. */
  value: number;
  /** The timestamp at which this metric was recorded. */
  timestamp: number;
  /** Step at which to log the metric. */
  step: number;
}

/** Param associated with a run. */
export interface Param {
  /** Key identifying this param. */
  key: string;
  /** Value associated with this param. */
  value: string;
}

/** A single run. */
export interface Run {
  /** Run metadata. */
  info: RunInfo | undefined;
  /** Run data. */
  data: RunData | undefined;
}

/** Run data (metrics, params, and tags). */
export interface RunData {
  /** Run metrics. */
  metrics: Metric[];
  /** Run parameters. */
  params: Param[];
  /** Additional metadata key-value pairs. */
  tags: RunTag[];
}

/** Tag for a run. */
export interface RunTag {
  /** The tag key. */
  key: string;
  /** The tag value. */
  value: string;
}

/** Tag for an experiment. */
export interface ExperimentTag {
  /** The tag key. */
  key: string;
  /** The tag value. */
  value: string;
}

/** Metadata of a single run. */
export interface RunInfo {
  /** Unique identifier for the run. */
  runId: string;
  /**
   * [Deprecated, use run_id instead] Unique identifier for the run. This field will
   * be removed in a future MLflow version.
   */
  runUuid: string;
  /** The experiment ID. */
  experimentId: string;
  /**
   * User who initiated the run.
   * This field is deprecated as of MLflow 1.0, and will be removed in a future
   * MLflow release. Use 'mlflow.user' tag instead.
   */
  userId: string;
  /** Current status of the run. */
  status: RunStatus;
  /** Unix timestamp of when the run started in milliseconds. */
  startTime: number;
  /** Unix timestamp of when the run ended in milliseconds. */
  endTime: number;
  /**
   * URI of the directory where artifacts should be uploaded.
   * This can be a local path (starting with "/"), or a distributed file system (DFS)
   * path, like ``s3://bucket/directory`` or ``dbfs:/my/directory``.
   * If not set, the local ``./mlruns`` directory is  chosen.
   */
  artifactUri: string;
  /** Current life cycle stage of the experiment : OneOf("active", "deleted") */
  lifecycleStage: string;
}

/** Experiment */
export interface Experiment {
  /** Unique identifier for the experiment. */
  experimentId: string;
  /** Human readable name that identifies the experiment. */
  name: string;
  /** Location where artifacts for the experiment are stored. */
  artifactLocation: string;
  /**
   * Current life cycle stage of the experiment: "active" or "deleted".
   * Deleted experiments are not returned by APIs.
   */
  lifecycleStage: string;
  /** Last update time */
  lastUpdateTime: number;
  /** Creation time */
  creationTime: number;
  /** Tags: Additional metadata key-value pairs. */
  tags: ExperimentTag[];
}

export interface CreateExperiment {
  /** Experiment name. */
  name: string;
  /**
   * Location where all artifacts for the experiment are stored.
   * If not provided, the remote server will select an appropriate default.
   */
  artifactLocation: string;
  /**
   * A collection of tags to set on the experiment. Maximum tag size and number of tags per request
   * depends on the storage backend. All storage backends are guaranteed to support tag keys up
   * to 250 bytes in size and tag values up to 5000 bytes in size. All storage backends are also
   * guaranteed to support up to 20 tags per request.
   */
  tags: ExperimentTag[];
}

export interface CreateExperiment_Response {
  /** Unique identifier for the experiment. */
  experimentId: string;
}

export interface ListExperiments {
  /**
   * Qualifier for type of experiments to be returned.
   * If unspecified, return only active experiments.
   */
  viewType: ViewType;
  /**
   * Maximum number of experiments desired.
   * Servers may select a desired default `max_results` value. All servers are
   * guaranteed to support a `max_results` threshold of at least 1,000 but may
   * support more. Callers of this endpoint are encouraged to pass max_results
   * explicitly and leverage page_token to iterate through experiments.
   */
  maxResults: number;
  /** Pagination token to go to the next page based on a previous query. */
  pageToken: string;
}

export interface ListExperiments_Response {
  /** All experiments. */
  experiments: Experiment[];
  /** Pagination token to request next page of experiments for the same query. */
  nextPageToken: string;
}

export interface GetExperiment {
  /** ID of the associated experiment. */
  experimentId: string;
}

export interface GetExperiment_Response {
  /** Experiment details. */
  experiment: Experiment | undefined;
  /**
   * A collection of active runs in the experiment. Note: this may not contain
   * all of the experiment's active runs.
   *
   * This field is deprecated. Please use the "Search Runs" API to fetch
   * runs within an experiment.
   *
   * @deprecated
   */
  runs: RunInfo[];
}

export interface DeleteExperiment {
  /** ID of the associated experiment. */
  experimentId: string;
}

export interface DeleteExperiment_Response {}

export interface RestoreExperiment {
  /** ID of the associated experiment. */
  experimentId: string;
}

export interface RestoreExperiment_Response {}

export interface UpdateExperiment {
  /** ID of the associated experiment. */
  experimentId: string;
  /** If provided, the experiment's name is changed to the new name. The new name must be unique. */
  newName: string;
}

export interface UpdateExperiment_Response {}

export interface CreateRun {
  /** ID of the associated experiment. */
  experimentId: string;
  /**
   * ID of the user executing the run.
   * This field is deprecated as of MLflow 1.0, and will be removed in a future
   * MLflow release. Use 'mlflow.user' tag instead.
   */
  userId: string;
  /** Unix timestamp in milliseconds of when the run started. */
  startTime: number;
  /** Additional metadata for run. */
  tags: RunTag[];
}

export interface CreateRun_Response {
  /** The newly created run. */
  run: Run | undefined;
}

export interface UpdateRun {
  /** ID of the run to update. Must be provided. */
  runId: string;
  /**
   * [Deprecated, use run_id instead] ID of the run to update.. This field will
   * be removed in a future MLflow version.
   */
  runUuid: string;
  /** Updated status of the run. */
  status: RunStatus;
  /** Unix timestamp in milliseconds of when the run ended. */
  endTime: number;
}

export interface UpdateRun_Response {
  /** Updated metadata of the run. */
  runInfo: RunInfo | undefined;
}

export interface DeleteRun {
  /** ID of the run to delete. */
  runId: string;
}

export interface DeleteRun_Response {}

export interface RestoreRun {
  /** ID of the run to restore. */
  runId: string;
}

export interface RestoreRun_Response {}

export interface LogMetric {
  /** ID of the run under which to log the metric. Must be provided. */
  runId: string;
  /**
   * [Deprecated, use run_id instead] ID of the run under which to log the metric. This field will
   * be removed in a future MLflow version.
   */
  runUuid: string;
  /** Name of the metric. */
  key: string;
  /** Double value of the metric being logged. */
  value: number;
  /** Unix timestamp in milliseconds at the time metric was logged. */
  timestamp: number;
  /** Step at which to log the metric */
  step: number;
}

export interface LogMetric_Response {}

export interface LogParam {
  /** ID of the run under which to log the param. Must be provided. */
  runId: string;
  /**
   * [Deprecated, use run_id instead] ID of the run under which to log the param. This field will
   * be removed in a future MLflow version.
   */
  runUuid: string;
  /** Name of the param. Maximum size is 255 bytes. */
  key: string;
  /** String value of the param being logged. Maximum size is 500 bytes. */
  value: string;
}

export interface LogParam_Response {}

export interface SetExperimentTag {
  /** ID of the experiment under which to log the tag. Must be provided. */
  experimentId: string;
  /**
   * Name of the tag. Maximum size depends on storage backend.
   * All storage backends are guaranteed to support key values up to 250 bytes in size.
   */
  key: string;
  /**
   * String value of the tag being logged. Maximum size depends on storage backend.
   * All storage backends are guaranteed to support key values up to 5000 bytes in size.
   */
  value: string;
}

export interface SetExperimentTag_Response {}

export interface SetTag {
  /** ID of the run under which to log the tag. Must be provided. */
  runId: string;
  /**
   * [Deprecated, use run_id instead] ID of the run under which to log the tag. This field will
   * be removed in a future MLflow version.
   */
  runUuid: string;
  /**
   * Name of the tag. Maximum size depends on storage backend.
   * All storage backends are guaranteed to support key values up to 250 bytes in size.
   */
  key: string;
  /**
   * String value of the tag being logged. Maximum size depends on storage backend.
   * All storage backends are guaranteed to support key values up to 5000 bytes in size.
   */
  value: string;
}

export interface SetTag_Response {}

export interface DeleteTag {
  /** ID of the run that the tag was logged under. Must be provided. */
  runId: string;
  /** Name of the tag. Maximum size is 255 bytes. Must be provided. */
  key: string;
}

export interface DeleteTag_Response {}

export interface GetRun {
  /** ID of the run to fetch. Must be provided. */
  runId: string;
  /**
   * [Deprecated, use run_id instead] ID of the run to fetch. This field will
   * be removed in a future MLflow version.
   */
  runUuid: string;
}

export interface GetRun_Response {
  /** Run metadata (name, start time, etc) and data (metrics, params, and tags). */
  run: Run | undefined;
}

export interface SearchRuns {
  /** List of experiment IDs to search over. */
  experimentIds: string[];
  /**
   * A filter expression over params, metrics, and tags, that allows returning a subset of
   * runs. The syntax is a subset of SQL that supports ANDing together binary operations
   * between a param, metric, or tag and a constant.
   *
   * Example: ``metrics.rmse < 1 and params.model_class = 'LogisticRegression'``
   *
   * You can select columns with special characters (hyphen, space, period, etc.) by using double quotes:
   * ``metrics."model class" = 'LinearRegression' and tags."user-name" = 'Tomas'``
   *
   * Supported operators are ``=``, ``!=``, ``>``, ``>=``, ``<``, and ``<=``.
   */
  filter: string;
  /**
   * Whether to display only active, only deleted, or all runs.
   * Defaults to only active runs.
   */
  runViewType: ViewType;
  /**
   * Maximum number of runs desired. If unspecified, defaults to 1000.
   * All servers are guaranteed to support a `max_results` threshold of at least 50,000
   * but may support more. Callers of this endpoint are encouraged to pass max_results
   * explicitly and leverage page_token to iterate through experiments.
   */
  maxResults: number;
  /**
   * List of columns to be ordered by, including attributes, params, metrics, and tags with an
   * optional "DESC" or "ASC" annotation, where "ASC" is the default.
   * Example: ["params.input DESC", "metrics.alpha ASC", "metrics.rmse"]
   * Tiebreaks are done by start_time DESC followed by run_id for runs with the same start time
   * (and this is the default ordering criterion if order_by is not provided).
   */
  orderBy: string[];
  pageToken: string;
}

export interface SearchRuns_Response {
  /** Runs that match the search criteria. */
  runs: Run[];
  nextPageToken: string;
}

export interface ListArtifacts {
  /** ID of the run whose artifacts to list. Must be provided. */
  runId: string;
  /**
   * [Deprecated, use run_id instead] ID of the run whose artifacts to list. This field will
   * be removed in a future MLflow version.
   */
  runUuid: string;
  /** Filter artifacts matching this path (a relative path from the root artifact directory). */
  path: string;
  /** Token indicating the page of artifact results to fetch */
  pageToken: string;
}

export interface ListArtifacts_Response {
  /** Root artifact directory for the run. */
  rootUri: string;
  /** File location and metadata for artifacts. */
  files: FileInfo[];
  /** Token that can be used to retrieve the next page of artifact results */
  nextPageToken: string;
}

/** Metadata of a single artifact file or directory. */
export interface FileInfo {
  /** Path relative to the root artifact directory run. */
  path: string;
  /** Whether the path is a directory. */
  isDir: boolean;
  /** Size in bytes. Unset for directories. */
  fileSize: number;
}

export interface GetMetricHistory {
  /** ID of the run from which to fetch metric values. Must be provided. */
  runId: string;
  /**
   * [Deprecated, use run_id instead] ID of the run from which to fetch metric values. This field
   * will be removed in a future MLflow version.
   */
  runUuid: string;
  /** Name of the metric. */
  metricKey: string;
}

export interface GetMetricHistory_Response {
  /** All logged values for this metric. */
  metrics: Metric[];
}

export interface LogBatch {
  /** ID of the run to log under */
  runId: string;
  /**
   * Metrics to log. A single request can contain up to 1000 metrics, and up to 1000
   * metrics, params, and tags in total.
   */
  metrics: Metric[];
  /**
   * Params to log. A single request can contain up to 100 params, and up to 1000
   * metrics, params, and tags in total.
   */
  params: Param[];
  /**
   * Tags to log. A single request can contain up to 100 tags, and up to 1000
   * metrics, params, and tags in total.
   */
  tags: RunTag[];
}

export interface LogBatch_Response {}

export interface LogModel {
  /** ID of the run to log under */
  runId: string;
  /** MLmodel file in json format. */
  modelJson: string;
}

export interface LogModel_Response {}

export interface GetExperimentByName {
  /** Name of the associated experiment. */
  experimentName: string;
}

export interface GetExperimentByName_Response {
  /** Experiment details. */
  experiment: Experiment | undefined;
}

function createBaseMetric(): Metric {
  return { key: "", value: 0, timestamp: 0, step: 0 };
}

export const Metric = {
  encode(
    message: Metric,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== 0) {
      writer.uint32(17).double(message.value);
    }
    if (message.timestamp !== 0) {
      writer.uint32(24).int64(message.timestamp);
    }
    if (message.step !== 0) {
      writer.uint32(32).int64(message.step);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Metric {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseMetric();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.key = reader.string();
          break;
        case 2:
          message.value = reader.double();
          break;
        case 3:
          message.timestamp = longToNumber(reader.int64() as Long);
          break;
        case 4:
          message.step = longToNumber(reader.int64() as Long);
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Metric {
    return {
      key: isSet(object.key) ? String(object.key) : "",
      value: isSet(object.value) ? Number(object.value) : 0,
      timestamp: isSet(object.timestamp) ? Number(object.timestamp) : 0,
      step: isSet(object.step) ? Number(object.step) : 0,
    };
  },

  toJSON(message: Metric): unknown {
    const obj: any = {};
    message.key !== undefined && (obj.key = message.key);
    message.value !== undefined && (obj.value = message.value);
    message.timestamp !== undefined &&
      (obj.timestamp = Math.round(message.timestamp));
    message.step !== undefined && (obj.step = Math.round(message.step));
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<Metric>, I>>(object: I): Metric {
    const message = createBaseMetric();
    message.key = object.key ?? "";
    message.value = object.value ?? 0;
    message.timestamp = object.timestamp ?? 0;
    message.step = object.step ?? 0;
    return message;
  },
};

function createBaseParam(): Param {
  return { key: "", value: "" };
}

export const Param = {
  encode(message: Param, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== "") {
      writer.uint32(18).string(message.value);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Param {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseParam();
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

  fromJSON(object: any): Param {
    return {
      key: isSet(object.key) ? String(object.key) : "",
      value: isSet(object.value) ? String(object.value) : "",
    };
  },

  toJSON(message: Param): unknown {
    const obj: any = {};
    message.key !== undefined && (obj.key = message.key);
    message.value !== undefined && (obj.value = message.value);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<Param>, I>>(object: I): Param {
    const message = createBaseParam();
    message.key = object.key ?? "";
    message.value = object.value ?? "";
    return message;
  },
};

function createBaseRun(): Run {
  return { info: undefined, data: undefined };
}

export const Run = {
  encode(message: Run, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.info !== undefined) {
      RunInfo.encode(message.info, writer.uint32(10).fork()).ldelim();
    }
    if (message.data !== undefined) {
      RunData.encode(message.data, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Run {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRun();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.info = RunInfo.decode(reader, reader.uint32());
          break;
        case 2:
          message.data = RunData.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Run {
    return {
      info: isSet(object.info) ? RunInfo.fromJSON(object.info) : undefined,
      data: isSet(object.data) ? RunData.fromJSON(object.data) : undefined,
    };
  },

  toJSON(message: Run): unknown {
    const obj: any = {};
    message.info !== undefined &&
      (obj.info = message.info ? RunInfo.toJSON(message.info) : undefined);
    message.data !== undefined &&
      (obj.data = message.data ? RunData.toJSON(message.data) : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<Run>, I>>(object: I): Run {
    const message = createBaseRun();
    message.info =
      object.info !== undefined && object.info !== null
        ? RunInfo.fromPartial(object.info)
        : undefined;
    message.data =
      object.data !== undefined && object.data !== null
        ? RunData.fromPartial(object.data)
        : undefined;
    return message;
  },
};

function createBaseRunData(): RunData {
  return { metrics: [], params: [], tags: [] };
}

export const RunData = {
  encode(
    message: RunData,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    for (const v of message.metrics) {
      Metric.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    for (const v of message.params) {
      Param.encode(v!, writer.uint32(18).fork()).ldelim();
    }
    for (const v of message.tags) {
      RunTag.encode(v!, writer.uint32(26).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): RunData {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRunData();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.metrics.push(Metric.decode(reader, reader.uint32()));
          break;
        case 2:
          message.params.push(Param.decode(reader, reader.uint32()));
          break;
        case 3:
          message.tags.push(RunTag.decode(reader, reader.uint32()));
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): RunData {
    return {
      metrics: Array.isArray(object?.metrics)
        ? object.metrics.map((e: any) => Metric.fromJSON(e))
        : [],
      params: Array.isArray(object?.params)
        ? object.params.map((e: any) => Param.fromJSON(e))
        : [],
      tags: Array.isArray(object?.tags)
        ? object.tags.map((e: any) => RunTag.fromJSON(e))
        : [],
    };
  },

  toJSON(message: RunData): unknown {
    const obj: any = {};
    if (message.metrics) {
      obj.metrics = message.metrics.map((e) =>
        e ? Metric.toJSON(e) : undefined
      );
    } else {
      obj.metrics = [];
    }
    if (message.params) {
      obj.params = message.params.map((e) => (e ? Param.toJSON(e) : undefined));
    } else {
      obj.params = [];
    }
    if (message.tags) {
      obj.tags = message.tags.map((e) => (e ? RunTag.toJSON(e) : undefined));
    } else {
      obj.tags = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<RunData>, I>>(object: I): RunData {
    const message = createBaseRunData();
    message.metrics = object.metrics?.map((e) => Metric.fromPartial(e)) || [];
    message.params = object.params?.map((e) => Param.fromPartial(e)) || [];
    message.tags = object.tags?.map((e) => RunTag.fromPartial(e)) || [];
    return message;
  },
};

function createBaseRunTag(): RunTag {
  return { key: "", value: "" };
}

export const RunTag = {
  encode(
    message: RunTag,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== "") {
      writer.uint32(18).string(message.value);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): RunTag {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRunTag();
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

  fromJSON(object: any): RunTag {
    return {
      key: isSet(object.key) ? String(object.key) : "",
      value: isSet(object.value) ? String(object.value) : "",
    };
  },

  toJSON(message: RunTag): unknown {
    const obj: any = {};
    message.key !== undefined && (obj.key = message.key);
    message.value !== undefined && (obj.value = message.value);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<RunTag>, I>>(object: I): RunTag {
    const message = createBaseRunTag();
    message.key = object.key ?? "";
    message.value = object.value ?? "";
    return message;
  },
};

function createBaseExperimentTag(): ExperimentTag {
  return { key: "", value: "" };
}

export const ExperimentTag = {
  encode(
    message: ExperimentTag,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.key !== "") {
      writer.uint32(10).string(message.key);
    }
    if (message.value !== "") {
      writer.uint32(18).string(message.value);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ExperimentTag {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseExperimentTag();
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

  fromJSON(object: any): ExperimentTag {
    return {
      key: isSet(object.key) ? String(object.key) : "",
      value: isSet(object.value) ? String(object.value) : "",
    };
  },

  toJSON(message: ExperimentTag): unknown {
    const obj: any = {};
    message.key !== undefined && (obj.key = message.key);
    message.value !== undefined && (obj.value = message.value);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ExperimentTag>, I>>(
    object: I
  ): ExperimentTag {
    const message = createBaseExperimentTag();
    message.key = object.key ?? "";
    message.value = object.value ?? "";
    return message;
  },
};

function createBaseRunInfo(): RunInfo {
  return {
    runId: "",
    runUuid: "",
    experimentId: "",
    userId: "",
    status: 1,
    startTime: 0,
    endTime: 0,
    artifactUri: "",
    lifecycleStage: "",
  };
}

export const RunInfo = {
  encode(
    message: RunInfo,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.runId !== "") {
      writer.uint32(122).string(message.runId);
    }
    if (message.runUuid !== "") {
      writer.uint32(10).string(message.runUuid);
    }
    if (message.experimentId !== "") {
      writer.uint32(18).string(message.experimentId);
    }
    if (message.userId !== "") {
      writer.uint32(50).string(message.userId);
    }
    if (message.status !== 1) {
      writer.uint32(56).int32(message.status);
    }
    if (message.startTime !== 0) {
      writer.uint32(64).int64(message.startTime);
    }
    if (message.endTime !== 0) {
      writer.uint32(72).int64(message.endTime);
    }
    if (message.artifactUri !== "") {
      writer.uint32(106).string(message.artifactUri);
    }
    if (message.lifecycleStage !== "") {
      writer.uint32(114).string(message.lifecycleStage);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): RunInfo {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRunInfo();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 15:
          message.runId = reader.string();
          break;
        case 1:
          message.runUuid = reader.string();
          break;
        case 2:
          message.experimentId = reader.string();
          break;
        case 6:
          message.userId = reader.string();
          break;
        case 7:
          message.status = reader.int32() as any;
          break;
        case 8:
          message.startTime = longToNumber(reader.int64() as Long);
          break;
        case 9:
          message.endTime = longToNumber(reader.int64() as Long);
          break;
        case 13:
          message.artifactUri = reader.string();
          break;
        case 14:
          message.lifecycleStage = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): RunInfo {
    return {
      runId: isSet(object.runId) ? String(object.runId) : "",
      runUuid: isSet(object.runUuid) ? String(object.runUuid) : "",
      experimentId: isSet(object.experimentId)
        ? String(object.experimentId)
        : "",
      userId: isSet(object.userId) ? String(object.userId) : "",
      status: isSet(object.status) ? runStatusFromJSON(object.status) : 1,
      startTime: isSet(object.startTime) ? Number(object.startTime) : 0,
      endTime: isSet(object.endTime) ? Number(object.endTime) : 0,
      artifactUri: isSet(object.artifactUri) ? String(object.artifactUri) : "",
      lifecycleStage: isSet(object.lifecycleStage)
        ? String(object.lifecycleStage)
        : "",
    };
  },

  toJSON(message: RunInfo): unknown {
    const obj: any = {};
    message.runId !== undefined && (obj.runId = message.runId);
    message.runUuid !== undefined && (obj.runUuid = message.runUuid);
    message.experimentId !== undefined &&
      (obj.experimentId = message.experimentId);
    message.userId !== undefined && (obj.userId = message.userId);
    message.status !== undefined &&
      (obj.status = runStatusToJSON(message.status));
    message.startTime !== undefined &&
      (obj.startTime = Math.round(message.startTime));
    message.endTime !== undefined &&
      (obj.endTime = Math.round(message.endTime));
    message.artifactUri !== undefined &&
      (obj.artifactUri = message.artifactUri);
    message.lifecycleStage !== undefined &&
      (obj.lifecycleStage = message.lifecycleStage);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<RunInfo>, I>>(object: I): RunInfo {
    const message = createBaseRunInfo();
    message.runId = object.runId ?? "";
    message.runUuid = object.runUuid ?? "";
    message.experimentId = object.experimentId ?? "";
    message.userId = object.userId ?? "";
    message.status = object.status ?? 1;
    message.startTime = object.startTime ?? 0;
    message.endTime = object.endTime ?? 0;
    message.artifactUri = object.artifactUri ?? "";
    message.lifecycleStage = object.lifecycleStage ?? "";
    return message;
  },
};

function createBaseExperiment(): Experiment {
  return {
    experimentId: "",
    name: "",
    artifactLocation: "",
    lifecycleStage: "",
    lastUpdateTime: 0,
    creationTime: 0,
    tags: [],
  };
}

export const Experiment = {
  encode(
    message: Experiment,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.experimentId !== "") {
      writer.uint32(10).string(message.experimentId);
    }
    if (message.name !== "") {
      writer.uint32(18).string(message.name);
    }
    if (message.artifactLocation !== "") {
      writer.uint32(26).string(message.artifactLocation);
    }
    if (message.lifecycleStage !== "") {
      writer.uint32(34).string(message.lifecycleStage);
    }
    if (message.lastUpdateTime !== 0) {
      writer.uint32(40).int64(message.lastUpdateTime);
    }
    if (message.creationTime !== 0) {
      writer.uint32(48).int64(message.creationTime);
    }
    for (const v of message.tags) {
      ExperimentTag.encode(v!, writer.uint32(58).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Experiment {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseExperiment();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.experimentId = reader.string();
          break;
        case 2:
          message.name = reader.string();
          break;
        case 3:
          message.artifactLocation = reader.string();
          break;
        case 4:
          message.lifecycleStage = reader.string();
          break;
        case 5:
          message.lastUpdateTime = longToNumber(reader.int64() as Long);
          break;
        case 6:
          message.creationTime = longToNumber(reader.int64() as Long);
          break;
        case 7:
          message.tags.push(ExperimentTag.decode(reader, reader.uint32()));
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): Experiment {
    return {
      experimentId: isSet(object.experimentId)
        ? String(object.experimentId)
        : "",
      name: isSet(object.name) ? String(object.name) : "",
      artifactLocation: isSet(object.artifactLocation)
        ? String(object.artifactLocation)
        : "",
      lifecycleStage: isSet(object.lifecycleStage)
        ? String(object.lifecycleStage)
        : "",
      lastUpdateTime: isSet(object.lastUpdateTime)
        ? Number(object.lastUpdateTime)
        : 0,
      creationTime: isSet(object.creationTime)
        ? Number(object.creationTime)
        : 0,
      tags: Array.isArray(object?.tags)
        ? object.tags.map((e: any) => ExperimentTag.fromJSON(e))
        : [],
    };
  },

  toJSON(message: Experiment): unknown {
    const obj: any = {};
    message.experimentId !== undefined &&
      (obj.experimentId = message.experimentId);
    message.name !== undefined && (obj.name = message.name);
    message.artifactLocation !== undefined &&
      (obj.artifactLocation = message.artifactLocation);
    message.lifecycleStage !== undefined &&
      (obj.lifecycleStage = message.lifecycleStage);
    message.lastUpdateTime !== undefined &&
      (obj.lastUpdateTime = Math.round(message.lastUpdateTime));
    message.creationTime !== undefined &&
      (obj.creationTime = Math.round(message.creationTime));
    if (message.tags) {
      obj.tags = message.tags.map((e) =>
        e ? ExperimentTag.toJSON(e) : undefined
      );
    } else {
      obj.tags = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<Experiment>, I>>(
    object: I
  ): Experiment {
    const message = createBaseExperiment();
    message.experimentId = object.experimentId ?? "";
    message.name = object.name ?? "";
    message.artifactLocation = object.artifactLocation ?? "";
    message.lifecycleStage = object.lifecycleStage ?? "";
    message.lastUpdateTime = object.lastUpdateTime ?? 0;
    message.creationTime = object.creationTime ?? 0;
    message.tags = object.tags?.map((e) => ExperimentTag.fromPartial(e)) || [];
    return message;
  },
};

function createBaseCreateExperiment(): CreateExperiment {
  return { name: "", artifactLocation: "", tags: [] };
}

export const CreateExperiment = {
  encode(
    message: CreateExperiment,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.name !== "") {
      writer.uint32(10).string(message.name);
    }
    if (message.artifactLocation !== "") {
      writer.uint32(18).string(message.artifactLocation);
    }
    for (const v of message.tags) {
      ExperimentTag.encode(v!, writer.uint32(26).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): CreateExperiment {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseCreateExperiment();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.name = reader.string();
          break;
        case 2:
          message.artifactLocation = reader.string();
          break;
        case 3:
          message.tags.push(ExperimentTag.decode(reader, reader.uint32()));
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): CreateExperiment {
    return {
      name: isSet(object.name) ? String(object.name) : "",
      artifactLocation: isSet(object.artifactLocation)
        ? String(object.artifactLocation)
        : "",
      tags: Array.isArray(object?.tags)
        ? object.tags.map((e: any) => ExperimentTag.fromJSON(e))
        : [],
    };
  },

  toJSON(message: CreateExperiment): unknown {
    const obj: any = {};
    message.name !== undefined && (obj.name = message.name);
    message.artifactLocation !== undefined &&
      (obj.artifactLocation = message.artifactLocation);
    if (message.tags) {
      obj.tags = message.tags.map((e) =>
        e ? ExperimentTag.toJSON(e) : undefined
      );
    } else {
      obj.tags = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<CreateExperiment>, I>>(
    object: I
  ): CreateExperiment {
    const message = createBaseCreateExperiment();
    message.name = object.name ?? "";
    message.artifactLocation = object.artifactLocation ?? "";
    message.tags = object.tags?.map((e) => ExperimentTag.fromPartial(e)) || [];
    return message;
  },
};

function createBaseCreateExperiment_Response(): CreateExperiment_Response {
  return { experimentId: "" };
}

export const CreateExperiment_Response = {
  encode(
    message: CreateExperiment_Response,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.experimentId !== "") {
      writer.uint32(10).string(message.experimentId);
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): CreateExperiment_Response {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseCreateExperiment_Response();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.experimentId = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): CreateExperiment_Response {
    return {
      experimentId: isSet(object.experimentId)
        ? String(object.experimentId)
        : "",
    };
  },

  toJSON(message: CreateExperiment_Response): unknown {
    const obj: any = {};
    message.experimentId !== undefined &&
      (obj.experimentId = message.experimentId);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<CreateExperiment_Response>, I>>(
    object: I
  ): CreateExperiment_Response {
    const message = createBaseCreateExperiment_Response();
    message.experimentId = object.experimentId ?? "";
    return message;
  },
};

function createBaseListExperiments(): ListExperiments {
  return { viewType: 1, maxResults: 0, pageToken: "" };
}

export const ListExperiments = {
  encode(
    message: ListExperiments,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.viewType !== 1) {
      writer.uint32(8).int32(message.viewType);
    }
    if (message.maxResults !== 0) {
      writer.uint32(16).int64(message.maxResults);
    }
    if (message.pageToken !== "") {
      writer.uint32(26).string(message.pageToken);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ListExperiments {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseListExperiments();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.viewType = reader.int32() as any;
          break;
        case 2:
          message.maxResults = longToNumber(reader.int64() as Long);
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

  fromJSON(object: any): ListExperiments {
    return {
      viewType: isSet(object.viewType) ? viewTypeFromJSON(object.viewType) : 1,
      maxResults: isSet(object.maxResults) ? Number(object.maxResults) : 0,
      pageToken: isSet(object.pageToken) ? String(object.pageToken) : "",
    };
  },

  toJSON(message: ListExperiments): unknown {
    const obj: any = {};
    message.viewType !== undefined &&
      (obj.viewType = viewTypeToJSON(message.viewType));
    message.maxResults !== undefined &&
      (obj.maxResults = Math.round(message.maxResults));
    message.pageToken !== undefined && (obj.pageToken = message.pageToken);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ListExperiments>, I>>(
    object: I
  ): ListExperiments {
    const message = createBaseListExperiments();
    message.viewType = object.viewType ?? 1;
    message.maxResults = object.maxResults ?? 0;
    message.pageToken = object.pageToken ?? "";
    return message;
  },
};

function createBaseListExperiments_Response(): ListExperiments_Response {
  return { experiments: [], nextPageToken: "" };
}

export const ListExperiments_Response = {
  encode(
    message: ListExperiments_Response,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    for (const v of message.experiments) {
      Experiment.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    if (message.nextPageToken !== "") {
      writer.uint32(18).string(message.nextPageToken);
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): ListExperiments_Response {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseListExperiments_Response();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.experiments.push(Experiment.decode(reader, reader.uint32()));
          break;
        case 2:
          message.nextPageToken = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): ListExperiments_Response {
    return {
      experiments: Array.isArray(object?.experiments)
        ? object.experiments.map((e: any) => Experiment.fromJSON(e))
        : [],
      nextPageToken: isSet(object.nextPageToken)
        ? String(object.nextPageToken)
        : "",
    };
  },

  toJSON(message: ListExperiments_Response): unknown {
    const obj: any = {};
    if (message.experiments) {
      obj.experiments = message.experiments.map((e) =>
        e ? Experiment.toJSON(e) : undefined
      );
    } else {
      obj.experiments = [];
    }
    message.nextPageToken !== undefined &&
      (obj.nextPageToken = message.nextPageToken);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ListExperiments_Response>, I>>(
    object: I
  ): ListExperiments_Response {
    const message = createBaseListExperiments_Response();
    message.experiments =
      object.experiments?.map((e) => Experiment.fromPartial(e)) || [];
    message.nextPageToken = object.nextPageToken ?? "";
    return message;
  },
};

function createBaseGetExperiment(): GetExperiment {
  return { experimentId: "" };
}

export const GetExperiment = {
  encode(
    message: GetExperiment,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.experimentId !== "") {
      writer.uint32(10).string(message.experimentId);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): GetExperiment {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseGetExperiment();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.experimentId = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): GetExperiment {
    return {
      experimentId: isSet(object.experimentId)
        ? String(object.experimentId)
        : "",
    };
  },

  toJSON(message: GetExperiment): unknown {
    const obj: any = {};
    message.experimentId !== undefined &&
      (obj.experimentId = message.experimentId);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<GetExperiment>, I>>(
    object: I
  ): GetExperiment {
    const message = createBaseGetExperiment();
    message.experimentId = object.experimentId ?? "";
    return message;
  },
};

function createBaseGetExperiment_Response(): GetExperiment_Response {
  return { experiment: undefined, runs: [] };
}

export const GetExperiment_Response = {
  encode(
    message: GetExperiment_Response,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.experiment !== undefined) {
      Experiment.encode(message.experiment, writer.uint32(10).fork()).ldelim();
    }
    for (const v of message.runs) {
      RunInfo.encode(v!, writer.uint32(18).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): GetExperiment_Response {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseGetExperiment_Response();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.experiment = Experiment.decode(reader, reader.uint32());
          break;
        case 2:
          message.runs.push(RunInfo.decode(reader, reader.uint32()));
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): GetExperiment_Response {
    return {
      experiment: isSet(object.experiment)
        ? Experiment.fromJSON(object.experiment)
        : undefined,
      runs: Array.isArray(object?.runs)
        ? object.runs.map((e: any) => RunInfo.fromJSON(e))
        : [],
    };
  },

  toJSON(message: GetExperiment_Response): unknown {
    const obj: any = {};
    message.experiment !== undefined &&
      (obj.experiment = message.experiment
        ? Experiment.toJSON(message.experiment)
        : undefined);
    if (message.runs) {
      obj.runs = message.runs.map((e) => (e ? RunInfo.toJSON(e) : undefined));
    } else {
      obj.runs = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<GetExperiment_Response>, I>>(
    object: I
  ): GetExperiment_Response {
    const message = createBaseGetExperiment_Response();
    message.experiment =
      object.experiment !== undefined && object.experiment !== null
        ? Experiment.fromPartial(object.experiment)
        : undefined;
    message.runs = object.runs?.map((e) => RunInfo.fromPartial(e)) || [];
    return message;
  },
};

function createBaseDeleteExperiment(): DeleteExperiment {
  return { experimentId: "" };
}

export const DeleteExperiment = {
  encode(
    message: DeleteExperiment,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.experimentId !== "") {
      writer.uint32(10).string(message.experimentId);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): DeleteExperiment {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDeleteExperiment();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.experimentId = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): DeleteExperiment {
    return {
      experimentId: isSet(object.experimentId)
        ? String(object.experimentId)
        : "",
    };
  },

  toJSON(message: DeleteExperiment): unknown {
    const obj: any = {};
    message.experimentId !== undefined &&
      (obj.experimentId = message.experimentId);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<DeleteExperiment>, I>>(
    object: I
  ): DeleteExperiment {
    const message = createBaseDeleteExperiment();
    message.experimentId = object.experimentId ?? "";
    return message;
  },
};

function createBaseDeleteExperiment_Response(): DeleteExperiment_Response {
  return {};
}

export const DeleteExperiment_Response = {
  encode(
    _: DeleteExperiment_Response,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): DeleteExperiment_Response {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDeleteExperiment_Response();
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

  fromJSON(_: any): DeleteExperiment_Response {
    return {};
  },

  toJSON(_: DeleteExperiment_Response): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<DeleteExperiment_Response>, I>>(
    _: I
  ): DeleteExperiment_Response {
    const message = createBaseDeleteExperiment_Response();
    return message;
  },
};

function createBaseRestoreExperiment(): RestoreExperiment {
  return { experimentId: "" };
}

export const RestoreExperiment = {
  encode(
    message: RestoreExperiment,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.experimentId !== "") {
      writer.uint32(10).string(message.experimentId);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): RestoreExperiment {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRestoreExperiment();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.experimentId = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): RestoreExperiment {
    return {
      experimentId: isSet(object.experimentId)
        ? String(object.experimentId)
        : "",
    };
  },

  toJSON(message: RestoreExperiment): unknown {
    const obj: any = {};
    message.experimentId !== undefined &&
      (obj.experimentId = message.experimentId);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<RestoreExperiment>, I>>(
    object: I
  ): RestoreExperiment {
    const message = createBaseRestoreExperiment();
    message.experimentId = object.experimentId ?? "";
    return message;
  },
};

function createBaseRestoreExperiment_Response(): RestoreExperiment_Response {
  return {};
}

export const RestoreExperiment_Response = {
  encode(
    _: RestoreExperiment_Response,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): RestoreExperiment_Response {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRestoreExperiment_Response();
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

  fromJSON(_: any): RestoreExperiment_Response {
    return {};
  },

  toJSON(_: RestoreExperiment_Response): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<RestoreExperiment_Response>, I>>(
    _: I
  ): RestoreExperiment_Response {
    const message = createBaseRestoreExperiment_Response();
    return message;
  },
};

function createBaseUpdateExperiment(): UpdateExperiment {
  return { experimentId: "", newName: "" };
}

export const UpdateExperiment = {
  encode(
    message: UpdateExperiment,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.experimentId !== "") {
      writer.uint32(10).string(message.experimentId);
    }
    if (message.newName !== "") {
      writer.uint32(18).string(message.newName);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UpdateExperiment {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUpdateExperiment();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.experimentId = reader.string();
          break;
        case 2:
          message.newName = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): UpdateExperiment {
    return {
      experimentId: isSet(object.experimentId)
        ? String(object.experimentId)
        : "",
      newName: isSet(object.newName) ? String(object.newName) : "",
    };
  },

  toJSON(message: UpdateExperiment): unknown {
    const obj: any = {};
    message.experimentId !== undefined &&
      (obj.experimentId = message.experimentId);
    message.newName !== undefined && (obj.newName = message.newName);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<UpdateExperiment>, I>>(
    object: I
  ): UpdateExperiment {
    const message = createBaseUpdateExperiment();
    message.experimentId = object.experimentId ?? "";
    message.newName = object.newName ?? "";
    return message;
  },
};

function createBaseUpdateExperiment_Response(): UpdateExperiment_Response {
  return {};
}

export const UpdateExperiment_Response = {
  encode(
    _: UpdateExperiment_Response,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): UpdateExperiment_Response {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUpdateExperiment_Response();
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

  fromJSON(_: any): UpdateExperiment_Response {
    return {};
  },

  toJSON(_: UpdateExperiment_Response): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<UpdateExperiment_Response>, I>>(
    _: I
  ): UpdateExperiment_Response {
    const message = createBaseUpdateExperiment_Response();
    return message;
  },
};

function createBaseCreateRun(): CreateRun {
  return { experimentId: "", userId: "", startTime: 0, tags: [] };
}

export const CreateRun = {
  encode(
    message: CreateRun,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.experimentId !== "") {
      writer.uint32(10).string(message.experimentId);
    }
    if (message.userId !== "") {
      writer.uint32(18).string(message.userId);
    }
    if (message.startTime !== 0) {
      writer.uint32(56).int64(message.startTime);
    }
    for (const v of message.tags) {
      RunTag.encode(v!, writer.uint32(74).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): CreateRun {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseCreateRun();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.experimentId = reader.string();
          break;
        case 2:
          message.userId = reader.string();
          break;
        case 7:
          message.startTime = longToNumber(reader.int64() as Long);
          break;
        case 9:
          message.tags.push(RunTag.decode(reader, reader.uint32()));
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): CreateRun {
    return {
      experimentId: isSet(object.experimentId)
        ? String(object.experimentId)
        : "",
      userId: isSet(object.userId) ? String(object.userId) : "",
      startTime: isSet(object.startTime) ? Number(object.startTime) : 0,
      tags: Array.isArray(object?.tags)
        ? object.tags.map((e: any) => RunTag.fromJSON(e))
        : [],
    };
  },

  toJSON(message: CreateRun): unknown {
    const obj: any = {};
    message.experimentId !== undefined &&
      (obj.experimentId = message.experimentId);
    message.userId !== undefined && (obj.userId = message.userId);
    message.startTime !== undefined &&
      (obj.startTime = Math.round(message.startTime));
    if (message.tags) {
      obj.tags = message.tags.map((e) => (e ? RunTag.toJSON(e) : undefined));
    } else {
      obj.tags = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<CreateRun>, I>>(
    object: I
  ): CreateRun {
    const message = createBaseCreateRun();
    message.experimentId = object.experimentId ?? "";
    message.userId = object.userId ?? "";
    message.startTime = object.startTime ?? 0;
    message.tags = object.tags?.map((e) => RunTag.fromPartial(e)) || [];
    return message;
  },
};

function createBaseCreateRun_Response(): CreateRun_Response {
  return { run: undefined };
}

export const CreateRun_Response = {
  encode(
    message: CreateRun_Response,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.run !== undefined) {
      Run.encode(message.run, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): CreateRun_Response {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseCreateRun_Response();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.run = Run.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): CreateRun_Response {
    return {
      run: isSet(object.run) ? Run.fromJSON(object.run) : undefined,
    };
  },

  toJSON(message: CreateRun_Response): unknown {
    const obj: any = {};
    message.run !== undefined &&
      (obj.run = message.run ? Run.toJSON(message.run) : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<CreateRun_Response>, I>>(
    object: I
  ): CreateRun_Response {
    const message = createBaseCreateRun_Response();
    message.run =
      object.run !== undefined && object.run !== null
        ? Run.fromPartial(object.run)
        : undefined;
    return message;
  },
};

function createBaseUpdateRun(): UpdateRun {
  return { runId: "", runUuid: "", status: 1, endTime: 0 };
}

export const UpdateRun = {
  encode(
    message: UpdateRun,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.runId !== "") {
      writer.uint32(34).string(message.runId);
    }
    if (message.runUuid !== "") {
      writer.uint32(10).string(message.runUuid);
    }
    if (message.status !== 1) {
      writer.uint32(16).int32(message.status);
    }
    if (message.endTime !== 0) {
      writer.uint32(24).int64(message.endTime);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UpdateRun {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUpdateRun();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 4:
          message.runId = reader.string();
          break;
        case 1:
          message.runUuid = reader.string();
          break;
        case 2:
          message.status = reader.int32() as any;
          break;
        case 3:
          message.endTime = longToNumber(reader.int64() as Long);
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): UpdateRun {
    return {
      runId: isSet(object.runId) ? String(object.runId) : "",
      runUuid: isSet(object.runUuid) ? String(object.runUuid) : "",
      status: isSet(object.status) ? runStatusFromJSON(object.status) : 1,
      endTime: isSet(object.endTime) ? Number(object.endTime) : 0,
    };
  },

  toJSON(message: UpdateRun): unknown {
    const obj: any = {};
    message.runId !== undefined && (obj.runId = message.runId);
    message.runUuid !== undefined && (obj.runUuid = message.runUuid);
    message.status !== undefined &&
      (obj.status = runStatusToJSON(message.status));
    message.endTime !== undefined &&
      (obj.endTime = Math.round(message.endTime));
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<UpdateRun>, I>>(
    object: I
  ): UpdateRun {
    const message = createBaseUpdateRun();
    message.runId = object.runId ?? "";
    message.runUuid = object.runUuid ?? "";
    message.status = object.status ?? 1;
    message.endTime = object.endTime ?? 0;
    return message;
  },
};

function createBaseUpdateRun_Response(): UpdateRun_Response {
  return { runInfo: undefined };
}

export const UpdateRun_Response = {
  encode(
    message: UpdateRun_Response,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.runInfo !== undefined) {
      RunInfo.encode(message.runInfo, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): UpdateRun_Response {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseUpdateRun_Response();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.runInfo = RunInfo.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): UpdateRun_Response {
    return {
      runInfo: isSet(object.runInfo)
        ? RunInfo.fromJSON(object.runInfo)
        : undefined,
    };
  },

  toJSON(message: UpdateRun_Response): unknown {
    const obj: any = {};
    message.runInfo !== undefined &&
      (obj.runInfo = message.runInfo
        ? RunInfo.toJSON(message.runInfo)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<UpdateRun_Response>, I>>(
    object: I
  ): UpdateRun_Response {
    const message = createBaseUpdateRun_Response();
    message.runInfo =
      object.runInfo !== undefined && object.runInfo !== null
        ? RunInfo.fromPartial(object.runInfo)
        : undefined;
    return message;
  },
};

function createBaseDeleteRun(): DeleteRun {
  return { runId: "" };
}

export const DeleteRun = {
  encode(
    message: DeleteRun,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.runId !== "") {
      writer.uint32(10).string(message.runId);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): DeleteRun {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDeleteRun();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.runId = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): DeleteRun {
    return {
      runId: isSet(object.runId) ? String(object.runId) : "",
    };
  },

  toJSON(message: DeleteRun): unknown {
    const obj: any = {};
    message.runId !== undefined && (obj.runId = message.runId);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<DeleteRun>, I>>(
    object: I
  ): DeleteRun {
    const message = createBaseDeleteRun();
    message.runId = object.runId ?? "";
    return message;
  },
};

function createBaseDeleteRun_Response(): DeleteRun_Response {
  return {};
}

export const DeleteRun_Response = {
  encode(
    _: DeleteRun_Response,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): DeleteRun_Response {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDeleteRun_Response();
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

  fromJSON(_: any): DeleteRun_Response {
    return {};
  },

  toJSON(_: DeleteRun_Response): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<DeleteRun_Response>, I>>(
    _: I
  ): DeleteRun_Response {
    const message = createBaseDeleteRun_Response();
    return message;
  },
};

function createBaseRestoreRun(): RestoreRun {
  return { runId: "" };
}

export const RestoreRun = {
  encode(
    message: RestoreRun,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.runId !== "") {
      writer.uint32(10).string(message.runId);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): RestoreRun {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRestoreRun();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.runId = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): RestoreRun {
    return {
      runId: isSet(object.runId) ? String(object.runId) : "",
    };
  },

  toJSON(message: RestoreRun): unknown {
    const obj: any = {};
    message.runId !== undefined && (obj.runId = message.runId);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<RestoreRun>, I>>(
    object: I
  ): RestoreRun {
    const message = createBaseRestoreRun();
    message.runId = object.runId ?? "";
    return message;
  },
};

function createBaseRestoreRun_Response(): RestoreRun_Response {
  return {};
}

export const RestoreRun_Response = {
  encode(
    _: RestoreRun_Response,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): RestoreRun_Response {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseRestoreRun_Response();
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

  fromJSON(_: any): RestoreRun_Response {
    return {};
  },

  toJSON(_: RestoreRun_Response): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<RestoreRun_Response>, I>>(
    _: I
  ): RestoreRun_Response {
    const message = createBaseRestoreRun_Response();
    return message;
  },
};

function createBaseLogMetric(): LogMetric {
  return { runId: "", runUuid: "", key: "", value: 0, timestamp: 0, step: 0 };
}

export const LogMetric = {
  encode(
    message: LogMetric,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.runId !== "") {
      writer.uint32(50).string(message.runId);
    }
    if (message.runUuid !== "") {
      writer.uint32(10).string(message.runUuid);
    }
    if (message.key !== "") {
      writer.uint32(18).string(message.key);
    }
    if (message.value !== 0) {
      writer.uint32(25).double(message.value);
    }
    if (message.timestamp !== 0) {
      writer.uint32(32).int64(message.timestamp);
    }
    if (message.step !== 0) {
      writer.uint32(40).int64(message.step);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): LogMetric {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseLogMetric();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 6:
          message.runId = reader.string();
          break;
        case 1:
          message.runUuid = reader.string();
          break;
        case 2:
          message.key = reader.string();
          break;
        case 3:
          message.value = reader.double();
          break;
        case 4:
          message.timestamp = longToNumber(reader.int64() as Long);
          break;
        case 5:
          message.step = longToNumber(reader.int64() as Long);
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): LogMetric {
    return {
      runId: isSet(object.runId) ? String(object.runId) : "",
      runUuid: isSet(object.runUuid) ? String(object.runUuid) : "",
      key: isSet(object.key) ? String(object.key) : "",
      value: isSet(object.value) ? Number(object.value) : 0,
      timestamp: isSet(object.timestamp) ? Number(object.timestamp) : 0,
      step: isSet(object.step) ? Number(object.step) : 0,
    };
  },

  toJSON(message: LogMetric): unknown {
    const obj: any = {};
    message.runId !== undefined && (obj.runId = message.runId);
    message.runUuid !== undefined && (obj.runUuid = message.runUuid);
    message.key !== undefined && (obj.key = message.key);
    message.value !== undefined && (obj.value = message.value);
    message.timestamp !== undefined &&
      (obj.timestamp = Math.round(message.timestamp));
    message.step !== undefined && (obj.step = Math.round(message.step));
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<LogMetric>, I>>(
    object: I
  ): LogMetric {
    const message = createBaseLogMetric();
    message.runId = object.runId ?? "";
    message.runUuid = object.runUuid ?? "";
    message.key = object.key ?? "";
    message.value = object.value ?? 0;
    message.timestamp = object.timestamp ?? 0;
    message.step = object.step ?? 0;
    return message;
  },
};

function createBaseLogMetric_Response(): LogMetric_Response {
  return {};
}

export const LogMetric_Response = {
  encode(
    _: LogMetric_Response,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): LogMetric_Response {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseLogMetric_Response();
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

  fromJSON(_: any): LogMetric_Response {
    return {};
  },

  toJSON(_: LogMetric_Response): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<LogMetric_Response>, I>>(
    _: I
  ): LogMetric_Response {
    const message = createBaseLogMetric_Response();
    return message;
  },
};

function createBaseLogParam(): LogParam {
  return { runId: "", runUuid: "", key: "", value: "" };
}

export const LogParam = {
  encode(
    message: LogParam,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.runId !== "") {
      writer.uint32(34).string(message.runId);
    }
    if (message.runUuid !== "") {
      writer.uint32(10).string(message.runUuid);
    }
    if (message.key !== "") {
      writer.uint32(18).string(message.key);
    }
    if (message.value !== "") {
      writer.uint32(26).string(message.value);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): LogParam {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseLogParam();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 4:
          message.runId = reader.string();
          break;
        case 1:
          message.runUuid = reader.string();
          break;
        case 2:
          message.key = reader.string();
          break;
        case 3:
          message.value = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): LogParam {
    return {
      runId: isSet(object.runId) ? String(object.runId) : "",
      runUuid: isSet(object.runUuid) ? String(object.runUuid) : "",
      key: isSet(object.key) ? String(object.key) : "",
      value: isSet(object.value) ? String(object.value) : "",
    };
  },

  toJSON(message: LogParam): unknown {
    const obj: any = {};
    message.runId !== undefined && (obj.runId = message.runId);
    message.runUuid !== undefined && (obj.runUuid = message.runUuid);
    message.key !== undefined && (obj.key = message.key);
    message.value !== undefined && (obj.value = message.value);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<LogParam>, I>>(object: I): LogParam {
    const message = createBaseLogParam();
    message.runId = object.runId ?? "";
    message.runUuid = object.runUuid ?? "";
    message.key = object.key ?? "";
    message.value = object.value ?? "";
    return message;
  },
};

function createBaseLogParam_Response(): LogParam_Response {
  return {};
}

export const LogParam_Response = {
  encode(
    _: LogParam_Response,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): LogParam_Response {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseLogParam_Response();
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

  fromJSON(_: any): LogParam_Response {
    return {};
  },

  toJSON(_: LogParam_Response): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<LogParam_Response>, I>>(
    _: I
  ): LogParam_Response {
    const message = createBaseLogParam_Response();
    return message;
  },
};

function createBaseSetExperimentTag(): SetExperimentTag {
  return { experimentId: "", key: "", value: "" };
}

export const SetExperimentTag = {
  encode(
    message: SetExperimentTag,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.experimentId !== "") {
      writer.uint32(10).string(message.experimentId);
    }
    if (message.key !== "") {
      writer.uint32(18).string(message.key);
    }
    if (message.value !== "") {
      writer.uint32(26).string(message.value);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): SetExperimentTag {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSetExperimentTag();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.experimentId = reader.string();
          break;
        case 2:
          message.key = reader.string();
          break;
        case 3:
          message.value = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): SetExperimentTag {
    return {
      experimentId: isSet(object.experimentId)
        ? String(object.experimentId)
        : "",
      key: isSet(object.key) ? String(object.key) : "",
      value: isSet(object.value) ? String(object.value) : "",
    };
  },

  toJSON(message: SetExperimentTag): unknown {
    const obj: any = {};
    message.experimentId !== undefined &&
      (obj.experimentId = message.experimentId);
    message.key !== undefined && (obj.key = message.key);
    message.value !== undefined && (obj.value = message.value);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<SetExperimentTag>, I>>(
    object: I
  ): SetExperimentTag {
    const message = createBaseSetExperimentTag();
    message.experimentId = object.experimentId ?? "";
    message.key = object.key ?? "";
    message.value = object.value ?? "";
    return message;
  },
};

function createBaseSetExperimentTag_Response(): SetExperimentTag_Response {
  return {};
}

export const SetExperimentTag_Response = {
  encode(
    _: SetExperimentTag_Response,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): SetExperimentTag_Response {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSetExperimentTag_Response();
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

  fromJSON(_: any): SetExperimentTag_Response {
    return {};
  },

  toJSON(_: SetExperimentTag_Response): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<SetExperimentTag_Response>, I>>(
    _: I
  ): SetExperimentTag_Response {
    const message = createBaseSetExperimentTag_Response();
    return message;
  },
};

function createBaseSetTag(): SetTag {
  return { runId: "", runUuid: "", key: "", value: "" };
}

export const SetTag = {
  encode(
    message: SetTag,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.runId !== "") {
      writer.uint32(34).string(message.runId);
    }
    if (message.runUuid !== "") {
      writer.uint32(10).string(message.runUuid);
    }
    if (message.key !== "") {
      writer.uint32(18).string(message.key);
    }
    if (message.value !== "") {
      writer.uint32(26).string(message.value);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): SetTag {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSetTag();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 4:
          message.runId = reader.string();
          break;
        case 1:
          message.runUuid = reader.string();
          break;
        case 2:
          message.key = reader.string();
          break;
        case 3:
          message.value = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): SetTag {
    return {
      runId: isSet(object.runId) ? String(object.runId) : "",
      runUuid: isSet(object.runUuid) ? String(object.runUuid) : "",
      key: isSet(object.key) ? String(object.key) : "",
      value: isSet(object.value) ? String(object.value) : "",
    };
  },

  toJSON(message: SetTag): unknown {
    const obj: any = {};
    message.runId !== undefined && (obj.runId = message.runId);
    message.runUuid !== undefined && (obj.runUuid = message.runUuid);
    message.key !== undefined && (obj.key = message.key);
    message.value !== undefined && (obj.value = message.value);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<SetTag>, I>>(object: I): SetTag {
    const message = createBaseSetTag();
    message.runId = object.runId ?? "";
    message.runUuid = object.runUuid ?? "";
    message.key = object.key ?? "";
    message.value = object.value ?? "";
    return message;
  },
};

function createBaseSetTag_Response(): SetTag_Response {
  return {};
}

export const SetTag_Response = {
  encode(
    _: SetTag_Response,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): SetTag_Response {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSetTag_Response();
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

  fromJSON(_: any): SetTag_Response {
    return {};
  },

  toJSON(_: SetTag_Response): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<SetTag_Response>, I>>(
    _: I
  ): SetTag_Response {
    const message = createBaseSetTag_Response();
    return message;
  },
};

function createBaseDeleteTag(): DeleteTag {
  return { runId: "", key: "" };
}

export const DeleteTag = {
  encode(
    message: DeleteTag,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.runId !== "") {
      writer.uint32(10).string(message.runId);
    }
    if (message.key !== "") {
      writer.uint32(18).string(message.key);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): DeleteTag {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDeleteTag();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.runId = reader.string();
          break;
        case 2:
          message.key = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): DeleteTag {
    return {
      runId: isSet(object.runId) ? String(object.runId) : "",
      key: isSet(object.key) ? String(object.key) : "",
    };
  },

  toJSON(message: DeleteTag): unknown {
    const obj: any = {};
    message.runId !== undefined && (obj.runId = message.runId);
    message.key !== undefined && (obj.key = message.key);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<DeleteTag>, I>>(
    object: I
  ): DeleteTag {
    const message = createBaseDeleteTag();
    message.runId = object.runId ?? "";
    message.key = object.key ?? "";
    return message;
  },
};

function createBaseDeleteTag_Response(): DeleteTag_Response {
  return {};
}

export const DeleteTag_Response = {
  encode(
    _: DeleteTag_Response,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): DeleteTag_Response {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseDeleteTag_Response();
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

  fromJSON(_: any): DeleteTag_Response {
    return {};
  },

  toJSON(_: DeleteTag_Response): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<DeleteTag_Response>, I>>(
    _: I
  ): DeleteTag_Response {
    const message = createBaseDeleteTag_Response();
    return message;
  },
};

function createBaseGetRun(): GetRun {
  return { runId: "", runUuid: "" };
}

export const GetRun = {
  encode(
    message: GetRun,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.runId !== "") {
      writer.uint32(18).string(message.runId);
    }
    if (message.runUuid !== "") {
      writer.uint32(10).string(message.runUuid);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): GetRun {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseGetRun();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 2:
          message.runId = reader.string();
          break;
        case 1:
          message.runUuid = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): GetRun {
    return {
      runId: isSet(object.runId) ? String(object.runId) : "",
      runUuid: isSet(object.runUuid) ? String(object.runUuid) : "",
    };
  },

  toJSON(message: GetRun): unknown {
    const obj: any = {};
    message.runId !== undefined && (obj.runId = message.runId);
    message.runUuid !== undefined && (obj.runUuid = message.runUuid);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<GetRun>, I>>(object: I): GetRun {
    const message = createBaseGetRun();
    message.runId = object.runId ?? "";
    message.runUuid = object.runUuid ?? "";
    return message;
  },
};

function createBaseGetRun_Response(): GetRun_Response {
  return { run: undefined };
}

export const GetRun_Response = {
  encode(
    message: GetRun_Response,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.run !== undefined) {
      Run.encode(message.run, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): GetRun_Response {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseGetRun_Response();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.run = Run.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): GetRun_Response {
    return {
      run: isSet(object.run) ? Run.fromJSON(object.run) : undefined,
    };
  },

  toJSON(message: GetRun_Response): unknown {
    const obj: any = {};
    message.run !== undefined &&
      (obj.run = message.run ? Run.toJSON(message.run) : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<GetRun_Response>, I>>(
    object: I
  ): GetRun_Response {
    const message = createBaseGetRun_Response();
    message.run =
      object.run !== undefined && object.run !== null
        ? Run.fromPartial(object.run)
        : undefined;
    return message;
  },
};

function createBaseSearchRuns(): SearchRuns {
  return {
    experimentIds: [],
    filter: "",
    runViewType: 1,
    maxResults: 0,
    orderBy: [],
    pageToken: "",
  };
}

export const SearchRuns = {
  encode(
    message: SearchRuns,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    for (const v of message.experimentIds) {
      writer.uint32(10).string(v!);
    }
    if (message.filter !== "") {
      writer.uint32(34).string(message.filter);
    }
    if (message.runViewType !== 1) {
      writer.uint32(24).int32(message.runViewType);
    }
    if (message.maxResults !== 0) {
      writer.uint32(40).int32(message.maxResults);
    }
    for (const v of message.orderBy) {
      writer.uint32(50).string(v!);
    }
    if (message.pageToken !== "") {
      writer.uint32(58).string(message.pageToken);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): SearchRuns {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSearchRuns();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.experimentIds.push(reader.string());
          break;
        case 4:
          message.filter = reader.string();
          break;
        case 3:
          message.runViewType = reader.int32() as any;
          break;
        case 5:
          message.maxResults = reader.int32();
          break;
        case 6:
          message.orderBy.push(reader.string());
          break;
        case 7:
          message.pageToken = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): SearchRuns {
    return {
      experimentIds: Array.isArray(object?.experimentIds)
        ? object.experimentIds.map((e: any) => String(e))
        : [],
      filter: isSet(object.filter) ? String(object.filter) : "",
      runViewType: isSet(object.runViewType)
        ? viewTypeFromJSON(object.runViewType)
        : 1,
      maxResults: isSet(object.maxResults) ? Number(object.maxResults) : 0,
      orderBy: Array.isArray(object?.orderBy)
        ? object.orderBy.map((e: any) => String(e))
        : [],
      pageToken: isSet(object.pageToken) ? String(object.pageToken) : "",
    };
  },

  toJSON(message: SearchRuns): unknown {
    const obj: any = {};
    if (message.experimentIds) {
      obj.experimentIds = message.experimentIds.map((e) => e);
    } else {
      obj.experimentIds = [];
    }
    message.filter !== undefined && (obj.filter = message.filter);
    message.runViewType !== undefined &&
      (obj.runViewType = viewTypeToJSON(message.runViewType));
    message.maxResults !== undefined &&
      (obj.maxResults = Math.round(message.maxResults));
    if (message.orderBy) {
      obj.orderBy = message.orderBy.map((e) => e);
    } else {
      obj.orderBy = [];
    }
    message.pageToken !== undefined && (obj.pageToken = message.pageToken);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<SearchRuns>, I>>(
    object: I
  ): SearchRuns {
    const message = createBaseSearchRuns();
    message.experimentIds = object.experimentIds?.map((e) => e) || [];
    message.filter = object.filter ?? "";
    message.runViewType = object.runViewType ?? 1;
    message.maxResults = object.maxResults ?? 0;
    message.orderBy = object.orderBy?.map((e) => e) || [];
    message.pageToken = object.pageToken ?? "";
    return message;
  },
};

function createBaseSearchRuns_Response(): SearchRuns_Response {
  return { runs: [], nextPageToken: "" };
}

export const SearchRuns_Response = {
  encode(
    message: SearchRuns_Response,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    for (const v of message.runs) {
      Run.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    if (message.nextPageToken !== "") {
      writer.uint32(18).string(message.nextPageToken);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): SearchRuns_Response {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseSearchRuns_Response();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.runs.push(Run.decode(reader, reader.uint32()));
          break;
        case 2:
          message.nextPageToken = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): SearchRuns_Response {
    return {
      runs: Array.isArray(object?.runs)
        ? object.runs.map((e: any) => Run.fromJSON(e))
        : [],
      nextPageToken: isSet(object.nextPageToken)
        ? String(object.nextPageToken)
        : "",
    };
  },

  toJSON(message: SearchRuns_Response): unknown {
    const obj: any = {};
    if (message.runs) {
      obj.runs = message.runs.map((e) => (e ? Run.toJSON(e) : undefined));
    } else {
      obj.runs = [];
    }
    message.nextPageToken !== undefined &&
      (obj.nextPageToken = message.nextPageToken);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<SearchRuns_Response>, I>>(
    object: I
  ): SearchRuns_Response {
    const message = createBaseSearchRuns_Response();
    message.runs = object.runs?.map((e) => Run.fromPartial(e)) || [];
    message.nextPageToken = object.nextPageToken ?? "";
    return message;
  },
};

function createBaseListArtifacts(): ListArtifacts {
  return { runId: "", runUuid: "", path: "", pageToken: "" };
}

export const ListArtifacts = {
  encode(
    message: ListArtifacts,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.runId !== "") {
      writer.uint32(26).string(message.runId);
    }
    if (message.runUuid !== "") {
      writer.uint32(10).string(message.runUuid);
    }
    if (message.path !== "") {
      writer.uint32(18).string(message.path);
    }
    if (message.pageToken !== "") {
      writer.uint32(34).string(message.pageToken);
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
        case 3:
          message.runId = reader.string();
          break;
        case 1:
          message.runUuid = reader.string();
          break;
        case 2:
          message.path = reader.string();
          break;
        case 4:
          message.pageToken = reader.string();
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
      runId: isSet(object.runId) ? String(object.runId) : "",
      runUuid: isSet(object.runUuid) ? String(object.runUuid) : "",
      path: isSet(object.path) ? String(object.path) : "",
      pageToken: isSet(object.pageToken) ? String(object.pageToken) : "",
    };
  },

  toJSON(message: ListArtifacts): unknown {
    const obj: any = {};
    message.runId !== undefined && (obj.runId = message.runId);
    message.runUuid !== undefined && (obj.runUuid = message.runUuid);
    message.path !== undefined && (obj.path = message.path);
    message.pageToken !== undefined && (obj.pageToken = message.pageToken);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ListArtifacts>, I>>(
    object: I
  ): ListArtifacts {
    const message = createBaseListArtifacts();
    message.runId = object.runId ?? "";
    message.runUuid = object.runUuid ?? "";
    message.path = object.path ?? "";
    message.pageToken = object.pageToken ?? "";
    return message;
  },
};

function createBaseListArtifacts_Response(): ListArtifacts_Response {
  return { rootUri: "", files: [], nextPageToken: "" };
}

export const ListArtifacts_Response = {
  encode(
    message: ListArtifacts_Response,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.rootUri !== "") {
      writer.uint32(10).string(message.rootUri);
    }
    for (const v of message.files) {
      FileInfo.encode(v!, writer.uint32(18).fork()).ldelim();
    }
    if (message.nextPageToken !== "") {
      writer.uint32(26).string(message.nextPageToken);
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
          message.rootUri = reader.string();
          break;
        case 2:
          message.files.push(FileInfo.decode(reader, reader.uint32()));
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

  fromJSON(object: any): ListArtifacts_Response {
    return {
      rootUri: isSet(object.rootUri) ? String(object.rootUri) : "",
      files: Array.isArray(object?.files)
        ? object.files.map((e: any) => FileInfo.fromJSON(e))
        : [],
      nextPageToken: isSet(object.nextPageToken)
        ? String(object.nextPageToken)
        : "",
    };
  },

  toJSON(message: ListArtifacts_Response): unknown {
    const obj: any = {};
    message.rootUri !== undefined && (obj.rootUri = message.rootUri);
    if (message.files) {
      obj.files = message.files.map((e) =>
        e ? FileInfo.toJSON(e) : undefined
      );
    } else {
      obj.files = [];
    }
    message.nextPageToken !== undefined &&
      (obj.nextPageToken = message.nextPageToken);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<ListArtifacts_Response>, I>>(
    object: I
  ): ListArtifacts_Response {
    const message = createBaseListArtifacts_Response();
    message.rootUri = object.rootUri ?? "";
    message.files = object.files?.map((e) => FileInfo.fromPartial(e)) || [];
    message.nextPageToken = object.nextPageToken ?? "";
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

function createBaseGetMetricHistory(): GetMetricHistory {
  return { runId: "", runUuid: "", metricKey: "" };
}

export const GetMetricHistory = {
  encode(
    message: GetMetricHistory,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.runId !== "") {
      writer.uint32(26).string(message.runId);
    }
    if (message.runUuid !== "") {
      writer.uint32(10).string(message.runUuid);
    }
    if (message.metricKey !== "") {
      writer.uint32(18).string(message.metricKey);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): GetMetricHistory {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseGetMetricHistory();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 3:
          message.runId = reader.string();
          break;
        case 1:
          message.runUuid = reader.string();
          break;
        case 2:
          message.metricKey = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): GetMetricHistory {
    return {
      runId: isSet(object.runId) ? String(object.runId) : "",
      runUuid: isSet(object.runUuid) ? String(object.runUuid) : "",
      metricKey: isSet(object.metricKey) ? String(object.metricKey) : "",
    };
  },

  toJSON(message: GetMetricHistory): unknown {
    const obj: any = {};
    message.runId !== undefined && (obj.runId = message.runId);
    message.runUuid !== undefined && (obj.runUuid = message.runUuid);
    message.metricKey !== undefined && (obj.metricKey = message.metricKey);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<GetMetricHistory>, I>>(
    object: I
  ): GetMetricHistory {
    const message = createBaseGetMetricHistory();
    message.runId = object.runId ?? "";
    message.runUuid = object.runUuid ?? "";
    message.metricKey = object.metricKey ?? "";
    return message;
  },
};

function createBaseGetMetricHistory_Response(): GetMetricHistory_Response {
  return { metrics: [] };
}

export const GetMetricHistory_Response = {
  encode(
    message: GetMetricHistory_Response,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    for (const v of message.metrics) {
      Metric.encode(v!, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): GetMetricHistory_Response {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseGetMetricHistory_Response();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.metrics.push(Metric.decode(reader, reader.uint32()));
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): GetMetricHistory_Response {
    return {
      metrics: Array.isArray(object?.metrics)
        ? object.metrics.map((e: any) => Metric.fromJSON(e))
        : [],
    };
  },

  toJSON(message: GetMetricHistory_Response): unknown {
    const obj: any = {};
    if (message.metrics) {
      obj.metrics = message.metrics.map((e) =>
        e ? Metric.toJSON(e) : undefined
      );
    } else {
      obj.metrics = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<GetMetricHistory_Response>, I>>(
    object: I
  ): GetMetricHistory_Response {
    const message = createBaseGetMetricHistory_Response();
    message.metrics = object.metrics?.map((e) => Metric.fromPartial(e)) || [];
    return message;
  },
};

function createBaseLogBatch(): LogBatch {
  return { runId: "", metrics: [], params: [], tags: [] };
}

export const LogBatch = {
  encode(
    message: LogBatch,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.runId !== "") {
      writer.uint32(10).string(message.runId);
    }
    for (const v of message.metrics) {
      Metric.encode(v!, writer.uint32(18).fork()).ldelim();
    }
    for (const v of message.params) {
      Param.encode(v!, writer.uint32(26).fork()).ldelim();
    }
    for (const v of message.tags) {
      RunTag.encode(v!, writer.uint32(34).fork()).ldelim();
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): LogBatch {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseLogBatch();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.runId = reader.string();
          break;
        case 2:
          message.metrics.push(Metric.decode(reader, reader.uint32()));
          break;
        case 3:
          message.params.push(Param.decode(reader, reader.uint32()));
          break;
        case 4:
          message.tags.push(RunTag.decode(reader, reader.uint32()));
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): LogBatch {
    return {
      runId: isSet(object.runId) ? String(object.runId) : "",
      metrics: Array.isArray(object?.metrics)
        ? object.metrics.map((e: any) => Metric.fromJSON(e))
        : [],
      params: Array.isArray(object?.params)
        ? object.params.map((e: any) => Param.fromJSON(e))
        : [],
      tags: Array.isArray(object?.tags)
        ? object.tags.map((e: any) => RunTag.fromJSON(e))
        : [],
    };
  },

  toJSON(message: LogBatch): unknown {
    const obj: any = {};
    message.runId !== undefined && (obj.runId = message.runId);
    if (message.metrics) {
      obj.metrics = message.metrics.map((e) =>
        e ? Metric.toJSON(e) : undefined
      );
    } else {
      obj.metrics = [];
    }
    if (message.params) {
      obj.params = message.params.map((e) => (e ? Param.toJSON(e) : undefined));
    } else {
      obj.params = [];
    }
    if (message.tags) {
      obj.tags = message.tags.map((e) => (e ? RunTag.toJSON(e) : undefined));
    } else {
      obj.tags = [];
    }
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<LogBatch>, I>>(object: I): LogBatch {
    const message = createBaseLogBatch();
    message.runId = object.runId ?? "";
    message.metrics = object.metrics?.map((e) => Metric.fromPartial(e)) || [];
    message.params = object.params?.map((e) => Param.fromPartial(e)) || [];
    message.tags = object.tags?.map((e) => RunTag.fromPartial(e)) || [];
    return message;
  },
};

function createBaseLogBatch_Response(): LogBatch_Response {
  return {};
}

export const LogBatch_Response = {
  encode(
    _: LogBatch_Response,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): LogBatch_Response {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseLogBatch_Response();
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

  fromJSON(_: any): LogBatch_Response {
    return {};
  },

  toJSON(_: LogBatch_Response): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<LogBatch_Response>, I>>(
    _: I
  ): LogBatch_Response {
    const message = createBaseLogBatch_Response();
    return message;
  },
};

function createBaseLogModel(): LogModel {
  return { runId: "", modelJson: "" };
}

export const LogModel = {
  encode(
    message: LogModel,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.runId !== "") {
      writer.uint32(10).string(message.runId);
    }
    if (message.modelJson !== "") {
      writer.uint32(18).string(message.modelJson);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): LogModel {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseLogModel();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.runId = reader.string();
          break;
        case 2:
          message.modelJson = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): LogModel {
    return {
      runId: isSet(object.runId) ? String(object.runId) : "",
      modelJson: isSet(object.modelJson) ? String(object.modelJson) : "",
    };
  },

  toJSON(message: LogModel): unknown {
    const obj: any = {};
    message.runId !== undefined && (obj.runId = message.runId);
    message.modelJson !== undefined && (obj.modelJson = message.modelJson);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<LogModel>, I>>(object: I): LogModel {
    const message = createBaseLogModel();
    message.runId = object.runId ?? "";
    message.modelJson = object.modelJson ?? "";
    return message;
  },
};

function createBaseLogModel_Response(): LogModel_Response {
  return {};
}

export const LogModel_Response = {
  encode(
    _: LogModel_Response,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): LogModel_Response {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseLogModel_Response();
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

  fromJSON(_: any): LogModel_Response {
    return {};
  },

  toJSON(_: LogModel_Response): unknown {
    const obj: any = {};
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<LogModel_Response>, I>>(
    _: I
  ): LogModel_Response {
    const message = createBaseLogModel_Response();
    return message;
  },
};

function createBaseGetExperimentByName(): GetExperimentByName {
  return { experimentName: "" };
}

export const GetExperimentByName = {
  encode(
    message: GetExperimentByName,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.experimentName !== "") {
      writer.uint32(10).string(message.experimentName);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): GetExperimentByName {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseGetExperimentByName();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.experimentName = reader.string();
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): GetExperimentByName {
    return {
      experimentName: isSet(object.experimentName)
        ? String(object.experimentName)
        : "",
    };
  },

  toJSON(message: GetExperimentByName): unknown {
    const obj: any = {};
    message.experimentName !== undefined &&
      (obj.experimentName = message.experimentName);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<GetExperimentByName>, I>>(
    object: I
  ): GetExperimentByName {
    const message = createBaseGetExperimentByName();
    message.experimentName = object.experimentName ?? "";
    return message;
  },
};

function createBaseGetExperimentByName_Response(): GetExperimentByName_Response {
  return { experiment: undefined };
}

export const GetExperimentByName_Response = {
  encode(
    message: GetExperimentByName_Response,
    writer: _m0.Writer = _m0.Writer.create()
  ): _m0.Writer {
    if (message.experiment !== undefined) {
      Experiment.encode(message.experiment, writer.uint32(10).fork()).ldelim();
    }
    return writer;
  },

  decode(
    input: _m0.Reader | Uint8Array,
    length?: number
  ): GetExperimentByName_Response {
    const reader = input instanceof _m0.Reader ? input : new _m0.Reader(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseGetExperimentByName_Response();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          message.experiment = Experiment.decode(reader, reader.uint32());
          break;
        default:
          reader.skipType(tag & 7);
          break;
      }
    }
    return message;
  },

  fromJSON(object: any): GetExperimentByName_Response {
    return {
      experiment: isSet(object.experiment)
        ? Experiment.fromJSON(object.experiment)
        : undefined,
    };
  },

  toJSON(message: GetExperimentByName_Response): unknown {
    const obj: any = {};
    message.experiment !== undefined &&
      (obj.experiment = message.experiment
        ? Experiment.toJSON(message.experiment)
        : undefined);
    return obj;
  },

  fromPartial<I extends Exact<DeepPartial<GetExperimentByName_Response>, I>>(
    object: I
  ): GetExperimentByName_Response {
    const message = createBaseGetExperimentByName_Response();
    message.experiment =
      object.experiment !== undefined && object.experiment !== null
        ? Experiment.fromPartial(object.experiment)
        : undefined;
    return message;
  },
};

export interface MlflowService {
  /**
   * Get metadata for an experiment.
   *
   * This endpoint will return deleted experiments, but prefers the active experiment
   * if an active and deleted experiment share the same name. If multiple deleted
   * experiments share the same name, the API will return one of them.
   *
   * Throws ``RESOURCE_DOES_NOT_EXIST`` if no experiment with the specified name exists.
   */
  getExperimentByName(
    request: GetExperimentByName
  ): Promise<GetExperimentByName_Response>;
  /**
   * Create an experiment with a name. Returns the ID of the newly created experiment.
   * Validates that another experiment with the same name does not already exist and fails
   * if another experiment with the same name already exists.
   *
   *
   * Throws ``RESOURCE_ALREADY_EXISTS`` if a experiment with the given name exists.
   */
  createExperiment(
    request: CreateExperiment
  ): Promise<CreateExperiment_Response>;
  /** Get a list of all experiments. */
  listExperiments(request: ListExperiments): Promise<ListExperiments_Response>;
  /** Get metadata for an experiment. This method works on deleted experiments. */
  getExperiment(request: GetExperiment): Promise<GetExperiment_Response>;
  /**
   * Mark an experiment and associated metadata, runs, metrics, params, and tags for deletion.
   * If the experiment uses FileStore, artifacts associated with experiment are also deleted.
   */
  deleteExperiment(
    request: DeleteExperiment
  ): Promise<DeleteExperiment_Response>;
  /**
   * Restore an experiment marked for deletion. This also restores
   * associated metadata, runs, metrics, params, and tags. If experiment uses FileStore, underlying
   * artifacts associated with experiment are also restored.
   *
   * Throws ``RESOURCE_DOES_NOT_EXIST`` if experiment was never created or was permanently deleted.
   */
  restoreExperiment(
    request: RestoreExperiment
  ): Promise<RestoreExperiment_Response>;
  /** Update experiment metadata. */
  updateExperiment(
    request: UpdateExperiment
  ): Promise<UpdateExperiment_Response>;
  /**
   * Create a new run within an experiment. A run is usually a single execution of a
   * machine learning or data ETL pipeline. MLflow uses runs to track :ref:`mlflowParam`,
   * :ref:`mlflowMetric`, and :ref:`mlflowRunTag` associated with a single execution.
   */
  createRun(request: CreateRun): Promise<CreateRun_Response>;
  /** Update run metadata. */
  updateRun(request: UpdateRun): Promise<UpdateRun_Response>;
  /** Mark a run for deletion. */
  deleteRun(request: DeleteRun): Promise<DeleteRun_Response>;
  /** Restore a deleted run. */
  restoreRun(request: RestoreRun): Promise<RestoreRun_Response>;
  /**
   * Log a metric for a run. A metric is a key-value pair (string key, float value) with an
   * associated timestamp. Examples include the various metrics that represent ML model accuracy.
   * A metric can be logged multiple times.
   */
  logMetric(request: LogMetric): Promise<LogMetric_Response>;
  /**
   * Log a param used for a run. A param is a key-value pair (string key,
   * string value). Examples include hyperparameters used for ML model training and
   * constant dates and values used in an ETL pipeline. A param can be logged only once for a run.
   */
  logParam(request: LogParam): Promise<LogParam_Response>;
  /** Set a tag on an experiment. Experiment tags are metadata that can be updated. */
  setExperimentTag(
    request: SetExperimentTag
  ): Promise<SetExperimentTag_Response>;
  /**
   * Set a tag on a run. Tags are run metadata that can be updated during a run and after
   * a run completes.
   */
  setTag(request: SetTag): Promise<SetTag_Response>;
  /**
   * Delete a tag on a run. Tags are run metadata that can be updated during a run and after
   * a run completes.
   */
  deleteTag(request: DeleteTag): Promise<DeleteTag_Response>;
  /**
   * Get metadata, metrics, params, and tags for a run. In the case where multiple metrics
   * with the same key are logged for a run, return only the value with the latest timestamp.
   * If there are multiple values with the latest timestamp, return the maximum of these values.
   */
  getRun(request: GetRun): Promise<GetRun_Response>;
  /**
   * Search for runs that satisfy expressions. Search expressions can use :ref:`mlflowMetric` and
   * :ref:`mlflowParam` keys.
   */
  searchRuns(request: SearchRuns): Promise<SearchRuns_Response>;
  /**
   * List artifacts for a run. Takes an optional ``artifact_path`` prefix which if specified,
   * the response contains only artifacts with the specified prefix.
   */
  listArtifacts(request: ListArtifacts): Promise<ListArtifacts_Response>;
  /** Get a list of all values for the specified metric for a given run. */
  getMetricHistory(
    request: GetMetricHistory
  ): Promise<GetMetricHistory_Response>;
  /**
   * Log a batch of metrics, params, and tags for a run.
   * If any data failed to be persisted, the server will respond with an error (non-200 status code).
   * In case of error (due to internal server error or an invalid request), partial data may
   * be written.
   *
   * You can write metrics, params, and tags in interleaving fashion, but within a given entity
   * type are guaranteed to follow the order specified in the request body. That is, for an API
   * request like
   *
   * .. code-block:: json
   *
   *   {
   *      "run_id": "2a14ed5c6a87499199e0106c3501eab8",
   *      "metrics": [
   *        {"key": "mae", "value": 2.5, "timestamp": 1552550804},
   *        {"key": "rmse", "value": 2.7, "timestamp": 1552550804},
   *      ],
   *      "params": [
   *        {"key": "model_class", "value": "LogisticRegression"},
   *      ]
   *   }
   *
   * the server is guaranteed to write metric "rmse" after "mae", though it may write param
   * "model_class" before both metrics, after "mae", or after both metrics.
   *
   * The overwrite behavior for metrics, params, and tags is as follows:
   *
   * - Metrics: metric values are never overwritten. Logging a metric (key, value, timestamp) appends to the set of values for the metric with the provided key.
   *
   * - Tags: tag values can be overwritten by successive writes to the same tag key. That is, if multiple tag values with the same key are provided in the same API request, the last-provided tag value is written. Logging the same tag (key, value) is permitted - that is, logging a tag is idempotent.
   *
   * - Params: once written, param values cannot be changed (attempting to overwrite a param value will result in an error). However, logging the same param (key, value) is permitted - that is, logging a param is idempotent.
   *
   * Request Limits
   * --------------
   * A single JSON-serialized API request may be up to 1 MB in size and contain:
   *
   * - No more than 1000 metrics, params, and tags in total
   * - Up to 1000 metrics
   * - Up to 100 params
   * - Up to 100 tags
   *
   * For example, a valid request might contain 900 metrics, 50 params, and 50 tags, but logging
   * 900 metrics, 50 params, and 51 tags is invalid. The following limits also apply
   * to metric, param, and tag keys and values:
   *
   * - Metric, param, and tag keys can be up to 250 characters in length
   * - Param and tag values can be up to 250 characters in length
   */
  logBatch(request: LogBatch): Promise<LogBatch_Response>;
  /**
   * .. note::
   *     Experimental: This API may change or be removed in a future release without warning.
   */
  logModel(request: LogModel): Promise<LogModel_Response>;
}

export class MlflowServiceClientImpl implements MlflowService {
  private readonly rpc: Rpc;
  constructor(rpc: Rpc) {
    this.rpc = rpc;
    this.getExperimentByName = this.getExperimentByName.bind(this);
    this.createExperiment = this.createExperiment.bind(this);
    this.listExperiments = this.listExperiments.bind(this);
    this.getExperiment = this.getExperiment.bind(this);
    this.deleteExperiment = this.deleteExperiment.bind(this);
    this.restoreExperiment = this.restoreExperiment.bind(this);
    this.updateExperiment = this.updateExperiment.bind(this);
    this.createRun = this.createRun.bind(this);
    this.updateRun = this.updateRun.bind(this);
    this.deleteRun = this.deleteRun.bind(this);
    this.restoreRun = this.restoreRun.bind(this);
    this.logMetric = this.logMetric.bind(this);
    this.logParam = this.logParam.bind(this);
    this.setExperimentTag = this.setExperimentTag.bind(this);
    this.setTag = this.setTag.bind(this);
    this.deleteTag = this.deleteTag.bind(this);
    this.getRun = this.getRun.bind(this);
    this.searchRuns = this.searchRuns.bind(this);
    this.listArtifacts = this.listArtifacts.bind(this);
    this.getMetricHistory = this.getMetricHistory.bind(this);
    this.logBatch = this.logBatch.bind(this);
    this.logModel = this.logModel.bind(this);
  }
  getExperimentByName(
    request: GetExperimentByName
  ): Promise<GetExperimentByName_Response> {
    const data = GetExperimentByName.encode(request).finish();
    const promise = this.rpc.request(
      "mlflow.MlflowService",
      "getExperimentByName",
      data
    );
    return promise.then((data) =>
      GetExperimentByName_Response.decode(new _m0.Reader(data))
    );
  }

  createExperiment(
    request: CreateExperiment
  ): Promise<CreateExperiment_Response> {
    const data = CreateExperiment.encode(request).finish();
    const promise = this.rpc.request(
      "mlflow.MlflowService",
      "createExperiment",
      data
    );
    return promise.then((data) =>
      CreateExperiment_Response.decode(new _m0.Reader(data))
    );
  }

  listExperiments(request: ListExperiments): Promise<ListExperiments_Response> {
    const data = ListExperiments.encode(request).finish();
    const promise = this.rpc.request(
      "mlflow.MlflowService",
      "listExperiments",
      data
    );
    return promise.then((data) =>
      ListExperiments_Response.decode(new _m0.Reader(data))
    );
  }

  getExperiment(request: GetExperiment): Promise<GetExperiment_Response> {
    const data = GetExperiment.encode(request).finish();
    const promise = this.rpc.request(
      "mlflow.MlflowService",
      "getExperiment",
      data
    );
    return promise.then((data) =>
      GetExperiment_Response.decode(new _m0.Reader(data))
    );
  }

  deleteExperiment(
    request: DeleteExperiment
  ): Promise<DeleteExperiment_Response> {
    const data = DeleteExperiment.encode(request).finish();
    const promise = this.rpc.request(
      "mlflow.MlflowService",
      "deleteExperiment",
      data
    );
    return promise.then((data) =>
      DeleteExperiment_Response.decode(new _m0.Reader(data))
    );
  }

  restoreExperiment(
    request: RestoreExperiment
  ): Promise<RestoreExperiment_Response> {
    const data = RestoreExperiment.encode(request).finish();
    const promise = this.rpc.request(
      "mlflow.MlflowService",
      "restoreExperiment",
      data
    );
    return promise.then((data) =>
      RestoreExperiment_Response.decode(new _m0.Reader(data))
    );
  }

  updateExperiment(
    request: UpdateExperiment
  ): Promise<UpdateExperiment_Response> {
    const data = UpdateExperiment.encode(request).finish();
    const promise = this.rpc.request(
      "mlflow.MlflowService",
      "updateExperiment",
      data
    );
    return promise.then((data) =>
      UpdateExperiment_Response.decode(new _m0.Reader(data))
    );
  }

  createRun(request: CreateRun): Promise<CreateRun_Response> {
    const data = CreateRun.encode(request).finish();
    const promise = this.rpc.request("mlflow.MlflowService", "createRun", data);
    return promise.then((data) =>
      CreateRun_Response.decode(new _m0.Reader(data))
    );
  }

  updateRun(request: UpdateRun): Promise<UpdateRun_Response> {
    const data = UpdateRun.encode(request).finish();
    const promise = this.rpc.request("mlflow.MlflowService", "updateRun", data);
    return promise.then((data) =>
      UpdateRun_Response.decode(new _m0.Reader(data))
    );
  }

  deleteRun(request: DeleteRun): Promise<DeleteRun_Response> {
    const data = DeleteRun.encode(request).finish();
    const promise = this.rpc.request("mlflow.MlflowService", "deleteRun", data);
    return promise.then((data) =>
      DeleteRun_Response.decode(new _m0.Reader(data))
    );
  }

  restoreRun(request: RestoreRun): Promise<RestoreRun_Response> {
    const data = RestoreRun.encode(request).finish();
    const promise = this.rpc.request(
      "mlflow.MlflowService",
      "restoreRun",
      data
    );
    return promise.then((data) =>
      RestoreRun_Response.decode(new _m0.Reader(data))
    );
  }

  logMetric(request: LogMetric): Promise<LogMetric_Response> {
    const data = LogMetric.encode(request).finish();
    const promise = this.rpc.request("mlflow.MlflowService", "logMetric", data);
    return promise.then((data) =>
      LogMetric_Response.decode(new _m0.Reader(data))
    );
  }

  logParam(request: LogParam): Promise<LogParam_Response> {
    const data = LogParam.encode(request).finish();
    const promise = this.rpc.request("mlflow.MlflowService", "logParam", data);
    return promise.then((data) =>
      LogParam_Response.decode(new _m0.Reader(data))
    );
  }

  setExperimentTag(
    request: SetExperimentTag
  ): Promise<SetExperimentTag_Response> {
    const data = SetExperimentTag.encode(request).finish();
    const promise = this.rpc.request(
      "mlflow.MlflowService",
      "setExperimentTag",
      data
    );
    return promise.then((data) =>
      SetExperimentTag_Response.decode(new _m0.Reader(data))
    );
  }

  setTag(request: SetTag): Promise<SetTag_Response> {
    const data = SetTag.encode(request).finish();
    const promise = this.rpc.request("mlflow.MlflowService", "setTag", data);
    return promise.then((data) => SetTag_Response.decode(new _m0.Reader(data)));
  }

  deleteTag(request: DeleteTag): Promise<DeleteTag_Response> {
    const data = DeleteTag.encode(request).finish();
    const promise = this.rpc.request("mlflow.MlflowService", "deleteTag", data);
    return promise.then((data) =>
      DeleteTag_Response.decode(new _m0.Reader(data))
    );
  }

  getRun(request: GetRun): Promise<GetRun_Response> {
    const data = GetRun.encode(request).finish();
    const promise = this.rpc.request("mlflow.MlflowService", "getRun", data);
    return promise.then((data) => GetRun_Response.decode(new _m0.Reader(data)));
  }

  searchRuns(request: SearchRuns): Promise<SearchRuns_Response> {
    const data = SearchRuns.encode(request).finish();
    const promise = this.rpc.request(
      "mlflow.MlflowService",
      "searchRuns",
      data
    );
    return promise.then((data) =>
      SearchRuns_Response.decode(new _m0.Reader(data))
    );
  }

  listArtifacts(request: ListArtifacts): Promise<ListArtifacts_Response> {
    const data = ListArtifacts.encode(request).finish();
    const promise = this.rpc.request(
      "mlflow.MlflowService",
      "listArtifacts",
      data
    );
    return promise.then((data) =>
      ListArtifacts_Response.decode(new _m0.Reader(data))
    );
  }

  getMetricHistory(
    request: GetMetricHistory
  ): Promise<GetMetricHistory_Response> {
    const data = GetMetricHistory.encode(request).finish();
    const promise = this.rpc.request(
      "mlflow.MlflowService",
      "getMetricHistory",
      data
    );
    return promise.then((data) =>
      GetMetricHistory_Response.decode(new _m0.Reader(data))
    );
  }

  logBatch(request: LogBatch): Promise<LogBatch_Response> {
    const data = LogBatch.encode(request).finish();
    const promise = this.rpc.request("mlflow.MlflowService", "logBatch", data);
    return promise.then((data) =>
      LogBatch_Response.decode(new _m0.Reader(data))
    );
  }

  logModel(request: LogModel): Promise<LogModel_Response> {
    const data = LogModel.encode(request).finish();
    const promise = this.rpc.request("mlflow.MlflowService", "logModel", data);
    return promise.then((data) =>
      LogModel_Response.decode(new _m0.Reader(data))
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
