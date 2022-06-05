import os
import posixpath
from pathlib import Path

import pytest
from mlflow.exceptions import MlflowException
from mlflow.utils.file_utils import TempDir

from mlflow_fusion.artifacts import FusionArtifactRepository


@pytest.fixture
def artifact_root(datadir: Path):
    return str(datadir / ".mlflow/mlruns")


@pytest.fixture
def artifact_repo(artifact_root: str, fusion_client) -> FusionArtifactRepository:
    return FusionArtifactRepository(artifact_uri="")


def test_list_artifacts(artifact_repo: FusionArtifactRepository, artifact_root: str):
    assert len(artifact_repo.list_artifacts()) == 0

    artifact_rel_path = "artifact"
    artifact_path = os.path.join(artifact_root, artifact_rel_path)
    with open(artifact_path, "w") as f:
        f.write("artifact")
    artifacts_list = artifact_repo.list_artifacts()
    assert len(artifacts_list) == 1
    assert artifacts_list[0].path == artifact_rel_path


def test_log_artifacts(artifact_repo: FusionArtifactRepository, artifact_root: str):
    artifact_rel_path = "test.txt"
    artifact_text = "hello world!"
    with TempDir() as src_dir:
        artifact_src_path = src_dir.path(artifact_rel_path)
        with open(artifact_src_path, "w") as f:
            f.write(artifact_text)
        artifact_repo.log_artifact(artifact_src_path)

    artifacts_list = artifact_repo.list_artifacts()
    assert len(artifacts_list) == 1
    assert artifacts_list[0].path == artifact_rel_path

    artifact_dst_path = os.path.join(artifact_root, artifact_rel_path)
    assert os.path.exists(artifact_dst_path)
    assert artifact_dst_path != artifact_src_path
    assert open(artifact_dst_path).read() == artifact_text


@pytest.mark.parametrize("dst_path", [None, "dest"])
def test_download_artifacts(artifact_repo: FusionArtifactRepository, dst_path):
    artifact_rel_path = "test.txt"
    artifact_text = "hello world!"
    empty_dir_path = "empty_dir"
    with TempDir(chdr=True) as local_dir:
        if dst_path:
            os.mkdir(dst_path)
        artifact_src_path = local_dir.path(artifact_rel_path)
        os.mkdir(local_dir.path(empty_dir_path))
        with open(artifact_src_path, "w") as f:
            f.write(artifact_text)
        artifact_repo.log_artifacts(local_dir.path())
        result = artifact_repo.download_artifacts(artifact_path=artifact_rel_path, dst_path=dst_path)
        assert open(result).read() == artifact_text
        result = artifact_repo.download_artifacts(artifact_path="", dst_path=dst_path)
        # TODO do we need to create empty local repos when downloading?
        # empty_dir_dst_path = os.path.join(result, empty_dir_path)
        # assert os.path.isdir(empty_dir_dst_path)
        # assert len(os.listdir(empty_dir_dst_path)) == 0


def test_download_artifacts_does_not_copy(artifact_repo: FusionArtifactRepository):
    """
    The LocalArtifactRepository.download_artifact function should not copy the artifact if
    the ``dst_path`` argument is None.
    """
    artifact_rel_path = "test.txt"
    artifact_text = "hello world!"
    with TempDir(chdr=True) as local_dir:
        artifact_src_path = local_dir.path(artifact_rel_path)
        with open(artifact_src_path, "w") as f:
            f.write(artifact_text)
        artifact_repo.log_artifact(artifact_src_path)
        dst_path = artifact_repo.download_artifacts(artifact_path=artifact_rel_path)
        assert open(dst_path).read() == artifact_text
        # assert dst_path.startswith(
        #      artifact_repo.artifact_dir
        # ), "downloaded artifact is not in local_artifact_repo.artifact_dir root"


def test_download_artifacts_returns_absolute_paths(artifact_repo: FusionArtifactRepository):
    artifact_rel_path = "test.txt"
    artifact_text = "hello world!"
    with TempDir(chdr=True) as local_dir:
        artifact_src_path = local_dir.path(artifact_rel_path)
        with open(artifact_src_path, "w") as f:
            f.write(artifact_text)
        artifact_repo.log_artifact(artifact_src_path)

        for dst_dir in ["dst1", local_dir.path("dst2"), None]:
            if dst_dir is not None:
                os.makedirs(dst_dir)
            dst_path = artifact_repo.download_artifacts(artifact_path=artifact_rel_path, dst_path=dst_dir)
            if dst_dir is not None:
                # If dst_dir isn't none, assert we're actually downloading to dst_dir.
                assert dst_path.startswith(os.path.abspath(dst_dir))
            assert dst_path == os.path.abspath(dst_path)


@pytest.mark.parametrize(
    "repo_subdir_path",
    [
        "aaa",
        "aaa/bbb",
        "aaa/bbb/ccc/ddd",
    ],
)
def test_artifacts_are_logged_to_and_downloaded_from_repo_subdirectory_successfully(
    artifact_repo: FusionArtifactRepository, repo_subdir_path: str
):
    artifact_rel_path = "test.txt"
    artifact_text = "hello world!"
    with TempDir(chdr=True) as local_dir:
        artifact_src_path = local_dir.path(artifact_rel_path)
        with open(artifact_src_path, "w") as f:
            f.write(artifact_text)
        artifact_repo.log_artifact(artifact_src_path, artifact_path=repo_subdir_path)

    downloaded_subdir = artifact_repo.download_artifacts(repo_subdir_path)
    assert os.path.isdir(downloaded_subdir)
    subdir_contents = os.listdir(downloaded_subdir)
    assert len(subdir_contents) == 1
    assert artifact_rel_path in subdir_contents
    assert open(os.path.join(downloaded_subdir, artifact_rel_path)).read() == artifact_text

    downloaded_file = artifact_repo.download_artifacts(posixpath.join(repo_subdir_path, artifact_rel_path))
    assert open(downloaded_file).read() == artifact_text


def test_log_artifact_throws_exception_for_invalid_artifact_paths(artifact_repo: FusionArtifactRepository):
    with TempDir() as local_dir:
        for bad_artifact_path in ["/", "//", "/tmp", "/bad_path", ".", "../terrible_path"]:
            with pytest.raises(MlflowException) as exc_info:
                artifact_repo.log_artifact(local_dir.path(), bad_artifact_path)
            assert "Invalid artifact path" in str(exc_info)


def test_logging_directory_of_artifacts_produces_expected_repo_contents(artifact_repo: FusionArtifactRepository):
    with TempDir() as local_dir:
        os.mkdir(local_dir.path("subdir"))
        os.mkdir(local_dir.path("subdir", "nested"))
        with open(local_dir.path("subdir", "a.txt"), "w") as f:
            f.write("A")
        with open(local_dir.path("subdir", "b.txt"), "w") as f:
            f.write("B")
        with open(local_dir.path("subdir", "nested", "c.txt"), "w") as f:
            f.write("C")
        artifact_repo.log_artifacts(local_dir.path("subdir"))
        assert open(artifact_repo.download_artifacts("a.txt")).read() == "A"
        assert open(artifact_repo.download_artifacts("b.txt")).read() == "B"
        assert open(artifact_repo.download_artifacts("nested/c.txt")).read() == "C"


def test_hidden_files_are_logged_correctly(artifact_repo: FusionArtifactRepository):
    with TempDir() as local_dir:
        hidden_file = local_dir.path(".mystery")
        with open(hidden_file, "w") as f:
            f.write("42")
        artifact_repo.log_artifact(hidden_file)
        assert open(artifact_repo.download_artifacts(".mystery")).read() == "42"
