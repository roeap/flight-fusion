use object_store::local::LocalFileSystem;
use object_store::path::{Error as PathError, Path};
use object_store::{DynObjectStore, Error as InnerObjectStoreError, ObjectMeta};
use pyo3::prelude::*;
use pyo3::{
    exceptions::{PyException, PyFileExistsError, PyFileNotFoundError},
    types::PyBytes,
    PyErr,
};
use std::fmt;
use std::future::Future;
use std::sync::Arc;
use tokio::runtime::Runtime;

#[derive(Debug)]
pub enum ObjectStoreError {
    ObjectStore(InnerObjectStoreError),
    Common(String),
    Python(PyErr),
    Path(PathError),
}

impl fmt::Display for ObjectStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ObjectStoreError::ObjectStore(e) => write!(f, "ObjectStore error: {:?}", e),
            ObjectStoreError::Python(e) => write!(f, "Python error {:?}", e),
            ObjectStoreError::Path(e) => write!(f, "Path error {:?}", e),
            ObjectStoreError::Common(e) => write!(f, "{}", e),
        }
    }
}

impl From<InnerObjectStoreError> for ObjectStoreError {
    fn from(err: InnerObjectStoreError) -> ObjectStoreError {
        ObjectStoreError::ObjectStore(err)
    }
}

impl From<PathError> for ObjectStoreError {
    fn from(err: PathError) -> ObjectStoreError {
        ObjectStoreError::Path(err)
    }
}

impl From<PyErr> for ObjectStoreError {
    fn from(err: PyErr) -> ObjectStoreError {
        ObjectStoreError::Python(err)
    }
}

impl From<ObjectStoreError> for PyErr {
    fn from(err: ObjectStoreError) -> PyErr {
        match err {
            ObjectStoreError::Python(py_err) => py_err,
            ObjectStoreError::ObjectStore(store_err) => match store_err {
                InnerObjectStoreError::NotFound { .. } => {
                    PyFileNotFoundError::new_err(store_err.to_string())
                }
                InnerObjectStoreError::AlreadyExists { .. } => {
                    PyFileExistsError::new_err(store_err.to_string())
                }
                _ => PyException::new_err(store_err.to_string()),
            },
            _ => PyException::new_err(err.to_string()),
        }
    }
}

#[pyclass(name = "Path", module = "object_store", subclass)]
#[derive(Clone)]
struct PyPath(Path);

impl From<PyPath> for Path {
    fn from(path: PyPath) -> Self {
        path.0
    }
}

#[pymethods]
impl PyPath {
    #[new]
    fn new(path: String) -> PyResult<Self> {
        Ok(Self(Path::parse(path).map_err(ObjectStoreError::from)?))
    }

    /// Creates a new child of this [`Path`]
    fn child(&self, part: String) -> Self {
        Self(self.0.child(part))
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }
}

#[pyclass(name = "ObjectMeta", module = "object_store", subclass)]
#[derive(Clone)]
struct PyObjectMeta {
    inner: ObjectMeta,
}

#[pyclass(name = "ObjectStore", module = "object_store", subclass)]
struct PyObjectStore {
    inner: Arc<DynObjectStore>,
}

impl From<Arc<DynObjectStore>> for PyObjectStore {
    fn from(inner: Arc<DynObjectStore>) -> Self {
        Self { inner }
    }
}

impl PyObjectStore {
    async fn get_inner(&self, location: &Path) -> PyResult<Vec<u8>> {
        Ok(self
            .inner
            .get(location)
            .await
            .map_err(ObjectStoreError::from)?
            .bytes()
            .await
            .map_err(ObjectStoreError::from)?
            .into())
    }
}

#[pymethods]
impl PyObjectStore {
    #[new]
    fn new(root: String) -> PyResult<Self> {
        let store = LocalFileSystem::new_with_prefix(root).map_err(ObjectStoreError::from)?;
        Ok(Self {
            inner: Arc::new(store),
        })
    }

    /// Save the provided bytes to the specified location.
    fn put(&self, location: PyPath, bytes: Vec<u8>, py: Python) -> PyResult<()> {
        wait_for_future(py, self.inner.put(&location.into(), bytes.into()))
            .map_err(ObjectStoreError::from)?;
        Ok(())
    }

    /// Return the bytes that are stored at the specified location.
    fn get<'py>(&self, location: PyPath, py: Python<'py>) -> PyResult<&'py PyBytes> {
        let obj = wait_for_future(py, self.get_inner(&location.into()))?;
        Ok(PyBytes::new(py, &obj))
    }
}

/// Utility to collect rust futures with GIL released
pub fn wait_for_future<F: Future>(py: Python, f: F) -> F::Output
where
    F: Send,
    F::Output: Send,
{
    let rt = Runtime::new().unwrap();
    py.allow_threads(|| rt.block_on(f))
}

#[pymodule]
fn _internal(_py: Python, m: &PyModule) -> PyResult<()> {
    // Register the python classes
    m.add_class::<PyObjectStore>()?;
    m.add_class::<PyPath>()?;

    Ok(())
}
