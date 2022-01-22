use flight_fusion::start_server;
use pyo3::prelude::*;
use tokio::runtime::Runtime;

mod error;

#[pyclass(module = "flight_fusion_server")]
struct FusionServer {}

#[pymethods]
impl FusionServer {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {})
    }

    fn run(&self) -> PyResult<()> {
        let rt = Runtime::new().unwrap();
        rt.block_on(start_server()).unwrap();
        Ok(())
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
