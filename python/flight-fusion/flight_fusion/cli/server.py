import subprocess  # nosec

import typer
from loguru import logger

from ._utils import get_app_directory, get_app_settings

app = typer.Typer(name="server")


@app.command()
def start():
    logger.info("Starting flight-fusion server")

    settings = get_app_settings()

    subprocess.run(str(settings.executable))  # nosec running our own executable


@app.command()
def stop():
    root = get_app_directory()
    typer.echo(f"{root}")
