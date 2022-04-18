import * as vscode from "vscode";
import { getModelRepositoryClient, geInferenceClient } from "../clients";
import { RepositoryIndexResponse_ModelIndex } from "../generated/inference/model_repository";
import * as R from "ramda";

let groupModels = R.groupBy(
  (model: RepositoryIndexResponse_ModelIndex) => model.name
);

type ModelTreeItem = ModelIndex | ModelVersion | ValueItem;

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
          element.models.map(
            (model) =>
              new ModelVersion(model, vscode.TreeItemCollapsibleState.Collapsed)
          )
        );
      }
      if (element instanceof ModelVersion) {
        return geInferenceClient()
          .modelMetadata({
            name: element.model.name,
            version: element.model.version,
          })
          .then(
            (meta) => {
              // let modelMap = groupModels(index.models);
              vscode.window.showErrorMessage(
                `Failed loading model index' - ${JSON.stringify(
                  meta.parameters
                )}`
              );
              let properties = [
                new ValueItem("Name", meta.name, "symbol-text"),
                new ValueItem("Version", meta.versions[0], "symbol-number"),
                new ValueItem("Status", element.model.state, "symbol-enum"),
              ];
              if ("current_stage" in meta.parameters) {
                properties.push(
                  new ValueItem(
                    "Lifecycle",
                    meta.parameters.current_stage.stringParam || "MISSING",
                    "symbol-enum"
                  )
                );
              }
              if ("mlflow_run_id" in meta.parameters) {
                properties.push(
                  new ValueItem(
                    "Run ID",
                    meta.parameters.mlflow_run_id.stringParam || "MISSING",
                    "symbol-key"
                  )
                );
              }
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
  constructor(
    public readonly model: RepositoryIndexResponse_ModelIndex,
    public readonly collapsibleState: vscode.TreeItemCollapsibleState
  ) {
    super(`Version: ${model.version}`, collapsibleState);

    this.id = `${model.name}-${model.version}`;
    this.description = `${model.state}`;
    this.iconPath = new vscode.ThemeIcon("git-commit");
  }

  contextValue = "mlfusion-model-version";
}

export class ModelSchema extends vscode.TreeItem {
  constructor(
    public readonly model: RepositoryIndexResponse_ModelIndex,
    public readonly collapsibleState: vscode.TreeItemCollapsibleState
  ) {
    super(`Version: ${model.version}`, collapsibleState);

    this.id = `${model.name}-${model.version}`;
    this.description = `${model.state}`;
    this.iconPath = new vscode.ThemeIcon("git-commit");
  }

  contextValue = "mlfusion-model-version";
}

export class ValueItem extends vscode.TreeItem {
  constructor(
    public readonly name: string,
    public readonly value: string,
    private icon: string = "symbol-constant",
  ) {
    super(name, vscode.TreeItemCollapsibleState.None);
    let col = vscode.ThemeColor;
    this.description = value;
    this.iconPath = new vscode.ThemeIcon(icon);
  }

  contextValue = "mlfusion-value";

  async copyValue(): Promise<void> {
    return vscode.env.clipboard.writeText(this.value);
  }
}
