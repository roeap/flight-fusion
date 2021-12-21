use super::OpenMetadataClient;
use bytes::Bytes;
use http::{
    header::*,
    method::Method,
    request::{Builder, Request},
};
use reqwest::Client;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct DatabasesCollectionClient {
    service_client: Arc<OpenMetadataClient>,
}

impl DatabasesCollectionClient {
    pub(crate) fn new(service_client: Arc<OpenMetadataClient>) -> Arc<Self> {
        Arc::new(Self { service_client })
    }

    #[allow(dead_code)]
    pub fn service_client(&self) -> &OpenMetadataClient {
        self.service_client.as_ref()
    }

    #[allow(dead_code)]
    pub fn http_client(&self) -> &Client {
        self.service_client.http_client()
    }

    #[allow(dead_code)]
    pub fn prepare_request(
        &self,
        url: &str,
        method: &Method,
        request_body: Option<Bytes>,
    ) -> crate::Result<(Request<Bytes>, url::Url)> {
        self.service_client
            .prepare_request(url, method, request_body)
    }
}
