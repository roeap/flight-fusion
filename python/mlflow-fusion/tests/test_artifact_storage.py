from pathlib import Path

from mlflow_fusion.client import MlflowArtifactsClient
from mlflow_fusion.ipc.artifacts import UploadArtifact

from flight_fusion import ClientOptions, FusionServiceClient


def test_artifact_roundtrip(fusion_client_with_options: tuple[FusionServiceClient, ClientOptions], datadir: Path):
    _, options = fusion_client_with_options
    client = MlflowArtifactsClient(host=options.host, port=options.port, use_ssl=False)
    path = "foo/bar.baz"
    data = b"arbitrary data"

    req_iter = [UploadArtifact(path=str(path), data=data)]
    client.upload_artifact(request_iterator=req_iter)

    upload_path = datadir / ".mlflow/mlruns" / "foo/bar.baz"
    assert upload_path.exists()

    responses = list(client.download_artifact(path=str(path)))
    assert responses[0].data == data
