use crate::client::KustoClient;
use crate::error::Result;
use crate::models::ResultTable;
use azure_core::Context;

type ExecuteQuery = futures::future::BoxFuture<'static, Result<Vec<ResultTable>>>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecuteQueryRequest {
    db: String,
    csl: String,
}

#[derive(Clone, Debug)]
pub struct ExecuteQueryBuilder {
    client: KustoClient,
    db: String,
    csl: String,
}

impl ExecuteQueryBuilder {
    pub(crate) fn new(client: KustoClient, database: String, query: String) -> Self {
        Self {
            client,
            db: database,
            csl: query,
        }
    }

    pub fn into_future(self) -> ExecuteQuery {
        let uri = self.client.query_url.clone();
        Box::pin(async move {
            let mut request = self.client.prepare_request(uri.as_str(), http::Method::GET);

            let body = ExecuteQueryRequest {
                db: self.db.clone(),
                csl: self.csl.clone(),
            };

            request.set_body(bytes::Bytes::from(serde_json::to_string(&body)?).into());
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
