use crate::generated::Paging;
pub use collections::*;
pub use databases::*;
pub use futures::StreamExt;
use reqwest_pipeline::Continuable;
pub use services::*;

pub mod collections;
pub mod databases;
pub mod services;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PagedReturn<T> {
    pub data: Vec<T>,
    pub paging: Option<Paging>,
}

impl<T> Continuable for PagedReturn<T> {
    fn continuation(&self) -> Option<String> {
        match &self.paging {
            Some(page) => page.after.clone(),
            _ => None,
        }
    }
}

impl<T> IntoIterator for PagedReturn<T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
