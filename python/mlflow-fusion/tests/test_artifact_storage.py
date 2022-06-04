from pathlib import Path

from flight_fusion import FusionServiceClient

from mlflow_fusion.client import MlflowArtifactsClient
from mlflow_fusion.ipc.artifacts import UploadArtifact


def test_artifact_roundtrip(fusion_client: FusionServiceClient, datadir: Path):
    client = MlflowArtifactsClient(use_ssl=False)
    path = "foo/bar.baz"
    data = b"arbitrary data"

    req_iter = [UploadArtifact(path=str(path), data=data)]
    client.upload_artifact(request_iterator=req_iter)

    upload_path = datadir / ".mlflow/mlruns" / "foo/bar.baz"
    assert upload_path.exists()
