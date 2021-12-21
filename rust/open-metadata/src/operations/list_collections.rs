use crate::{
    clients::OpenMetadataClient,
    models::{CollectionDescriptor, CollectionList},
    request_options::*,
};
use chrono::{DateTime, Utc};
use reqwest_pipeline::{collect_pinned_stream, prelude::*, setters, Pageable, Response};

impl CollectionList {
    pub(crate) async fn try_from(response: Response) -> reqwest_pipeline::Result<Self> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;
        Ok(serde_json::from_slice::<CollectionList>(&body)?)
    }
}

impl Continuable for CollectionList {
    fn continuation(&self) -> Option<String> {
        // TODO actually inspect paging return
        None
    }
}

impl IntoIterator for CollectionList {
    type Item = CollectionDescriptor;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
