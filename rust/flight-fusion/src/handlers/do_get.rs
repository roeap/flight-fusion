use super::DoGetHandler;
use crate::{
    error::{FusionServiceError, Result},
    service::FlightFusionService,
};
use area_store::store::AreaStore;
use arrow_deps::datafusion::{
    physical_plan::SendableRecordBatchStream,
    prelude::{SessionConfig, SessionContext},
};
use async_trait::async_trait;
use flight_fusion_ipc::{
    command_execute_query::Context as QueryContext, CommandExecuteQuery, CommandReadDataset,
    CommandSqlOperation,
};

#[async_trait]
impl DoGetHandler<CommandSqlOperation> for FlightFusionService {
    async fn execute_do_get(
        &self,
        ticket: CommandSqlOperation,
    ) -> Result<SendableRecordBatchStream> {
        let config = SessionConfig::new().with_information_schema(true);
        let ctx = SessionContext::with_config(config);
        ctx.register_catalog("catalog", self.catalog.clone());

        // execute the query
        Ok(ctx
            .sql(ticket.query.as_str())
            .await?
            .execute_stream()
            .await?)
    }
}

#[async_trait]
impl DoGetHandler<CommandReadDataset> for FlightFusionService {
    async fn execute_do_get(
        &self,
        ticket: CommandReadDataset,
    ) -> Result<SendableRecordBatchStream> {
        if let Some(table) = ticket.source {
            let location = self.area_store.get_table_location(&table)?;
            Ok(self.area_store.open_file(&location).await?)
        } else {
            Err(FusionServiceError::InputError(
                "missing table reference".to_string(),
            ))
        }
    }
}

#[async_trait]
impl DoGetHandler<CommandExecuteQuery> for FlightFusionService {
    async fn execute_do_get(
        &self,
        ticket: CommandExecuteQuery,
    ) -> Result<SendableRecordBatchStream> {
        let mut ctx = SessionContext::new();
        match ticket.context {
            Some(QueryContext::Source(source)) => {
                self.register_source(&mut ctx, &source).await?;
            }
            Some(QueryContext::Collection(collection)) => {
                for source in collection.sources {
                    self.register_source(&mut ctx, &source).await?
                }
            }
            _ => todo!(),
        };
        Ok(ctx.sql(&ticket.query).await?.execute_stream().await?)
    }
}
