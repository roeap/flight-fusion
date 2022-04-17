import * as vscode from "vscode";
import { getModelRepositoryClient } from "../clients";
import { RepositoryIndexResponse_ModelIndex } from "../generated/inference/model_repository";

export class ModelIndexProvider implements vscode.TreeDataProvider<ModelIndex> {
  private _onDidChangeTreeData: vscode.EventEmitter<
    ModelIndex | undefined | void
  > = new vscode.EventEmitter<ModelIndex | undefined | void>();
  readonly onDidChangeTreeData: vscode.Event<ModelIndex | undefined | void> =
    this._onDidChangeTreeData.event;

  constructor() {}

  refresh(): void {
    this._onDidChangeTreeData.fire();
  }

  getTreeItem(element: ModelIndex): vscode.TreeItem {
    return element;
  }

  async getChildren(element?: ModelIndex): Promise<ModelIndex[]> {
    const grpcClient = getModelRepositoryClient();

    if (element) {
      return Promise.resolve([]);
    } else {
      return grpcClient
        .repositoryIndex({})
        .then((index) =>
          index.models.map(
            (model) =>
              new ModelIndex(model, vscode.TreeItemCollapsibleState.None)
          )
        );
    }
  }

  async loadModel(element: ModelIndex): Promise<string | undefined> {
    const grpcClient = getModelRepositoryClient();
    return grpcClient
      .repositoryModelLoad({ modelName: element.index.name })
      .then((_response) =>
        vscode.window.showInformationMessage(
          `Successfully loaded model: '${element.index.name}'`
        )
      );
  }
}

export class ModelIndex extends vscode.TreeItem {
  constructor(
    public readonly index: RepositoryIndexResponse_ModelIndex,
    public readonly collapsibleState: vscode.TreeItemCollapsibleState
  ) {
    super(`${index.name}`, collapsibleState);

    this.id = `${index.name}-${index.version}`;
    this.description = `(${index.version}) - ${index.state}`;
    this.iconPath = new vscode.ThemeIcon("notebook-kernel-configure");
  }

  contextValue = "mlfusion-model";
}
