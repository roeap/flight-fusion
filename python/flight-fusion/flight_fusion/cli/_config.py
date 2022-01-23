from pathlib import Path

from pydantic import BaseSettings


class AppSettings(BaseSettings):
    executable: Path

    class Config:
        env_nested_delimiter = "__"
        env_prefix = "FF_"
