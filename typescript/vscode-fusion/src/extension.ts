import * as vscode from "vscode";
import { ModelIndexProvider, ModelIndex } from "./trees";

// this method is called when your extension is activated
// your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {
  // Use the console to output diagnostic information (console.log) and errors (console.error)
  // This line of code will only be executed once when your extension is activated
  console.log('Congratulations, your extension "vscode-fusion" is now active!');

  const modelIndexProvider = new ModelIndexProvider();
  context.subscriptions.push(
    vscode.window.registerTreeDataProvider(
      "mlfusion-models",
      modelIndexProvider
    )
  );
  context.subscriptions.push(
    vscode.commands.registerCommand("vscode-fusion.models.refreshList", () =>
      modelIndexProvider.refresh()
    )
  );
  context.subscriptions.push(
    vscode.commands.registerCommand(
      "vscode-fusion.models.loadModel",
      (model: ModelIndex) => modelIndexProvider.loadModel(model)
    )
  );
  context.subscriptions.push(
    vscode.commands.registerCommand(
      "vscode-fusion.models.unloadModel",
      (model: ModelIndex) => modelIndexProvider.unloadModel(model)
    )
  );
}

// this method is called when your extension is deactivated
export function deactivate() {}
