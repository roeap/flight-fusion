use super::{ActionHandler, FusionActionHandler};
use crate::error::{FusionServiceError, Result};
use area_store::{
    catalog::AreaCatalog,
    store::{flatten_list_stream, AreaStore},
};
use flight_fusion_ipc::{ActionStatus, CommandDropSource, CommandSetMetadata, ResultActionStatus};
use object_store::ObjectStoreApi;

#[async_trait::async_trait]
impl ActionHandler<CommandDropSource> for FusionActionHandler {
    async fn handle_do_action(&self, action: CommandDropSource) -> Result<ResultActionStatus> {
        if let Some(source) = action.source {
            // TODO remove panic
            let location = self.area_store.get_table_location(&source)?;
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
            Ok(ResultActionStatus {
                status: ActionStatus::Success.into(),
            })
        } else {
            Err(FusionServiceError::input("missing table reference"))
        }
    }
}

#[async_trait::async_trait]
impl ActionHandler<CommandSetMetadata> for FusionActionHandler {
    async fn handle_do_action(&self, action: CommandSetMetadata) -> Result<ResultActionStatus> {
        match action {
            CommandSetMetadata {
                source: Some(source),
                meta: Some(meta),
            } => {
                self.area_catalog.set_source_metadata(source, meta).await?;
                Ok(ResultActionStatus {
                    status: ActionStatus::Success.into(),
                })
            }
            _ => Err(FusionServiceError::input(
                "source and metadata must be specified",
            )),
        }
    }
}
