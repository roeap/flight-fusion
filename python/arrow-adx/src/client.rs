use crate::utils::wait_for_future;
use azure_kusto_data::prelude::*;
use pyo3::prelude::*;
use std::sync::Arc;

#[pyclass(name = "KustoClient", module = "arrow_adx")]
pub(crate) struct PyKustoClient {
    client: Arc<KustoClient>,
}

#[pymethods]
impl PyKustoClient {
    #[new]
    fn new(service_url: String) -> Self {
        let credential = Arc::new(EnvironmentCredential::new(TokenCredentialOptions::default()));
        let options = KustoClientOptions::default();
        let client =
            Arc::new(KustoClient::new_with_options(service_url, credential, options).unwrap());
        PyKustoClient { client }
    }

    fn execute(
        &self,
        py: Python,
        database: String,
        query: String,
    ) -> Vec<arrow::record_batch::RecordBatch> {
        let future = async {
            let data = self
                .client
                .execute_query(database, query)
                .into_future()
                .await
                .unwrap();
            data
        };
        let result = wait_for_future(py, future);
        let batches = result
            .into_record_batches()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        batches
    }
}
