use super::PagedReturn;
use crate::{
    clients::OpenMetadataClient,
    generated::{CatalogVersion, CollectionDescriptor},
};
use reqwest_pipeline::{collect_pinned_stream, Context, Pageable, Response, Result as RPResult};
use std::pin::Pin;

type CreateDatabase = futures::future::BoxFuture<'static, crate::Result<CatalogVersion>>;
type ListCollections = Pin<Box<Pageable<PagedReturn<CollectionDescriptor>>>>;

impl PagedReturn<CollectionDescriptor> {
    pub(crate) async fn try_from(response: Response) -> RPResult<Self> {
        let (_status_code, _headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;
        Ok(serde_json::from_slice(&body)?)
    }
}

impl CatalogVersion {
    pub(crate) async fn try_from(response: Response) -> reqwest_pipeline::Result<Self> {
        let (_status_code, _headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;
        Ok(serde_json::from_slice::<CatalogVersion>(&body)?)
    }
}

#[derive(Clone, Debug)]
pub struct GerVersionBuilder {
    client: OpenMetadataClient,
}

impl GerVersionBuilder {
    pub(crate) fn new(client: OpenMetadataClient) -> Self {
        Self { client }
    }

    pub fn into_future(self) -> CreateDatabase {
        let uri = self.client.api_routes().catalog().clone();
        Box::pin(async move {
            let mut request = self.client.prepare_request(uri.as_str(), http::Method::GET);

            let response = self
                .client
                .pipeline()
                .send(&mut Context::new(), &mut request)
                .await?;

            Ok(CatalogVersion::try_from(response).await?)
        })
    }
}

#[derive(Clone, Debug)]
pub struct ListCollectionsBuilder {
    client: OpenMetadataClient,
}

impl ListCollectionsBuilder {
    pub fn new(client: OpenMetadataClient) -> Self {
        Self { client }
    }

    pub fn into_stream<'a>(self) -> ListCollections {
        let make_request = move |_continuation: Option<String>| {
            let this = self.clone();
            let ctx = Context::new();

            async move {
                let uri = this.client.api_routes().catalog();
                let mut request = this.client.prepare_request(uri.as_str(), http::Method::GET);

                let response = match this
                    .client
                    .pipeline()
                    .send(&mut ctx.clone(), &mut request)
                    .await
                {
                    Ok(r) => r,
                    Err(e) => return Err(e),
                };

                PagedReturn::<CollectionDescriptor>::try_from(response).await
            }
        };

        Box::pin(Pageable::new(make_request))
    }
}
