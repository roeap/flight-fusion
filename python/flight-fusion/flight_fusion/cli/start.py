# flake8: noqa 401
import asyncio
import os
import subprocess  # nosec
import sys
from enum import Enum
from functools import wraps
from pathlib import Path

import typer
from loguru import logger

from ._utils import DAGSTER_DIR, MLFLOW_DIR, MLSERVER_DIR, get_app_directory

app = typer.Typer(name="start")


def click_async(f):
    @wraps(f)
    def wrapper(*args, **kwargs):
        return asyncio.run(f(*args, **kwargs))

    return wrapper


class LogLevel(str, Enum):
    info = "info"
    warn = "warn"
    error = "error"


@app.command()
def flight(host: str = "127.0.0.1", port: int = 50051, log_level: LogLevel = LogLevel.info):
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

    subprocess.run(  # nosec running our own executable
        [
            "mlflow",
            "ui",
            "--host",
            "127.0.0.1",
            "--port",
            str(port),
            "--backend-store-uri",
            "sqlite:///mlruns.sqlite",
            "--default-artifact-root",
            f"fusion://",
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
@click_async
async def mlserver(host: str = "http://127.0.0.1", port: int = 5000):
    from mlserver.cli.serve import load_settings

    from mlserver_fusion.server import MLServer

    workdir = get_app_directory().absolute() / MLSERVER_DIR
    settings, models_settings = await load_settings(str(workdir))

    server = MLServer(settings, f"{host}:{port}")
    await server.start(models_settings)
