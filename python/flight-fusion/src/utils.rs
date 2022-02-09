use pyo3::prelude::*;
use std::future::Future;
use tokio::runtime::Runtime;

/// Utility to collect rust futures with GIL released
#[allow(dead_code)]
pub(crate) fn wait_for_future<F: Future>(py: Python, f: F) -> F::Output
where
    F: Send,
    F::Output: Send,
{
    let rt = Runtime::new().unwrap();
    py.allow_threads(|| rt.block_on(f))
}

pub(crate) fn wait_for_future_blocking<F: Future>(f: F) -> F::Output
where
    F: Send,
    F::Output: Send,
{
    let rt = Runtime::new().unwrap();
    rt.block_on(f)
}
