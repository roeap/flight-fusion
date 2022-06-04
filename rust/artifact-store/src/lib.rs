use async_trait::async_trait;
use futures::Stream;
use futures::StreamExt;
use gen::{
    download_artifact::Response as DownloadArtifactResponse,
    list_artifacts::Response as ListArtifactsResponse,
    mlflow_artifacts_service_server::MlflowArtifactsService,
    upload_artifact::Response as UploadArtifactResponse, *,
};
use object_store::{local::LocalFileSystem, path::Path, DynObjectStore, Error as ObjectStoreError};
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use tonic::{Request, Response, Status, Streaming};

pub type BoxedFileStream<T> =
    Pin<Box<dyn Stream<Item = Result<T, Status>> + Send + Sync + 'static>>;

#[rustfmt::skip]
mod gen {
    include!("gen/mlflow.artifacts.rs");
}

pub struct MlflowArtifacts {
    object_store: Arc<DynObjectStore>,
}

impl From<ListArtifacts> for Path {
    fn from(req: ListArtifacts) -> Self {
        Self::from(req.path.unwrap_or("".to_string()))
    }
}

impl From<UploadArtifact> for Path {
    fn from(req: UploadArtifact) -> Self {
        Self::from(req.path)
    }
}

impl From<DownloadArtifact> for Path {
    fn from(req: DownloadArtifact) -> Self {
        Self::from(req.path)
    }
}

impl MlflowArtifacts {
    pub fn new(object_store: Arc<DynObjectStore>) -> Self {
        Self { object_store }
    }

    pub fn new_default(root: impl Into<PathBuf>) -> Result<Self, ObjectStoreError> {
        let buf: PathBuf = root.into();
        let object_store = Arc::new(LocalFileSystem::new_with_prefix(buf)?);
        Ok(Self { object_store })
    }
}

#[async_trait]
impl MlflowArtifactsService for MlflowArtifacts {
    type downloadArtifactStream = BoxedFileStream<DownloadArtifactResponse>;

    async fn list_artifacts(
        &self,
        request: Request<ListArtifacts>,
    ) -> Result<Response<ListArtifactsResponse>, Status> {
        let path = request.into_inner().into();
        let list_path = self
            .object_store
            .list_with_delimiter(Some(&path))
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        let mut files = list_path
            .common_prefixes
            .into_iter()
            .map(|p| FileInfo {
                is_dir: Some(true),
                file_size: Some(0),
                path: Some(p.into()),
            })
            .collect::<Vec<_>>();
        let tmp_files = list_path.objects.into_iter().map(|o| FileInfo {
            is_dir: Some(false),
            file_size: Some(o.size as i64),
            path: Some(o.location.into()),
        });
        files.extend(tmp_files);
        Ok(Response::new(ListArtifactsResponse { files }))
    }

    async fn upload_artifact(
        &self,
        request: Request<Streaming<UploadArtifact>>,
    ) -> Result<Response<UploadArtifactResponse>, Status> {
        todo!()
    }

    async fn download_artifact(
        &self,
        request: Request<DownloadArtifact>,
    ) -> Result<Response<Self::downloadArtifactStream>, Status> {
        let path = request.into_inner().into();
        let data = self
            .object_store
            .get(&path)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .into_stream()
            .map(|data| {
                Ok(DownloadArtifactResponse {
                    data: data.unwrap().to_vec(),
                })
            })
            .collect::<Vec<_>>()
            .await;
        Ok(Response::new(
            Box::pin(futures::stream::iter(data)) as Self::downloadArtifactStream
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use futures::TryStreamExt;
    use object_store::ObjectStore;

    #[tokio::test]
    async fn list_artifacts() {
        let root = tempfile::tempdir().unwrap();
        let object_store = Arc::new(LocalFileSystem::new_with_prefix(root).unwrap());
        let service = MlflowArtifacts::new(object_store.clone());

        let directory = Path::from("directory");
        let object = directory.child("child.txt");
        let data = Bytes::from("arbitrary");
        object_store.put(&object, data.clone()).await.unwrap();

        let request = Request::new(ListArtifacts {
            path: Some(directory.into()),
        });
        let response = service.list_artifacts(request).await.unwrap().into_inner();

        assert_eq!(response.files.len(), 1);
        assert_eq!(response.files[0].path, Some(object.into()));
    }

    #[tokio::test]
    async fn download_artifact() {
        let root = tempfile::tempdir().unwrap();
        let object_store = Arc::new(LocalFileSystem::new_with_prefix(root).unwrap());
        let service = MlflowArtifacts::new(object_store.clone());

        let directory = Path::from("directory");
        let object = directory.child("child.txt");
        let data = Bytes::from("arbitrary");
        object_store.put(&object, data.clone()).await.unwrap();

        let request = Request::new(DownloadArtifact {
            path: object.into(),
        });
        let response = service
            .download_artifact(request)
            .await
            .unwrap()
            .into_inner()
            .try_collect::<Vec<_>>()
            .await
            .unwrap();

        let download_data = Bytes::from(response[0].data.clone());
        assert_eq!(data, download_data);
    }
}
