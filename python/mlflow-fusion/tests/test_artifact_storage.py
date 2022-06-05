from pathlib import Path

from flight_fusion import ClientOptions, FusionServiceClient

from mlflow_fusion.client import MlflowArtifactsClient
from mlflow_fusion.ipc.artifacts import UploadArtifact


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


def test_list_artifacts(fusion_client_with_options: tuple[FusionServiceClient, ClientOptions]):
    _, options = fusion_client_with_options
    client = MlflowArtifactsClient(host=options.host, port=options.port, use_ssl=False)
    data = b"arbitrary data"

    paths = ["foo/bar.baz", "foo/asd.baz"]
    for path in paths:
        req_iter = [UploadArtifact(path=str(path), data=data)]
        client.upload_artifact(request_iterator=req_iter)

    artifacts = client.list_artifacts(path=None)
    assert len(artifacts.files) == 1
    assert artifacts.files[0].is_dir

    artifacts = client.list_artifacts(path="foo")
    assert len(artifacts.files) == 2
    assert all([not f.is_dir for f in artifacts.files])
