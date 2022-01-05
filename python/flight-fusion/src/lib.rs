use error::FlightFusionClientError;
use flight_fusion_client::{arrow::record_batch::RecordBatch, FlightFusionClient};
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use utils::wait_for_future;

mod error;
mod utils;

#[pyclass(module = "flight_fusion")]
struct FusionClient {}

#[pymethods]
impl FusionClient {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {})
    }

    fn register_memory_table<'py>(
        &self,
        py: Python<'py>,
        table_name: &str,
        batches: Vec<RecordBatch>,
    ) -> PyResult<&'py PyBytes> {
        let op = async {
            let client = FlightFusionClient::try_new().await.unwrap();
            client.register_memory_table(table_name, batches).await
        };
        let response = wait_for_future(py, op).map_err(FlightFusionClientError::from)?;
        let obj = serialize_message(response).map_err(FlightFusionClientError::from)?;
        Ok(PyBytes::new(py, &obj))
    }

    fn drop_table<'py>(&self, py: Python<'py>, table_name: &str) -> PyResult<&'py PyBytes> {
        let op = async {
            let client = FlightFusionClient::try_new().await.unwrap();
            client.drop_table(table_name).await
        };
        let response = wait_for_future(py, op).map_err(FlightFusionClientError::from)?;
        let obj = serialize_message(response).map_err(FlightFusionClientError::from)?;
        Ok(PyBytes::new(py, &obj))
    }
}

fn serialize_message<T: prost::Message>(
    msg: T,
) -> std::result::Result<Vec<u8>, prost::EncodeError> {
    let mut buf = Vec::new();
    buf.reserve(msg.encoded_len());
    msg.encode(&mut buf)?;
    Ok(buf)
}

/// Low-level flight fusion internal package.
///
/// The higher-level public API is defined in pure python files under the
/// flight_fusion directory.
#[pymodule]
fn _internal(py: Python, m: &PyModule) -> PyResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    m.add_class::<FusionClient>()?;
    // m.add(
    //     "FlightFusionClientError",
    //     py.get_type::<FlightFusionClientError>(),
    // )?;
    Ok(())
}
