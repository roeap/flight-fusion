import subprocess  # nosec
from pathlib import Path
from typing import Dict, Optional

import toml
import yaml
from typer import get_app_dir

from flight_fusion.errors import FlightFusionError

_APP_NAME = "flight-fusion"
_CONFIG_FILE_STEM = "app"

MLFLOW_DIR = ".mlflow"
DAGSTER_DIR = ".dagster"


def find_git_root() -> Optional[Path]:
    try:
        args = ["git", "rev-parse", "--show-toplevel"]
        output = subprocess.check_output(args)  # nosec
    except subprocess.CalledProcessError:
        return None

    return Path(output.strip(b"\n").decode())


def _read_config_data(app_root: Path) -> Dict:
    if not app_root.is_dir():
        print(app_root)
        raise FlightFusionError("App root must be directory")

    for suffix in ["yml", "yaml", "json"]:
        path = app_root / f"{_CONFIG_FILE_STEM}.{suffix}"
        if path.exists():
            with path.open(encoding="utf-8") as f_:
                return yaml.safe_load(f_)

    for suffix in ["toml"]:
        path = app_root / f"{_CONFIG_FILE_STEM}.{suffix}"
        if path.exists():
            with path.open(encoding="utf-8") as f_:
                return toml.load(f_)  # type: ignore

    raise FlightFusionError("Unsupported file format for config file")


def get_project_directory():
    git_root = find_git_root()
    if git_root is not None:
        return git_root / f".{_APP_NAME}"
    return None


def get_global_directory():
    return Path(get_app_dir(app_name=_APP_NAME, force_posix=True, roaming=False))


def get_app_directory() -> Path:
    """Get path to the application config directory

    This function tries to find the most appropriate location for app configuration.

    1. It checks if the app is running inside a git repository. If so it looks in the git root.
    2. It traverses up the directory tree to see if it finds a config folder.
    3. It checks in the os-specific user directory for global config.

    Returns:
        Path: path to application config directory
    """
    directory = get_project_directory()
    if directory is not None and directory.exists():
        return directory

    raise NotImplementedError("Global dir not supported yet.")
    return get_global_directory()
