import subprocess  # nosec

import typer

from .charts import MlfusionHelmValues


def git_repo_root():
    return subprocess.check_output(["git", "rev-parse", "--show-toplevel"]).decode("utf-8").strip()  # nosec


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


app = typer.Typer(name="mlfusion-helm")
app.add_typer(schema_app)
