/* eslint-disable @typescript-eslint/naming-convention */
import * as vscode from "vscode";
import * as R from "ramda";
import { getModelRepositoryClient, getInferenceClient } from "../clients";
import { RepositoryIndexResponse_ModelIndex } from "../generated/inference/model_repository";
import {
  ModelMetadataResponse_TensorMetadata,
  ModelMetadataResponse,
} from "../generated/inference/dataplane";

let groupModels = R.groupBy(
  (model: RepositoryIndexResponse_ModelIndex) => model.name
);

type ModelTreeItem = ModelIndex | ModelVersion | ModelSchema | ValueItem;

let typeMap: { [key: string]: string } = {
  BOOL: "bool",
  UINT8: "unsigned int8",
  UINT16: "unsigned int16",
  UINT32: "unsigned int32",
  UINT64: "unsigned int64",
  INT8: "int8",
  INT16: "int16",
  INT32: "int32",
  INT64: "int64",
  FP16: "float16",
  FP32: "float32",
  FP64: "float64",
  BYTES: "bytes",
};

export class ModelIndexProvider
  implements vscode.TreeDataProvider<ModelTreeItem>
{
  private _onDidChangeTreeData: vscode.EventEmitter<
    ModelTreeItem | undefined | void
  > = new vscode.EventEmitter<ModelTreeItem | undefined | void>();
  readonly onDidChangeTreeData: vscode.Event<ModelTreeItem | undefined | void> =
    this._onDidChangeTreeData.event;

  constructor() {}

  refresh(): void {
    this._onDidChangeTreeData.fire();
  }

  getTreeItem(element: ModelTreeItem): vscode.TreeItem {
    return element;
  }

  async getChildren(element?: ModelTreeItem): Promise<ModelTreeItem[]> {
    if (element) {
      if (element instanceof ModelIndex) {
        return Promise.resolve(
          element.models.map((model) => new ModelVersion(model))
        );
      }
      if (element instanceof ModelVersion) {
        return getInferenceClient()
          .modelMetadata({
            name: element.model.name,
            version: element.model.version,
          })
          .then(
            (meta: ModelMetadataResponse) => {
              let properties: (ModelSchema | ValueItem)[] = [];
              if ("current_stage" in meta.parameters) {
                let value =
                  meta.parameters.current_stage.stringParam || "MISSING";
                properties.push(
                  new ValueItem("Lifecycle", value, "symbol-enum")
                );
              }
              if ("experiment_id" in meta.parameters) {
                let id = meta.parameters.experiment_id.int64Param || -1;
                properties.push(
                  new ValueItem("Experiment ID", String(id), "symbol-number")
                );
              }
              if ("mlflow_run_id" in meta.parameters) {
                let value =
                  meta.parameters.mlflow_run_id.stringParam || "MISSING";
                properties.push(new ValueItem("Run ID", value, "symbol-key"));
              }
              if ("created" in meta.parameters) {
                let date = new Date(meta.parameters.created.int64Param || 0);
                properties.push(
                  new ValueItem("Created", date.toISOString(), "calendar")
                );
              }
              if ("last_updated" in meta.parameters) {
                let date = new Date(
                  meta.parameters.last_updated.int64Param || 0
                );
                properties.push(
                  new ValueItem("Last updated", date.toISOString(), "calendar")
                );
              }
              properties.push(new ModelSchema("Inputs", element, meta.inputs));
              properties.push(
                new ModelSchema("Outputs", element, meta.outputs)
              );
              return properties;
            },
            (err) => {
              vscode.window.showErrorMessage(
                `Failed loading metadata' - ${err}`
              );
              return [];
            }
          );
      }
      if (element instanceof ModelSchema) {
        return Promise.resolve(element.columns());
      }
      return Promise.resolve([]);
    } else {
      return this.getRegisteredModels();
    }
  }

  async getRegisteredModels(): Promise<ModelIndex[]> {
    return getModelRepositoryClient()
      .repositoryIndex({})
      .then(
        (index) => {
          let modelMap = groupModels(index.models);
          return Object.entries(modelMap).map(
            ([name, models]) =>
              new ModelIndex(
                name,
                models,
                vscode.TreeItemCollapsibleState.Collapsed
              )
          );
        },
        (err) => {
          vscode.window.showErrorMessage(
            `Failed loading model index' - ${err}`
          );
          return [];
        }
      );
  }

  async loadModel(element: ModelIndex): Promise<string | undefined> {
    const grpcClient = getModelRepositoryClient();
    return grpcClient.repositoryModelLoad({ modelName: element.name }).then(
      (_response) => {
        this.refresh();
        return vscode.window.showInformationMessage(
          `Successfully loaded model: '${element.name}'`
        );
      },
      (err) =>
        vscode.window.showErrorMessage(
          `Failed loading model: '${element.name}' - ${err}`
        )
    );
  }

  async unloadModel(element: ModelIndex): Promise<string | undefined> {
    const grpcClient = getModelRepositoryClient();
    return grpcClient.repositoryModelUnload({ modelName: element.name }).then(
      (_response) => {
        this.refresh();
        return vscode.window.showInformationMessage(
          `Successfully unloaded model: '${element.name}'`
        );
      },
      (err) =>
        vscode.window.showErrorMessage(
          `Failed unloading model: '${element.name}' - ${err}`
        )
    );
  }
}

export class ModelIndex extends vscode.TreeItem {
  constructor(
    public readonly name: string,
    public readonly models: RepositoryIndexResponse_ModelIndex[],
    public readonly collapsibleState: vscode.TreeItemCollapsibleState
  ) {
    super(name, collapsibleState);

    this.id = name;
    this.description = `(${models.length}) - ${models[0].state}`;
    this.iconPath = new vscode.ThemeIcon("repo");
  }

  contextValue = "mlfusion-model";
}

export class ModelVersion extends vscode.TreeItem {
  constructor(public readonly model: RepositoryIndexResponse_ModelIndex) {
    super(
      `Version: ${model.version}`,
      model.state === "READY"
        ? vscode.TreeItemCollapsibleState.Collapsed
        : vscode.TreeItemCollapsibleState.None
    );

    this.id = `${model.name}-${model.version}`;
    this.description = `${model.state}`;
    this.iconPath = new vscode.ThemeIcon("git-commit");
  }

  contextValue = "mlfusion-model-version";
}

export class ModelSchema extends vscode.TreeItem {
  constructor(
    public readonly label: string,
    public readonly model: ModelVersion,
    public readonly tensors: ModelMetadataResponse_TensorMetadata[]
  ) {
    super(label, vscode.TreeItemCollapsibleState.Collapsed);

    this.id = `${model.model.name}-${model.model.version}-${label}`;
    this.description = `(${tensors.length})`;
    this.iconPath = new vscode.ThemeIcon("symbol-class");
  }

  contextValue = "mlfusion-model-schema";

  columns() {
    return this.tensors.map(
      (tensor) => new ValueItem(tensor.name, typeMap[tensor.datatype], "array")
    );
  }
}

export class ValueItem extends vscode.TreeItem {
  constructor(
    public readonly name: string,
    public readonly value: string,
    private icon: string = "symbol-constant"
  ) {
    super(name, vscode.TreeItemCollapsibleState.None);

    this.description = value;
    this.iconPath = new vscode.ThemeIcon(icon);
  }

  contextValue = "mlfusion-value";

  async copyValue(): Promise<void> {
    return vscode.env.clipboard.writeText(this.value);
  }
}
