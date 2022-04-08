use super::{utils::create_response_stream, *};
use crate::error::{FusionServiceError, Result};
use area_store::store::AreaStore;
use arrow_deps::datafusion::prelude::{SessionConfig, SessionContext};
use arrow_flight::FlightData;
use async_trait::async_trait;
use flight_fusion_ipc::{
    command_execute_query::Context as QueryContext, CommandExecuteQuery, CommandReadDataset,
    CommandSqlOperation,
};

#[async_trait]
impl DoGetHandler<CommandSqlOperation> for FusionActionHandler {
    async fn handle_do_get(
        &self,
        ticket: CommandSqlOperation,
    ) -> Result<BoxedFlightStream<FlightData>> {
        let config = SessionConfig::new().with_information_schema(true);
        let ctx = SessionContext::with_config(config);
        ctx.register_catalog("catalog", self.catalog.clone());

        // execute the query
        let df = ctx.sql(ticket.query.as_str()).await?;
        let results = df.collect().await?;
        create_response_stream(results).await
    }
}

#[async_trait]
impl DoGetHandler<CommandReadDataset> for FusionActionHandler {
    async fn handle_do_get(
        &self,
        ticket: CommandReadDataset,
    ) -> Result<BoxedFlightStream<FlightData>> {
        if let Some(table) = ticket.source {
            let location = self.area_store.get_table_location(&table)?;
            let batches = self.area_store.get_batches(&location).await?;
            create_response_stream(batches).await
        } else {
            Err(FusionServiceError::InputError(
                "missing table reference".to_string(),
            ))
        }
    }
}

#[async_trait]
impl DoGetHandler<CommandExecuteQuery> for FusionActionHandler {
    async fn handle_do_get(
        &self,
        ticket: CommandExecuteQuery,
    ) -> Result<BoxedFlightStream<FlightData>> {
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
        create_response_stream(ctx.sql(&ticket.query).await?.collect().await?).await
    }
}
