use flight_fusion_client::{crate_version, FlightFusionClient, FusionClientError};
use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

create_exception!(flight_fusion, FlightClientError, PyException);

impl FlightClientError {
    fn from_tokio(err: tokio::io::Error) -> pyo3::PyErr {
        FlightClientError::new_err(err.to_string())
    }

    fn from_raw(err: FusionClientError) -> pyo3::PyErr {
        FlightClientError::new_err(err.to_string())
    }

    fn from_prost(err: prost::EncodeError) -> pyo3::PyErr {
        FlightClientError::new_err(err.to_string())
    }
}

#[pyfunction]
fn rust_core_version() -> &'static str {
    crate_version()
}

#[inline]
fn rt() -> PyResult<tokio::runtime::Runtime> {
    tokio::runtime::Runtime::new().map_err(FlightClientError::from_tokio)
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
        let response = rt()?
            .block_on(async {
                let client = FlightFusionClient::try_new().await.unwrap();
                client.drop_table(table_name).await
            })
            .map_err(FlightClientError::from_raw)?;
        let obj = serialize_message(response).map_err(FlightClientError::from_prost)?;
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

#[pymodule]
fn flight_fusion(py: Python, m: &PyModule) -> PyResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    m.add_function(pyo3::wrap_pyfunction!(rust_core_version, m)?)?;
    m.add_class::<FusionClient>()?;
    m.add("FlightClientError", py.get_type::<FlightClientError>())?;
    Ok(())
}
