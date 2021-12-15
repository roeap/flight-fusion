use super::{ActionHandler, FusionActionHandler};
use datafusion::catalog::catalog::CatalogProvider;
use datafusion::datasource::MemTable;
use datafusion::parquet::arrow::ArrowReader;
use flight_fusion_ipc::{
    DatasetFormat, DropDatasetRequest, DropDatasetResponse, RegisterDatasetRequest,
    RegisterDatasetResponse, Result as FusionResult,
};
use std::sync::Arc;

#[async_trait::async_trait]
impl ActionHandler<DropDatasetRequest> for FusionActionHandler {
    async fn handle_do_action(
        &self,
        action: DropDatasetRequest,
    ) -> FusionResult<DropDatasetResponse> {
        let response = DropDatasetResponse { name: action.name };
        Ok(response)
    }
}

#[async_trait::async_trait]
impl ActionHandler<RegisterDatasetRequest> for FusionActionHandler {
    async fn handle_do_action(
        &self,
        action: RegisterDatasetRequest,
    ) -> FusionResult<RegisterDatasetResponse> {
        match DatasetFormat::from_i32(action.format) {
            Some(DatasetFormat::File) => {
                let mut reader = self.get_arrow_reader_from_path(action.path).await;
                let schema = Arc::new(reader.get_schema().unwrap());
                let batch_reader = reader.get_record_reader(1024).unwrap();
                let batches = batch_reader
                    .into_iter()
                    .map(|batch| batch.unwrap())
                    .collect::<Vec<_>>();

                // declare a table in memory. In spark API, this corresponds to createDataFrame(...).
                let table_provider = MemTable::try_new(schema, vec![batches]).unwrap();
                let schema_provider = self.catalog.schema("schema").unwrap();
                schema_provider
                    .register_table(action.name.clone(), Arc::new(table_provider))
                    .unwrap();

                self.catalog
                    .register_schema("schema".to_string(), schema_provider);
            }
            Some(DatasetFormat::Dataset) => {
                todo!()
            }
            Some(DatasetFormat::Delta) => (),
            _ => (),
        };

        Ok(RegisterDatasetResponse {
            message: action.name,
        })
    }
}
