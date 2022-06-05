from pathlib import Path

from flight_fusion import ClientOptions, FusionServiceClient

from mlflow_fusion.client import ArtifactRepoOptions, MlflowArtifactsClient
from mlflow_fusion.ipc.artifacts import UploadArtifact


def test_artifact_roundtrip(fusion_client: tuple[FusionServiceClient, ClientOptions], datadir: Path):
    opts = ArtifactRepoOptions()
    client = MlflowArtifactsClient(opts)
    path = "foo/bar.baz"
    data = b"arbitrary data"

    req_iter = [UploadArtifact(path=str(path), data=data)]
    client.upload_artifact(request_iterator=req_iter)

    upload_path = datadir / ".mlflow/mlruns" / "foo/bar.baz"
    assert upload_path.exists()

    response = client.download_artifact(path=str(path))
    assert response == data


def test_list_artifacts(fusion_client: tuple[FusionServiceClient, ClientOptions]):
    opts = ArtifactRepoOptions()
    client = MlflowArtifactsClient(opts)
    data = b"arbitrary data"

    paths = ["foo/bar.baz", "foo/asd.baz"]
    for path in paths:
        req_iter = [UploadArtifact(path=str(path), data=data)]
        client.upload_artifact(request_iterator=req_iter)

    artifacts = client.list_artifacts(path=None)
    assert len(artifacts) == 1
    assert artifacts[0].is_dir

    artifacts = client.list_artifacts(path="foo")
    assert len(artifacts) == 2
    assert all([not f.is_dir for f in artifacts])
