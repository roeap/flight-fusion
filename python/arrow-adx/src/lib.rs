mod client;
mod errors;
mod utils;

use pyo3::prelude::*;

#[pyfunction]
fn rust_core_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// High performance access to Kusto (Azure Data Explorer)
#[pymodule]
fn _internal(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(rust_core_version, m)?)?;
    m.add_class::<client::PyKustoClient>()?;
    Ok(())
}
