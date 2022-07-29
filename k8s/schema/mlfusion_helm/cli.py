import subprocess  # nosec
from pathlib import Path

import typer

from .charts import MlflowHelmValues, MlfusionHelmValues


def git_repo_root() -> Path:
    return Path(subprocess.check_output(["git", "rev-parse", "--show-toplevel"]).decode("utf-8").strip())  # nosec


CLI_HELP = "Tools to help generate the schema file for the MlFusion Helm chart."

schema_app = typer.Typer(name="schema")


@schema_app.command()
def schema():
    """Generates the `values.schema.json` file according to user specified pydantic models."""


@schema_app.command()
def show():
    """
    Displays the json schema on the console.
    """

    typer.echo("--- MLFusion Helm Values ---")
    typer.echo(MlfusionHelmValues.schema_json(indent=4))


@schema_app.command()
def apply():
    """
    Saves the json schema in the Helm `values.schema.json`.
    """
    helm_values_path_tuples = {
        (MlfusionHelmValues, git_repo_root() / "k8s" / "mlfusion" / "values.schema.json"),
        (MlflowHelmValues, git_repo_root() / "k8s" / "mlflow" / "values.schema.json"),
    }

    for helm_values, path in helm_values_path_tuples:
        with path.open("w", encoding="utf8") as f:
            f.write(helm_values.schema_json(indent=4))
            f.write("\n")


app = typer.Typer(name="mlfusion-helm")
app.add_typer(schema_app)
