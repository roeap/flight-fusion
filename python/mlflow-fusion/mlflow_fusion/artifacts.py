from __future__ import annotations

from os import walk
from pathlib import Path

from mlflow.entities import FileInfo
from mlflow.store.artifact.artifact_repo import ArtifactRepository

from mlflow_fusion.client import MlflowArtifactsClient
from mlflow_fusion.ipc.artifacts import UploadArtifact


class FusionArtifactRepository(ArtifactRepository):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self._repo_root = Path(self.artifact_uri.strip("fusion:").strip("/"))
        self._client = MlflowArtifactsClient()

    def _get_object_key(self, artifact_path: str | None) -> str:
        return str(self._repo_root / (artifact_path or ""))

    def log_artifact(self, local_file: str, artifact_path=None) -> None:
        """
        Log a local file as an artifact, optionally taking an ``artifact_path`` to place it in
        within the run's artifacts. Run artifacts can be organized into directories, so you can
        place the artifact in a directory this way.

        :param local_file: Path to artifact to log
        :param artifact_path: Directory within the run's artifact directory in which to log the artifact.
        """
        # TODO handle files larger then single request limit
        with Path(local_file).open("rb") as file:
            data = file.read()
        key = self._get_object_key(artifact_path)
        self._client.upload_artifact([UploadArtifact(path=key, data=data)])

    def log_artifacts(self, local_dir: str, artifact_path=None) -> None:
        """
        Log the files in the specified local directory as artifacts, optionally taking
        an ``artifact_path`` to place them in within the run's artifacts.

        :param local_dir: Directory of local artifacts to log
        :param artifact_path: Directory within the run's artifact directory in which to log the artifacts
        """
        for (root, _, files) in walk(local_dir):
            for file in files:
                self.log_artifact(local_file=str(Path(root) / file), artifact_path=f"{artifact_path}/{file}")

    def list_artifacts(self, path=None) -> list[FileInfo]:
        """
        Return all the artifacts for this run_id directly under path. If path is a file, returns
        an empty list. Will error if path is neither a file nor directory.

        :param path: Relative source path that contains desired artifacts

        :return: List of artifacts as FileInfo listed directly under path.
        """
        return self._client.list_artifacts(path=self._get_object_key(path))

    def _download_file(self, remote_file_path: str, local_path: str) -> None:
        """
        Download the file at the specified relative remote path and saves it at the specified local path.

        :param remote_file_path: Source path to the remote file, relative to the root directory of the artifact repository.
        :param local_path: The path to which to save the downloaded file.
        """
        data = self._client.download_artifact(path=self._get_object_key(remote_file_path))
        with Path(local_path).open("wb") as file:
            file.write(data)
