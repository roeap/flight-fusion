# flake8: noqa 401
import os
import subprocess  # nosec
import sys
from enum import Enum
from pathlib import Path

import typer
from loguru import logger

from ._utils import DAGSTER_DIR, MLFLOW_DIR, get_app_directory, get_project_directory

app = typer.Typer(name="server")


class LogLevel(str, Enum):
    info = "info"
    warn = "warn"
    error = "error"


@app.command()
def start(host: str = "127.0.0.1", port: int = 50051, log_level: LogLevel = LogLevel.info):
    """Run a local instance of the flight-fusion server"""
    logger.info("Starting flight-fusion server")

    exec_path = Path(sys.executable).parent / "flight-fusion-server"
    if not exec_path.exists():
        logger.error("Install package 'flight-fusion-server' to tun server")
        raise typer.Abort()

    workdir = get_app_directory()
    subprocess.run(  # nosec running our own executable
        [exec_path, "--host", "127.0.0.1", "--port", str(port), "--log-level", log_level.value],
        cwd=str(workdir),
    )


@app.command()
def mlflow(port: int = 5000):
    """Run a local instance of the mlflow server"""
    logger.info(f"Serving mlflow on 127.0.0.1:{port}")

    workdir = (get_app_directory() / MLFLOW_DIR).absolute()
    artifact_root = workdir / "mlruns/"

    subprocess.run(  # nosec running our own executable
        [
            "mlflow",
            "server",
            "--host",
            "127.0.0.1",
            "--port",
            str(port),
            "--backend-store-uri",
            "sqlite:///mlruns.sqlite",
            "--default-artifact-root",
            str(artifact_root),
        ],
        cwd=str(workdir),
    )


@app.command()
def dagit(port: int = 3000):
    """Run a local instance of dagit server"""
    logger.info(f"Serving dagit on 127.0.0.1:{port}")

    workdir = get_app_directory().absolute()
    home = workdir / DAGSTER_DIR
    ws_file = workdir / "workspace.yaml"

    subprocess.run(  # nosec running our own executable
        ["dagit", "--workspace", str(ws_file), "--port", str(port)],
        cwd=str(workdir),
        env={**os.environ, "DAGSTER_HOME": str(home)},
    )


@app.command()
def daemon():
    """Run a local instance of the dagit server"""
    logger.info("Starting dagit server")

    workdir = get_app_directory().absolute()
    home = workdir / DAGSTER_DIR
    ws_file = workdir / "workspace.yaml"

    subprocess.run(  # nosec running our own executable
        ["dagster-daemon", "run", "--workspace", str(ws_file)],
        cwd=str(workdir),
        env={**os.environ, "DAGSTER_HOME": str(home)},
    )


@app.command()
def stop():
    root = get_app_directory()
    typer.echo(f"{root}")
