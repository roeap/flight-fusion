import os
import subprocess  # nosec: just testing code
import time
from pathlib import Path
from typing import Generator

import pytest

from flight_fusion import ClientOptions, FusionServiceClient
from flight_fusion.cli._utils import _find_git_root

ENV_FF_EXECUTABLE = "FF_EXECUTABLE"


@pytest.fixture
def ff_server():
    # try and find a suitable executable
    ff_executable_path = os.environ.get(ENV_FF_EXECUTABLE)
    if ff_executable_path is None:
        git_root = _find_git_root()
        if git_root is None:
            raise ValueError("Executable path must be configured")
        ff_executable_path = git_root / "target/debug/flight-fusion"
    else:
        ff_executable_path = Path(ff_executable_path)

    if not ff_executable_path.exists():
        raise ValueError("Configured executable does not exist.")

    # make sure we have configuration
    # TODO
    # - if cwd is git root, don't bother
    # - else copy test config from git root in cwd

    ds_proc = subprocess.Popen(  # nosec: running only in tests
        str(ff_executable_path.absolute()),
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
    )

    # Give the server time to start
    time.sleep(1)

    # Check it started successfully
    assert not ds_proc.poll(), ds_proc.stdout.read().decode("utf-8")  # type: ignore

    yield ds_proc

    # Shut it down at the end of the pytest session
    # TODO
    # - delete config if we copied it
    # - clean up data / cache folder
    ds_proc.terminate()


@pytest.fixture
def ff_client(ff_server) -> Generator[FusionServiceClient, None, None]:
    options = ClientOptions(host="localhost", port=50051)
    client = FusionServiceClient(options)

    yield client
