use crate::models::{CatalogVersion, CollectionList};
use reqwest_pipeline::{collect_pinned_stream, Response};

impl CollectionList {
    pub(crate) async fn try_from(response: Response) -> reqwest_pipeline::Result<Self> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;
        Ok(serde_json::from_slice::<CollectionList>(&body)?)
    }
}

impl CatalogVersion {
    pub(crate) async fn try_from(response: Response) -> reqwest_pipeline::Result<Self> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;
        Ok(serde_json::from_slice::<CatalogVersion>(&body)?)
    }
}
