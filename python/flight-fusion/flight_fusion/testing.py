import socket
import subprocess  # nosec: just testing code
import sys
import time
from pathlib import Path
from typing import Generator

from flight_fusion import ClientOptions, FusionServiceClient

try:
    import pytest
except ImportError as err:
    print(
        "\033[31m"
        f"Unable to import `{err.name}` from flight_fusion plugin! "
        "Please ensure that you've installed flight_fusion as "
        '`pip install "flight_fusion[dev]"` so that dev dependencies '
        "are included."
        "\033[0m"
    )
    raise SystemExit(1)


@pytest.fixture
def fusion_client(datadir: Path) -> Generator[FusionServiceClient, None, None]:

    exec_path = Path(sys.executable).parent / "flight-fusion-server"

    if not exec_path.exists():
        raise ImportError(
            "flight-fusion-server not installed."
            "It needs to be installed separately via `pip install flight-fusion-server`"
        )

    # lets just hope we find a free port by getting assigned one,
    # and then closing the socket to free up the port.
    sock = socket.socket()
    sock.bind(("", 0))
    port = sock.getsockname()[1]
    sock.close()

    (datadir / ".fusion").mkdir(exist_ok=True)
    (datadir / ".mlflow/mlruns").mkdir(exist_ok=True, parents=True)

    ds_proc = subprocess.Popen(  # nosec: running only in tests
        [str(exec_path.absolute()), "--host", "127.0.0.1", "--port", str(port)],
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        cwd=str(datadir),
    )

    # Give the server time to start
    time.sleep(1)

    # Check it started successfully
    assert not ds_proc.poll(), ds_proc.stdout.read().decode("utf-8")  # type: ignore  # nosec

    options = ClientOptions(host="localhost", port=port)
    client = FusionServiceClient(options)

    yield client

    # Shut down server at the end of the pytest session
    ds_proc.terminate()
