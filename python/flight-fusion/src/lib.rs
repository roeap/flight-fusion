use error::FusionClientError;
use flight_fusion_client::{crate_version, FlightFusionClient};
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use utils::wait_for_future;

mod error;
mod utils;

#[pyfunction]
fn rust_core_version() -> &'static str {
    crate_version()
}

#[pyclass]
struct FusionClient {}

#[pymethods]
impl FusionClient {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {})
    }

    pub fn drop_table<'py>(&self, py: Python<'py>, table_name: &str) -> PyResult<&'py PyBytes> {
        let op = async {
            let client = FlightFusionClient::try_new().await.unwrap();
            client.drop_table(table_name).await
        };
        let response = wait_for_future(py, op).map_err(FusionClientError::from)?;
        let obj = serialize_message(response).map_err(FusionClientError::from)?;
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

    m.add_function(pyo3::wrap_pyfunction!(rust_core_version, m)?)?;
    m.add_class::<FusionClient>()?;
    m.add("FlightClientError", py.get_type::<FlightClientError>())?;
    Ok(())
}
