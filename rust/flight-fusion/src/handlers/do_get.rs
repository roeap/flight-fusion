use super::DoGetHandler;
use crate::{
    error::{FusionServiceError, Result},
    service::FlightFusionService,
    stream::MergeStream,
};
use area_store::store::AreaStore;
use area_store::{store::DefaultAreaStore, Path};
use arrow_deps::arrow::{
    error::{ArrowError, Result as ArrowResult},
    record_batch::RecordBatch,
};
use arrow_deps::datafusion::{
    physical_plan::{common::AbortOnDropMany, SendableRecordBatchStream},
    prelude::{SessionConfig, SessionContext},
};
use async_trait::async_trait;
use flight_fusion_ipc::{
    command_execute_query::Context as QueryContext, CommandExecuteQuery, CommandReadDataset,
    CommandSqlOperation,
};
use futures::channel::mpsc;
use futures::SinkExt;
use futures::StreamExt;
use std::sync::Arc;
use tokio::task::JoinHandle;

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
            let files = self.area_store.get_location_files(&location).await?;
            let schema = self.area_store.get_schema(&table).await?;

            let (sender, receiver) = mpsc::channel::<ArrowResult<RecordBatch>>(files.len());
            let mut join_handles = Vec::with_capacity(files.len());

            for file_path in files {
                join_handles.push(spawn_execution(
                    sender.clone(),
                    self.area_store.clone(),
                    file_path,
                ));
            }

            Ok(Box::pin(MergeStream::new(
                schema,
                receiver,
                AbortOnDropMany(join_handles),
            )))

            // Ok(self.area_store.open_file(&location).await?)
        } else {
            Err(FusionServiceError::InputError(
                "missing table reference".to_string(),
            ))
        }
    }
}

pub(crate) fn spawn_execution(
    mut output: mpsc::Sender<ArrowResult<RecordBatch>>,
    area_store: Arc<DefaultAreaStore>,
    path: Path,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut stream = match area_store.open_file(&path).await {
            Err(e) => {
                // If send fails, plan being torn
                // down, no place to send the error
                let arrow_error = ArrowError::ExternalError(Box::new(e));
                output.send(Err(arrow_error)).await.ok();
                return;
            }
            Ok(stream) => stream,
        };
        while let Some(item) = stream.next().await {
            output.send(item).await.ok();
        }
    })
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
    use crate::handlers::DoGetHandler;
    use crate::handlers::DoPutHandler;
    use crate::test_utils::{get_fusion_handler, get_input_plan};
    use arrow_deps::datafusion::physical_plan::common::collect;
    use arrow_deps::datafusion::physical_plan::ExecutionPlan;
    use flight_fusion_ipc::{
        area_source_reference::Table as TableReference, AreaSourceReference, AreaTableLocation,
        CommandReadDataset, CommandWriteIntoDataset, SaveMode,
    };

    #[tokio::test]
    async fn test_put_get() {
        let root = tempfile::tempdir().unwrap();
        let plan = get_input_plan(None, false);
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
            .handle_do_put(put_request.clone(), plan.clone())
            .await
            .unwrap();

        let get_request = CommandReadDataset {
            source: Some(table_ref.clone()),
        };

        let response = handler.execute_do_get(get_request.clone()).await.unwrap();
        let data = collect(response).await.unwrap();

        assert_eq!(data[0].schema(), plan.schema())
    }
}
