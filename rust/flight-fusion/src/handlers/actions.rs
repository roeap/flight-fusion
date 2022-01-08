use super::{ActionHandler, FusionActionHandler};
use crate::area_store::{flatten_list_stream, AreaStore};
use arrow_deps::datafusion::{
    catalog::catalog::CatalogProvider, datasource::MemTable, parquet::arrow::ArrowReader,
};
use flight_fusion_ipc::{
    DatasetFormat, DropDatasetRequest, DropDatasetResponse, FlightFusionError,
    RegisterDatasetRequest, RegisterDatasetResponse, Result as FusionResult,
};
use object_store::ObjectStoreApi;
use std::sync::Arc;

#[async_trait::async_trait]
impl ActionHandler<DropDatasetRequest> for FusionActionHandler {
    async fn handle_do_action(
        &self,
        action: DropDatasetRequest,
    ) -> FusionResult<DropDatasetResponse> {
        if let Some(source) = action.table {
            // TODO remove panic
            let location = self.area_store.get_table_location(&source).unwrap();
            let files = flatten_list_stream(&self.area_store.object_store(), Some(&location))
                .await
                .unwrap();
            for file in files {
                // TODO remove panic
                self.area_store.object_store().delete(&file).await.unwrap();
            }
            self.area_store
                .object_store()
                .delete_dir(&location)
                .await
                .unwrap();
            // TODO return a more meaningful message
            Ok(DropDatasetResponse {
                name: "dropped".to_string(),
            })
        } else {
            Err(FlightFusionError::InputError(
                "missing table reference".to_string(),
            ))
        }
    }
}

// #[async_trait::async_trait]
// impl ActionHandler<RegisterDatasetRequest> for FusionActionHandler {
//     async fn handle_do_action(
//         &self,
//         action: RegisterDatasetRequest,
//     ) -> FusionResult<RegisterDatasetResponse> {
//         match DatasetFormat::from_i32(action.format) {
//             Some(DatasetFormat::File) => {
//                 let mut reader = self.area_store.get_arrow_reader(&action.path).await;
//                 let schema = Arc::new(reader.get_schema().unwrap());
//                 let batch_reader = reader.get_record_reader(1024).unwrap();
//                 let batches = batch_reader
//                     .into_iter()
//                     .map(|batch| batch.unwrap())
//                     .collect::<Vec<_>>();
//                 let table_provider = MemTable::try_new(schema, vec![batches]).unwrap();
//                 let schema_provider = self.catalog.schema("schema").unwrap();
//                 schema_provider
//                     .register_table(action.name.clone(), Arc::new(table_provider))
//                     .unwrap();
//                 self.catalog
//                     .register_schema("schema".to_string(), schema_provider);
//             }
//             Some(DatasetFormat::Dataset) => {
//                 todo!()
//             }
//             Some(DatasetFormat::Delta) => (),
//             _ => (),
//         };
//
//         Ok(RegisterDatasetResponse {
//             message: action.name,
//         })
//     }
// }

#[cfg(test)]
mod tests {
    use crate::area_store::BytesReader;
    use arrow_deps::datafusion::parquet::{
        arrow::{ArrowReader, ParquetFileArrowReader},
        file::serialized_reader::SerializedFileReader,
    };
    use bytes::Bytes;
    use object_store::{path::ObjectStorePath, ObjectStore, ObjectStoreApi};
    use std::sync::Arc;

    #[tokio::test]
    async fn read_file() {
        let ws_root = crate::test_utils::workspace_root().unwrap();
        let ws_root = std::path::Path::new(&ws_root);
        let ws_root = ws_root.join("test");
        println!("{:?}", ws_root);
        let storage = ObjectStore::new_file(ws_root);
        let mut location = storage.new_path();
        location.push_dir("data");
        location.set_file_name("P1.parquet");

        let obj_reader = BytesReader(Bytes::from(
            storage.get(&location).await.unwrap().bytes().await.unwrap(),
        ));
        let file_reader = Arc::new(SerializedFileReader::new(obj_reader).unwrap());
        let mut arrow_reader = ParquetFileArrowReader::new(file_reader);
        let _schema = Arc::new(arrow_reader.get_schema().unwrap());

        // println!("{:?}", schema)
    }
}
