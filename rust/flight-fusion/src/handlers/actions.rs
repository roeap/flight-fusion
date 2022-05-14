use super::ActionHandler;
use crate::{
    error::{FusionServiceError, Result},
    service::FlightFusionService,
};
use area_store::store::AreaStore;
use flight_fusion_ipc::{ActionStatus, CommandDropSource, CommandSetMetadata, ResultActionStatus};

#[async_trait::async_trait]
impl ActionHandler<CommandDropSource> for FlightFusionService {
    async fn handle_do_action(&self, action: CommandDropSource) -> Result<ResultActionStatus> {
        if let Some(source) = action.source {
            // TODO remove panic
            let location = self.area_store.get_table_location(&source)?;
            self.area_store.delete_location(&location).await?;
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
impl ActionHandler<CommandSetMetadata> for FlightFusionService {
    async fn handle_do_action(&self, action: CommandSetMetadata) -> Result<ResultActionStatus> {
        match action {
            CommandSetMetadata {
                source: Some(_source),
                meta: Some(_meta),
            } => {
                todo!();
            }
            _ => Err(FusionServiceError::input(
                "source and metadata must be specified",
            )),
        }
    }
}
