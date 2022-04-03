# flake8: noqa 401
import subprocess  # nosec
import sys
from pathlib import Path

import typer
from loguru import logger

from ._utils import get_app_directory, get_app_settings

app = typer.Typer(name="server")


@app.command()
def start():
    logger.info("Starting flight-fusion server")

    settings = get_app_settings()

    try:
        import flight_fusion_server  # type: ignore
    except ImportError:
        logger.error("Install package 'flight-fusion-server' to tun server")
        typer.Abort()

    exec_path = Path(sys.executable).parent / "flight-fusion-server"
    subprocess.run([exec_path, "--log-level", "info"])  # nosec running our own executable


@app.command()
def stop():
    root = get_app_directory()
    typer.echo(f"{root}")


@app.command()
def install():
    subprocess.run(  # nosec cargo is a trusted application
        ["cargo", "install", "--git", "https://github.com/roeap/flight-fusion", "flight-fusion"]
    )
