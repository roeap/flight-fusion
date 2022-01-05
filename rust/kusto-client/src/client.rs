use crate::authorization_policy::{AuthorizationPolicy, AutoRefreshingCredential};
use crate::connection_string::ConnectionString;
use crate::error::{KustoRsError, Result};
use crate::request::ExecuteQueryBuilder;
use azure_core::auth::TokenCredential;
use azure_core::{ClientOptions, Pipeline, Request};
use azure_identity::token_credentials::{
    AzureCliCredential, ClientSecretCredential, DefaultAzureCredential,
    ImdsManagedIdentityCredential, TokenCredentialOptions,
};
use http::{method::Method, request::Builder as RequestBuilder};
use std::convert::{From, TryFrom};
use std::fmt::Debug;
use std::sync::Arc;
use url::Url;

/// Options for specifying how a OpenMetadata client will behave
#[derive(Clone, Default)]
pub struct KustoClientOptions {
    options: ClientOptions,
    pub credential: Option<Arc<dyn TokenCredential>>,
    pub resource: String,
}

impl KustoClientOptions {
    /// Create new options
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(feature = "mock_transport_framework")]
    /// Create new options with a given transaction name
    pub fn new_with_transaction_name<T: Into<String>>(name: T) -> Self {
        Self {
            options: ClientOptions::new_with_transaction_name(name.into()),
        }
    }
}

fn new_pipeline_from_options(options: KustoClientOptions) -> Pipeline {
    let credential = if let Some(cred) = options.credential {
        Arc::new(AutoRefreshingCredential::new(cred.clone()))
    } else {
        Arc::new(AutoRefreshingCredential::new(Arc::new(
            DefaultAzureCredential::default(),
        )))
    };
    let auth_policy = Arc::new(AuthorizationPolicy::new(credential, &options.resource));
    // take care of adding the AuthorizationPolicy as **last** retry policy.
    let per_retry_policies: Vec<Arc<(dyn azure_core::Policy + 'static)>> = vec![auth_policy];

    Pipeline::new(
        option_env!("CARGO_PKG_NAME"),
        option_env!("CARGO_PKG_VERSION"),
        options.options,
        Vec::new(),
        per_retry_policies,
    )
}

/// Kusto client for Rust.
/// The client is a wrapper around the Kusto REST API.
/// To read more about it, go to https://docs.microsoft.com/en-us/azure/kusto/api/rest/
///
/// The primary methods are:
/// `execute_query`:  executes a KQL query against the Kusto service.
#[derive(Clone, Debug)]
pub struct KustoClient {
    pipeline: Pipeline,
    pub query_url: Url,
    pub management_url: Url,
}

impl KustoClient {
    pub fn new_with_options<T>(url: T, options: KustoClientOptions) -> Result<Self>
    where
        T: Into<String>,
    {
        let service_url = Url::parse(&url.into()).expect("A valid service url must ne provided");
        let query_url = service_url.join("v2/rest/query").unwrap();
        let management_url = service_url.join("v1/rest/mgmt").unwrap();
        let pipeline = new_pipeline_from_options(options);

        Ok(Self {
            pipeline,
            query_url,
            management_url,
        })
    }

    /// Execute a KQL query.
    /// To learn more about KQL go to https://docs.microsoft.com/en-us/azure/kusto/query/
    ///
    /// # Arguments
    ///
    /// * `database` - Database against query will be executed.
    /// * `query` - Query to be executed.
    pub fn execute_query(&self, database: &str, query: &str) -> ExecuteQueryBuilder {
        ExecuteQueryBuilder::new(self.clone(), database.into(), query.into())
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

impl<'a> TryFrom<ConnectionString<'a>> for KustoClient {
    type Error = KustoRsError;

    fn try_from(value: ConnectionString) -> Result<Self> {
        let service_url = value
            .data_source
            .expect("A data source / service url must always be specified");
        let resource = if service_url.ends_with('/') {
            String::from(service_url)
        } else {
            format!("{}/", service_url)
        };

        let credential: Arc<dyn TokenCredential> = match value {
            ConnectionString {
                application_client_id: Some(client_id),
                application_key: Some(client_secret),
                authority_id: Some(tenant_id),
                ..
            } => Arc::new(ClientSecretCredential::new(
                tenant_id.to_string(),
                client_id.to_string(),
                client_secret.to_string(),
                TokenCredentialOptions::default(),
            )),
            ConnectionString {
                msi_auth: Some(true),
                ..
            } => Arc::new(ImdsManagedIdentityCredential {}),
            ConnectionString {
                az_cli: Some(true), ..
            } => Arc::new(AzureCliCredential {}),
            _ => Arc::new(DefaultAzureCredential::default()),
        };
        let mut options = KustoClientOptions::new();
        options.credential = Some(credential);
        options.resource = resource;

        Self::new_with_options(service_url, options)
    }
}

impl TryFrom<String> for KustoClient {
    type Error = KustoRsError;

    fn try_from(value: String) -> Result<Self> {
        let connection_string = ConnectionString::new(value.as_str())?;
        Self::try_from(connection_string)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use dotenv::dotenv;
//
//     #[tokio::test]
//     async fn test_query() {
//         dotenv().ok();
//         let options = KustoClientOptions::new();
//         let client =
//             KustoClient::new_with_options("https://chronos.kusto.windows.net", options).unwrap();
//         let query = "signals | sample 10";
//         let result = client
//             .execute_query("argus-stage", query)
//             .into_future()
//             .await
//             .unwrap();
//         println!("{:#?}", result)
//     }
// }
