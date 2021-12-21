use crate::{
    clients::OpenMetadataClient,
    models::{Database, ResultListDatabase},
    request_options::{FieldsQuery, MaxResults, PagingAfter, PagingBefore},
};
use reqwest_pipeline::{
    collect_pinned_stream, setters, AppendToUrlQuery, Context, Continuable, Pageable, Response,
    Result as RPResult,
};
use std::sync::Arc;
use futures_util::StreamExt;

#[derive(Debug, Clone)]
pub struct DatabasesCollectionClient {
    open_meta_client: OpenMetadataClient,
    fully_qualified_name: Option<String>,
    entity_id: Option<String>,
}

impl DatabasesCollectionClient {
    pub(crate) fn new(open_meta_client: OpenMetadataClient) -> Self {
        Self {
            open_meta_client,
            fully_qualified_name: None,
            entity_id: None,
        }
    }

    #[allow(dead_code)]
    pub fn open_meta_client(&self) -> &OpenMetadataClient {
        &self.open_meta_client
    }

    pub fn list_databases(&self) -> ListDatabases {
        ListDatabases::new(self.open_meta_client.clone())
    }
}

#[derive(Clone, Debug)]
pub struct ListDatabases {
    open_meta_client: OpenMetadataClient,
    fields: Option<FieldsQuery>,
    // TODO make option
    service: Option<String>,
    limit: Option<MaxResults>,
    before: Option<PagingBefore>,
    after: Option<PagingAfter>,
    context: Option<Context>,
}

impl ListDatabases {
    pub fn new(client: OpenMetadataClient) -> Self {
        Self {
            open_meta_client: client,
            fields: None,
            service: None,
            limit: None,
            before: None,
            after: None,
            context: None,
        }
    }

    setters! {
        fields: FieldsQuery => Some(fields),
        limit: MaxResults => Some(limit),
        before: PagingBefore => Some(before),
        after: PagingAfter => Some(after),
        context: Context => Some(context),
    }

    pub fn into_stream(self) -> Pageable<ResultListDatabase> {
        let make_request = move |continuation: Option<String>| {
            let this = self.clone();
            let ctx = self.context.clone().unwrap_or_default();

            async move {
                let mut uri = this.open_meta_client.databases_url().clone();
                this.fields.append_to_url_query(&mut uri);
                this.limit.append_to_url_query(&mut uri);
                this.before.append_to_url_query(&mut uri);
                this.after.append_to_url_query(&mut uri);

                println!("{:?}", uri);

                let mut request = this
                    .open_meta_client
                    .prepare_request(uri.as_str(), http::Method::GET);

                // if let Some(c) = continuation {
                //     match http::HeaderValue::from_str(c.as_str()) {
                //         Ok(h) => request.headers_mut().append(headers::CONTINUATION, h),
                //         Err(e) => return Err(azure_core::Error::Other(Box::new(e))),
                //     };
                // }

                let response = match this
                    .open_meta_client
                    .pipeline()
                    .send(&mut ctx.clone(), &mut request)
                    .await
                {
                    Ok(r) => r,
                    Err(e) => return Err(e),
                };

                ResultListDatabase::try_from(response).await
            }
        };

        Pageable::new(make_request)
    }
}

impl ResultListDatabase {
    pub(crate) async fn try_from(response: Response) -> RPResult<Self> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;

        Ok(serde_json::from_slice(&body)?)
    }
}

impl Continuable for ResultListDatabase {
    fn continuation(&self) -> Option<String> {
        // TODO actually get continuation token
        None
    }
}

impl IntoIterator for ResultListDatabase {
    type Item = Database;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::clients::{OpenMetadataClient, OpenMetadataOptions};
    use futures_util::StreamExt;

    #[tokio::test]
    async fn test_list_databases() {
        let client = initialize().into_databases_collection_client();

        let databases = Box::pin(client.list_databases().into_stream())
            .next()
            .await
            .unwrap()
            .unwrap();

        assert!(databases.data.len() >= 2)
    }

    #[cfg(not(feature = "mock_transport_framework"))]
    pub fn initialize() -> OpenMetadataClient {
        let client =
            OpenMetadataClient::new("http://localhost:8585", OpenMetadataOptions::default());

        client
    }
}
