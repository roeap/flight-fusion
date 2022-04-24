use core::fmt;

use arrow::error::ArrowError;
use azure_kusto_data::error::Error as InnerError;
use pyo3::{exceptions::PyException, PyErr};

#[derive(Debug)]
pub enum KustoAdxError {
    ExecutionError(InnerError),
    ArrowError(ArrowError),
    Common(String),
}

impl fmt::Display for KustoAdxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KustoAdxError::ExecutionError(e) => write!(f, "KustoRs error: {:?}", e),
            KustoAdxError::ArrowError(e) => write!(f, "Arrow error: {:?}", e),
            KustoAdxError::Common(e) => write!(f, "{}", e),
        }
    }
}

impl From<ArrowError> for KustoAdxError {
    fn from(err: ArrowError) -> KustoAdxError {
        KustoAdxError::ArrowError(err)
    }
}

impl From<InnerError> for KustoAdxError {
    fn from(err: InnerError) -> KustoAdxError {
        KustoAdxError::ExecutionError(err)
    }
}

impl From<KustoAdxError> for PyErr {
    fn from(err: KustoAdxError) -> PyErr {
        PyException::new_err(err.to_string())
    }
}
