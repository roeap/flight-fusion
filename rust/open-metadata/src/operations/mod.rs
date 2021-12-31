use crate::generated::{CollectionDescriptor, Database, Paging, Table};
pub use collections::*;
pub use databases::*;
pub use futures::StreamExt;
use http::{HeaderMap, HeaderValue};
use reqwest_pipeline::{
    collect_pinned_stream, Continuable, CustomHeaders, Response, Result as RPResult,
};
use serde::Deserialize;
pub use services::*;
pub use tables::*;

pub mod collections;
pub mod databases;
pub mod services;
pub mod tables;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PagedReturn<T> {
    pub data: Vec<T>,
    pub paging: Option<Paging>,
}

impl<T> Continuable for PagedReturn<T> {
    fn continuation(&self) -> Option<String> {
        match &self.paging {
            Some(page) => page.after.clone(),
            _ => None,
        }
    }
}

impl<T> IntoIterator for PagedReturn<T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl PagedReturn<Database> {
    pub(crate) async fn try_from(response: Response) -> RPResult<Self> {
        let (_status_code, _headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;
        Ok(serde_json::from_slice(&body).unwrap())
    }
}

impl PagedReturn<Table> {
    pub(crate) async fn try_from(response: Response) -> RPResult<Self> {
        let (_status_code, _headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;
        Ok(serde_json::from_slice(&body).unwrap())
    }
}

impl PagedReturn<CollectionDescriptor> {
    pub(crate) async fn try_from(response: Response) -> RPResult<Self> {
        let (_status_code, _headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;
        Ok(serde_json::from_slice(&body).unwrap())
    }
}

fn get_headers() -> CustomHeaders {
    let mut custom_headers = HeaderMap::new();
    custom_headers.insert(
        "Content-Type",
        HeaderValue::from_static("application/json;charset=utf-8"),
    );
    custom_headers.into()
}
