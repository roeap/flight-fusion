use flight_fusion::start_server;
use pyo3::prelude::*;
use std::future::Future;
use tokio::runtime::Runtime;

mod error;

pub(crate) fn wait_for_future<F: Future>(_py: Python, f: F) -> F::Output
where
    F: Send,
    F::Output: Send,
{
    let rt = Runtime::new().unwrap();
    rt.block_on(f)
}

#[pyclass(module = "flight_fusion_server")]
struct FusionServer {}

#[pymethods]
impl FusionServer {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {})
    }

    fn run<'py>(&self, py: Python<'py>) -> PyResult<()> {
        Ok(wait_for_future(py, start_server()).map_err(error::FlightFusionServerError::from)?)
    }
}

/// Low-level flight fusion internal package.
///
/// The higher-level public API is defined in pure python files under the
/// flight_fusion directory.
#[pymodule]
fn _internal(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<FusionServer>()?;
    // m.add(
    //     "FlightFusionServerError",
    //     py.get_type::<FlightFusionServerError>(),
    // )?;
    Ok(())
}
