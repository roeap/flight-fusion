use super::DoGetHandler;
use crate::{error::Result, service::FlightFusionService};
use area_store::store::AreaPath;
use arrow_deps::datafusion::{physical_plan::SendableRecordBatchStream, prelude::SessionContext};
use async_trait::async_trait;
use flight_fusion_ipc::{
    area_source_reference::Table, command_execute_query::Context as QueryContext,
    AreaSourceReference, CommandExecuteQuery,
};

impl FlightFusionService {
    pub async fn register_source(
        &self,
        ctx: &mut SessionContext,
        source: &AreaSourceReference,
    ) -> crate::error::Result<()> {
        let location: AreaPath = source.clone().into();
        let table_provider = self.area_store.table_provider(&location).await?;
        let name = match &source {
            AreaSourceReference {
                table: Some(Table::Location(tbl)),
            } => tbl.name.clone(),
            _ => todo!(),
        };
        ctx.register_table(&*name, table_provider)?;
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::DoGetHandler;
    use crate::handlers::DoPutHandler;
    use crate::test_utils::{get_fusion_handler, get_input_stream};
    use arrow_deps::datafusion::physical_plan::common::collect;
    use flight_fusion_ipc::{
        area_source_reference::Table as TableReference, command_execute_query::Context,
        AreaSourceReference, AreaTableLocation, CommandWriteIntoDataset, SaveMode,
        SourceCollection,
    };

    #[tokio::test]
    async fn test_execute_query() {
        let root = tempfile::tempdir().unwrap();
        let plan = get_input_stream(None, false);
        let ref_schema = plan.schema().clone();
        let handler = get_fusion_handler(root.path());
        // let table_dir = root.path().join("_ff_data/new_table");

        let table_ref = AreaSourceReference {
            table: Some(TableReference::Location(AreaTableLocation {
                name: "new_table".to_string(),
                areas: vec![],
            })),
        };

        let put_request = CommandWriteIntoDataset {
            source: Some(table_ref.clone()),
            save_mode: SaveMode::Append.into(),
        };

        let _ = handler
            .handle_do_put(put_request.clone(), plan)
            .await
            .unwrap();

        let get_request = CommandExecuteQuery {
            query: "SELECT id, modified FROM new_table".to_string(),
            context: Some(Context::Source(table_ref.clone())),
            ..CommandExecuteQuery::default()
        };

        let response = handler.execute_do_get(get_request.clone()).await.unwrap();
        let data = collect(response).await.unwrap();

        assert_eq!(
            data[0].schema(),
            std::sync::Arc::new(ref_schema.project(&[0, 2]).unwrap())
        )
    }

    #[tokio::test]
    async fn test_execute_query_collection() {
        let root = tempfile::tempdir().unwrap();
        let plan = get_input_stream(None, false);
        let plan2 = get_input_stream(None, false);
        let ref_schema = plan.schema().clone();
        let handler = get_fusion_handler(root.path());
        // let table_dir = root.path().join("_ff_data/new_table");

        let table_ref_1 = AreaSourceReference {
            table: Some(TableReference::Location(AreaTableLocation {
                name: "table_1".to_string(),
                areas: vec![],
            })),
        };
        let table_ref_2 = AreaSourceReference {
            table: Some(TableReference::Location(AreaTableLocation {
                name: "table_2".to_string(),
                areas: vec![],
            })),
        };

        let put_request_1 = CommandWriteIntoDataset {
            source: Some(table_ref_1.clone()),
            save_mode: SaveMode::Append.into(),
        };
        let put_request_2 = CommandWriteIntoDataset {
            source: Some(table_ref_2.clone()),
            save_mode: SaveMode::Append.into(),
        };

        let _ = handler
            .handle_do_put(put_request_1.clone(), plan)
            .await
            .unwrap();
        let _ = handler
            .handle_do_put(put_request_2.clone(), plan2)
            .await
            .unwrap();

        let get_request = CommandExecuteQuery {
            query: "SELECT * FROM table_1 UNION ALL SELECT * FROM table_2".to_string(),
            context: Some(Context::Collection(SourceCollection {
                sources: vec![table_ref_1.clone(), table_ref_2.clone()],
            })),
            ..CommandExecuteQuery::default()
        };

        let response = handler.execute_do_get(get_request.clone()).await.unwrap();
        let data = collect(response).await.unwrap();

        assert_eq!(data[0].schema(), ref_schema)
    }
}
