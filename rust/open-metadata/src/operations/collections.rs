use super::PagedReturn;
use crate::{
    clients::OpenMetadataClient,
    generated::{CatalogVersion, CollectionDescriptor},
};
use reqwest_pipeline::{Context, Pageable};
use std::pin::Pin;

type CreateDatabase = futures::future::BoxFuture<'static, crate::Result<CatalogVersion>>;
type ListCollections = Pin<Box<Pageable<PagedReturn<CollectionDescriptor>>>>;

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
                .await?
                .into_body_string()
                .await;

            Ok(serde_json::from_str(&response)?)
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
        let make_request = move |_: Option<String>| {
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
