# flake8: noqa 401
import shutil

import typer

from ._utils import (
    DAGSTER_DIR,
    FLIGHT_DIR,
    MLFLOW_DIR,
    get_app_directory,
    get_project_directory,
)

app = typer.Typer(name="env")


_GITIGNORE = """
*
!.gitignore
!workspace.yaml
"""


@app.command()
def init(local: bool = True):
    if local:
        work_dir = get_project_directory()
        if work_dir is None:
            raise ValueError
    else:
        raise NotImplementedError("Global project dir not yet implemented")

    if not work_dir.exists():
        work_dir.mkdir(parents=True)

    mlflow_dir = work_dir / MLFLOW_DIR
    if not mlflow_dir.exists():
        mlflow_dir.mkdir(parents=False)
    if not (mlflow_dir / "mlruns").exists():
        (mlflow_dir / "mlruns").mkdir(parents=False)

    dagster_dir = work_dir / DAGSTER_DIR
    if not dagster_dir.exists():
        dagster_dir.mkdir(parents=False)

    dagster_dir = work_dir / FLIGHT_DIR
    if not dagster_dir.exists():
        dagster_dir.mkdir(parents=False)

    ignore_file = work_dir / ".gitignore"
    if not ignore_file.exists():
        with ignore_file.open("w", encoding="utf-8") as f_:
            f_.write(_GITIGNORE)

    typer.echo(f"Initialized flight-fusion env in: {str(work_dir)}")


@app.command()
def clear():
    work_dir = get_project_directory()
    if work_dir is not None and work_dir.exists():
        shutil.rmtree(str(work_dir.absolute()))
