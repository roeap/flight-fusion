use crate::{
    catalog::error::{AreaCatalogResult, AreaCatalogResultStream},
    store::{flatten_list_stream, AreaStore},
};
use object_store::path::{parsed::DirsAndFileName, Path};
use object_store::ObjectStore;
use std::sync::Arc;

pub struct FileIndex {
    store: Arc<dyn AreaStore>,
}

impl FileIndex {
    pub fn new(store: Arc<dyn AreaStore>) -> Self {
        Self { store }
    }

    async fn build_index(&self) -> crate::error::Result<()> {
        let all_files = flatten_list_stream(&self.store.object_store(), None)
            .await?
            .into_iter()
            .map(|p| DirsAndFileName::from(p));
        println!("{:?}", all_files);
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::store::DefaultAreaStore;
//
//     #[tokio::test]
//     async fn create_index() {
//         let area_store = Arc::new(DefaultAreaStore::new(
//             "/home/robstar/github/flight-fusion/.flight-fusion/.fusion",
//         ));
//         let file_index = FileIndex::new(area_store);
//         file_index.build_index().await.unwrap();
//     }
// }
