from __future__ import annotations

# from mlflow.exceptions import MlflowException
from mlflow.entities import FileInfo
from mlflow.store.artifact.artifact_repo import ArtifactRepository
from mlflow.store.artifact.local_artifact_repo import LocalArtifactRepository
from pydantic import BaseSettings


class FusionArtifactRepositorySetting(BaseSettings):
    pass


class FusionArtifactRepository(ArtifactRepository):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        file_uri = self.artifact_uri.replace("fusion:/", "")
        self._repo = LocalArtifactRepository(artifact_uri=file_uri)

    def log_artifact(self, local_file, artifact_path=None) -> None:
        """
        Log a local file as an artifact, optionally taking an ``artifact_path`` to place it in
        within the run's artifacts. Run artifacts can be organized into directories, so you can
        place the artifact in a directory this way.

        :param local_file: Path to artifact to log
        :param artifact_path: Directory within the run's artifact directory in which to log the
                              artifact.
        """
        self._repo.log_artifact(local_file=local_file, artifact_path=artifact_path)

    def log_artifacts(self, local_dir, artifact_path=None) -> None:
        """
        Log the files in the specified local directory as artifacts, optionally taking
        an ``artifact_path`` to place them in within the run's artifacts.

        :param local_dir: Directory of local artifacts to log
        :param artifact_path: Directory within the run's artifact directory in which to log the
                              artifacts
        """
        self._repo.log_artifacts(local_dir=local_dir, artifact_path=artifact_path)

    def list_artifacts(self, path=None) -> list[FileInfo]:
        """
        Return all the artifacts for this run_id directly under path. If path is a file, returns
        an empty list. Will error if path is neither a file nor directory.

        :param path: Relative source path that contains desired artifacts

        :return: List of artifacts as FileInfo listed directly under path.
        """
        return self._repo.list_artifacts(path=path)

    def _download_file(self, remote_file_path, local_path) -> None:
        """
        Download the file at the specified relative remote path and saves
        it at the specified local path.

        :param remote_file_path: Source path to the remote file, relative to the root
                                 directory of the artifact repository.
        :param local_path: The path to which to save the downloaded file.
        """
        self._repo._download_file(remote_file_path=remote_file_path, local_path=local_path)
