use crate::errors::*;
use crate::utils::wait_for_future;
use azure_kusto_data::prelude::*;
use pyo3::prelude::*;
use pyo3::types::PyType;
use std::sync::Arc;

#[pyclass(name = "KustoClient", module = "arrow_adx")]
pub(crate) struct PyKustoClient {
    client: Arc<KustoClient>,
}

#[pymethods]
impl PyKustoClient {
    #[new]
    fn new(service_url: String) -> PyResult<Self> {
        let credential = Arc::new(EnvironmentCredential::new(TokenCredentialOptions::default()));
        let options = KustoClientOptions::default();
        let client = Arc::new(
            KustoClient::new_with_options(service_url, credential, options)
                .map_err(KustoAdxError::from)?,
        );
        Ok(PyKustoClient { client })
    }

    #[classmethod]
    fn with_aad_application_key_authentication(
        _cls: &PyType,
        connection_string: String,
        aad_app_id: String,
        app_key: String,
        authority_id: String,
    ) -> PyResult<Self> {
        let credential = Arc::new(ClientSecretCredential::new(
            authority_id,
            aad_app_id,
            app_key,
            TokenCredentialOptions::default(),
        ));
        let options = KustoClientOptions::default();
        let client = Arc::new(
            KustoClient::new_with_options(connection_string, credential, options)
                .map_err(KustoAdxError::from)?,
        );
        Ok(PyKustoClient { client })
    }

    fn execute(
        &self,
        py: Python,
        database: String,
        query: String,
    ) -> PyResult<Vec<arrow::record_batch::RecordBatch>> {
        let future = self.client.execute_query(database, query).into_future();
        let result = wait_for_future(py, future).map_err(KustoAdxError::from)?;
        let batches = result
            .into_record_batches()
            .collect::<Result<Vec<_>, _>>()
            .map_err(KustoAdxError::from)?;
        Ok(batches)
    }
}
