use bytes::Bytes;
use http::{
    header::*,
    method::Method,
    request::{Builder, Request},
};
use log::debug;
use reqwest::Client;
use std::convert::TryFrom;
use std::num::NonZeroU32;
use std::sync::Arc;
use url::Url;

pub mod databases;

/// An empty HTTP body.
#[allow(clippy::declare_interior_mutable_const)]
pub const EMPTY_BODY: bytes::Bytes = bytes::Bytes::from_static(&[]);

/// Add a new query pair into the target URL's query string.
pub trait AppendToUrlQuery {
    fn append_to_url_query(&self, url: &mut url::Url);
}

impl<T> AppendToUrlQuery for Option<T>
where
    T: AppendToUrlQuery,
{
    fn append_to_url_query(&self, url: &mut url::Url) {
        if let Some(i) = self {
            i.append_to_url_query(url);
        }
    }
}

#[derive(Debug, Clone)]
pub struct OpenMetadataClient {
    pub service_url: Url,
    pub base_path: String,
    pub user_agent: Option<String>,
    http_client: Arc<Client>,
    databases_url: Url,
}

impl OpenMetadataClient {
    pub fn new<T>(url: T) -> Self
    where
        T: Into<String>,
    {
        let service_url = Url::parse(&url.into()).expect("A valid service url must ne provided");
        let databases_url = service_url.join("api/v1/databases").unwrap();

        Self {
            service_url,
            base_path: "/api".to_string(),
            user_agent: None,
            http_client: Arc::new(Client::new()),
            databases_url,
        }
    }

    pub fn http_client(&self) -> &Client {
        self.http_client.as_ref()
    }

    pub fn databases_url(&self) -> &Url {
        &self.databases_url
    }

    pub(crate) fn prepare_request(
        &self,
        url: &str,
        method: &Method,
        request_body: Option<Bytes>,
    ) -> crate::Result<(Request<Bytes>, url::Url)> {
        let dt = chrono::Utc::now();
        let time = format!("{}", dt.format("%a, %d %h %Y %T GMT"));

        let mut url = url::Url::parse(url)?;

        let mut request = Request::builder();
        request = request.method(method).uri(url.as_str());

        let request = if let Some(request_body) = request_body {
            request.body(request_body)
        } else {
            request.body(EMPTY_BODY)
        }?;

        debug!("using request == {:#?}", request);

        Ok((request, url))
    }
}

// This type forbids zero as value.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct MaxResults(NonZeroU32);

impl MaxResults {
    pub fn new(max_results: NonZeroU32) -> Self {
        Self(max_results)
    }
}

impl AppendToUrlQuery for MaxResults {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut()
            .append_pair("limit", &format!("{}", self.0));
    }
}

impl From<NonZeroU32> for MaxResults {
    fn from(max_results: NonZeroU32) -> Self {
        Self::new(max_results)
    }
}

impl TryFrom<u32> for MaxResults {
    type Error = String;

    fn try_from(max_results: u32) -> Result<Self, Self::Error> {
        match NonZeroU32::new(max_results) {
            Some(max_results) => Ok(max_results.into()),
            None => Err(format!(
                "number {} is not a valid NonZeroU32 value",
                max_results
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldsQuery(String);

impl FieldsQuery {
    pub fn new(fields: String) -> Self {
        Self(fields)
    }
}

impl AppendToUrlQuery for FieldsQuery {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut()
            .append_pair("fields", &format!("{}", self.0));
    }
}

impl From<String> for FieldsQuery {
    fn from(fields: String) -> Self {
        Self::new(fields)
    }
}

#[derive(Debug, Clone)]
pub struct PagingBefore(String);

impl PagingBefore {
    pub fn new(fields: String) -> Self {
        Self(fields)
    }
}

impl AppendToUrlQuery for PagingBefore {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut()
            .append_pair("before", &format!("{}", self.0));
    }
}

impl From<String> for PagingBefore {
    fn from(fields: String) -> Self {
        Self::new(fields)
    }
}

#[derive(Debug, Clone)]
pub struct PagingAfter(String);

impl PagingAfter {
    pub fn new(fields: String) -> Self {
        Self(fields)
    }
}

impl AppendToUrlQuery for PagingAfter {
    fn append_to_url_query(&self, url: &mut url::Url) {
        url.query_pairs_mut()
            .append_pair("before", &format!("{}", self.0));
    }
}

impl From<String> for PagingAfter {
    fn from(fields: String) -> Self {
        Self::new(fields)
    }
}
