use crate::models::{CollectionList, CatalogVersion};
use http::{method::Method, request::Builder as RequestBuilder};
use reqwest_pipeline::{ClientOptions, Context, Pipeline, Request};
use url::Url;
use databases::DatabasesCollectionClient;

pub mod databases;

pub const ROUTE_DATABASES: &str = "api/v1/databases";

#[derive(Debug, Clone)]
pub struct OpenMetadataClient {
    pub service_url: Url,
    pub user_agent: Option<String>,
    pipeline: Pipeline,
    databases_url: Url,
}

/// Options for specifying how a OpenMetadata client will behave
#[derive(Debug, Clone, Default)]
pub struct OpenMetadataOptions {
    options: ClientOptions,
}

impl OpenMetadataOptions {
    /// Create new options
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(feature = "mock_transport_framework")]
    /// Create new options with a given transaction name
    pub fn new_with_transaction_name(name: String) -> Self {
        Self {
            options: ClientOptions::new_with_transaction_name(name.into()),
        }
    }
}

/// Create a Pipeline from OpenMetadataOptions
fn new_pipeline_from_options(
    options: OpenMetadataOptions,
    // authorization_token: AuthorizationToken,
) -> Pipeline {
    // let auth_policy: Arc<dyn azure_core::Policy> =
    //    Arc::new(crate::AuthorizationPolicy::new(authorization_token));

    // take care of adding the AuthorizationPolicy as **last** retry policy.
    // Policies can change the url and/or the headers and the AuthorizationPolicy
    // must be able to inspect them or the resulting token will be invalid.
    // TODO add authorization policy
    let per_retry_policies = vec![]; // vec![auth_policy];

    Pipeline::new(
        option_env!("CARGO_PKG_NAME"),
        option_env!("CARGO_PKG_VERSION"),
        options.options,
        Vec::new(),
        per_retry_policies,
    )
}

impl OpenMetadataClient {
    pub fn new<T>(url: T, options: OpenMetadataOptions) -> Self
    where
        T: Into<String>,
    {
        let service_url = Url::parse(&url.into()).expect("A valid service url must ne provided");
        let databases_url = service_url.join( "api/v1/databases").unwrap();

        let pipeline = new_pipeline_from_options(options);

        Self {
            service_url,
            user_agent: None,
            pipeline,
            databases_url,
        }
    }

    #[cfg(feature = "mock_transport_framework")]
    /// Create new options with a given transaction name
    pub fn new_with_transaction(
        url: impl Into<String>,
        transaction_name: impl Into<String>,
    ) -> Self {
        Self::new(
            url.into(),
            OpenMetadataOptions::new_with_transaction_name(transaction_name.into()),
        )
    }

    pub fn databases_url(&self) -> &Url {
        &self.databases_url
    }

    pub fn into_databases_collection_client(&self) -> DatabasesCollectionClient {
        DatabasesCollectionClient::new(self.clone())
    }

    pub async fn get_version(&self) -> crate::Result<CatalogVersion> {
        let url = self.service_url.join("api/v1/version").unwrap();
        let mut request = self.prepare_request(url.as_str(), http::Method::GET);
        // options.decorate_request(&mut request)?;
        let response = self.pipeline().send(&mut Context::new(), &mut request).await?;
        Ok(CatalogVersion::try_from(response).await?)
    }

    /// Get the database
    pub async fn list_collections(&self, ctx: Context) -> crate::Result<CollectionList> {
        let url = self.service_url.join("api/v1").unwrap();
        let mut request = self.prepare_request(url.as_str(), http::Method::GET);
        // options.decorate_request(&mut request)?;
        let response = self.pipeline().send(&mut ctx.clone(), &mut request).await?;
        Ok(CollectionList::try_from(response).await?)
    }

    pub(crate) fn prepare_request(&self, url: &str, http_method: Method) -> Request {
        RequestBuilder::new()
            .method(http_method)
            .uri(url)
            .body(bytes::Bytes::new())
            .unwrap()
            .into()
    }

    pub(crate) fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_collections() {
        let client =
            OpenMetadataClient::new("http://localhost:8585", OpenMetadataOptions::default());

        let collections = client.list_collections(Context::default()).await.unwrap();

        println!("{:?}", collections)
    }
}
