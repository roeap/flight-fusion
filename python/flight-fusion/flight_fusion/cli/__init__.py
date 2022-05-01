import sys

import typer
from loguru import logger

from .environment import app as env_app
from .server import app as server_app

_LOGGER_FORMAT = (
    "<green>{time:YYYY-MM-DD HH:mm:ss.SSS}</green> | "
    "<level>{level: <8}</level> | "
    "<level>{message}</level>"
)

logger.remove()  # All configured handlers are removed
logger.add(sys.stderr, format=_LOGGER_FORMAT)

app = typer.Typer(name="flight-fusion")
app.add_typer(server_app)
app.add_typer(env_app)
