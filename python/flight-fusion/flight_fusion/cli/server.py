# flake8: noqa 401
import subprocess  # nosec
import sys
from enum import Enum
from pathlib import Path

import typer
from loguru import logger

from ._utils import get_app_directory, get_app_settings

app = typer.Typer(name="server")


class LogLevel(str, Enum):
    info = "info"
    warn = "warn"
    error = "error"


@app.command()
def start(host: str = "127.0.0.1", port: int = 50051, log_level: LogLevel = LogLevel.info):
    """Run a local instance of the flight-fusion server"""
    logger.info("Starting flight-fusion server")

    settings = get_app_settings()

    exec_path = Path(sys.executable).parent / "flight-fusion-server"
    if not exec_path.exists():
        logger.error("Install package 'flight-fusion-server' to tun server")
        raise typer.Abort()

    subprocess.run(  # nosec running our own executable
        [exec_path, "--host", host, "--port", str(port), "--log-level", log_level.value]
    )


@app.command()
def stop():
    root = get_app_directory()
    typer.echo(f"{root}")
