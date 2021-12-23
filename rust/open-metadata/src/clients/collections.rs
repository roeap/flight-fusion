use crate::operations::{GerVersionBuilder, ListCollectionsBuilder};
use super::databases::DatabasesCollectionClient;
use http::{method::Method, request::Builder as RequestBuilder};
use reqwest_pipeline::{ClientOptions, Pipeline, Request};
use url::Url;

pub const ROUTE_DATABASES: &str = "api/v1/databases";

#[derive(Debug, Clone)]
pub struct OpenMetadataClient {
    pub user_agent: Option<String>,
    pipeline: Pipeline,
    routes: ApiRoutes,
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

    // #[cfg(feature = "mock_transport_framework")]
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
        let routes = ApiRoutes::new(service_url);
        let pipeline = new_pipeline_from_options(options);

        Self {
            user_agent: None,
            pipeline,
            routes,
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

    pub fn into_databases_collection_client(&self) -> DatabasesCollectionClient {
        DatabasesCollectionClient::new(self.clone())
    }

    pub fn api_routes(&self) -> &ApiRoutes {
        &self.routes
    }

    /// Get the version for the service
    pub fn get_version(&self) -> GerVersionBuilder {
        GerVersionBuilder::new(self.clone())
    }

    /// List all collections available in the service
    pub fn list_collections(&self) -> ListCollectionsBuilder {
        ListCollectionsBuilder::new(self.clone())
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

#[derive(Debug, Clone)]
pub struct ApiRoutes {
    catalog_url: Url,
    bots_url: Url,
    charts_url: Url,
    dashboards_url: Url,
    databases_url: Url,
    tables_url: Url,
    events_url: Url,
    feeds_url: Url,
    lineage_url: Url,
}

impl ApiRoutes {
    /// Create new options
    pub fn new(service_url: Url) -> Self {
        let catalog_url = service_url.join("api/v1").unwrap();
        let bots_url = service_url.join("api/v1/bots").unwrap();
        let charts_url = service_url.join("api/v1/charts").unwrap();
        let dashboards_url = service_url.join("api/v1/dashboards").unwrap();
        let databases_url = service_url.join("api/v1/databases").unwrap();
        let tables_url = service_url.join("api/v1/tables").unwrap();
        let events_url = service_url.join("api/v1/events").unwrap();
        let feeds_url = service_url.join("api/v1/feed").unwrap();
        let lineage_url = service_url.join("api/v1/lineage").unwrap();

        Self {
            catalog_url,
            bots_url,
            charts_url,
            dashboards_url,
            databases_url,
            tables_url,
            events_url,
            feeds_url,
            lineage_url,
        }
    }

    pub fn catalog(&self) -> &Url {
        &self.catalog_url
    }

    pub fn bots(&self) -> &Url {
        &self.bots_url
    }

    pub fn charts(&self) -> &Url {
        &self.charts_url
    }

    pub fn dashboards(&self) -> &Url {
        &self.dashboards_url
    }

    pub fn databases(&self) -> &Url {
        &self.databases_url
    }

    pub fn tables(&self) -> &Url {
        &self.tables_url
    }

    pub fn events(&self) -> &Url {
        &self.events_url
    }

    pub fn feeds(&self) -> &Url {
        &self.feeds_url
    }

    pub fn lineage(&self) -> &Url {
        &self.lineage_url
    }
}
