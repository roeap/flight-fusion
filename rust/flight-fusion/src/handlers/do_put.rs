use super::*;
use crate::{
    error::{FusionServiceError, Result},
    service::FlightFusionService,
};
use area_store::store::AreaStore;
use arrow_deps::datafusion::physical_plan::{collect, ExecutionPlan};
use async_trait::async_trait;
use flight_fusion_ipc::{CommandWriteIntoDataset, ResultDoPutUpdate, SaveMode};
use std::sync::Arc;

#[async_trait]
impl DoPutHandler<CommandWriteIntoDataset> for FlightFusionService {
    async fn handle_do_put(
        &self,
        ticket: CommandWriteIntoDataset,
        input: Arc<dyn ExecutionPlan>,
    ) -> Result<ResultDoPutUpdate> {
        if let Some(source) = ticket.source {
            let location = self.area_store.get_table_location(&source)?;
            let session_ctx = SessionContext::new();
            let task_ctx = session_ctx.task_ctx();
            let batches = collect(input, task_ctx).await?;
            let _adds = self
                .area_store
                .put_batches(
                    batches,
                    &location,
                    SaveMode::from_i32(ticket.save_mode).unwrap_or(SaveMode::Overwrite),
                )
                .await?;
            Ok(ResultDoPutUpdate { statistics: None })
        } else {
            // TODO migrate errors and raise something more meaningful
            Err(FusionServiceError::generic("Source not found"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{get_fusion_handler, get_input_plan};
    use flight_fusion_ipc::{
        area_source_reference::Table as TableReference, AreaSourceReference, AreaTableLocation,
        SaveMode,
    };

    #[tokio::test]
    async fn test_put_table() {
        let root = tempfile::tempdir().unwrap();
        let plan = get_input_plan(None, false);
        let handler = get_fusion_handler(root.path());
        let table_dir = root.path().join("_ff_data/new_table");

        let table = TableReference::Location(AreaTableLocation {
            name: "new_table".to_string(),
            areas: vec![],
        });
        let request = CommandWriteIntoDataset {
            source: Some(AreaSourceReference { table: Some(table) }),
            save_mode: SaveMode::Overwrite.into(),
        };

        assert!(!table_dir.exists());

        let _response = handler
            .handle_do_put(request.clone(), plan.clone())
            .await
            .unwrap();

        assert!(table_dir.is_dir())
    }

    #[tokio::test]
    async fn test_put_table_append_overwrite() {
        let root = tempfile::tempdir().unwrap();
        let plan = get_input_plan(None, false);
        let handler = get_fusion_handler(root.path());
        let table_dir = root.path().join("_ff_data/new_table");

        let table_ref = AreaSourceReference {
            table: Some(TableReference::Location(AreaTableLocation {
                name: "new_table".to_string(),
                areas: vec![],
            })),
        };
        let request = CommandWriteIntoDataset {
            source: Some(table_ref.clone()),
            save_mode: SaveMode::Append.into(),
        };

        assert!(!table_dir.exists());

        let _response = handler
            .handle_do_put(request.clone(), plan.clone())
            .await
            .unwrap();

        assert!(table_dir.is_dir());

        let table_location = handler.area_store.get_table_location(&table_ref).unwrap();
        let files = handler
            .area_store
            .get_location_files(&table_location)
            .await
            .unwrap();
        assert!(files.len() == 1);

        let _response = handler
            .handle_do_put(request.clone(), plan.clone())
            .await
            .unwrap();
        let files = handler
            .area_store
            .get_location_files(&table_location)
            .await
            .unwrap();
        assert!(files.len() == 2);

        let request = CommandWriteIntoDataset {
            source: Some(table_ref.clone()),
            save_mode: SaveMode::Overwrite.into(),
        };

        let _response = handler
            .handle_do_put(request.clone(), plan.clone())
            .await
            .unwrap();
        let files = handler
            .area_store
            .get_location_files(&table_location)
            .await
            .unwrap();
        assert!(files.len() == 1)
    }
}
