use super::ActionHandler;
use crate::{
    error::{FusionServiceError, Result},
    service::FlightFusionService,
};
use area_store::store::AreaPath;
use flight_fusion_ipc::{ActionStatus, CommandDropSource, ResultActionStatus};

#[async_trait::async_trait]
impl ActionHandler<CommandDropSource> for FlightFusionService {
    async fn handle_do_action(&self, action: CommandDropSource) -> Result<ResultActionStatus> {
        if let Some(source) = action.source {
            let location = AreaPath::from(source);
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
