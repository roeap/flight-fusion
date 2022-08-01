use super::{DoGetHandler, DoPutHandler};
use crate::{
    error::{FusionServiceError, Result},
    service::FlightFusionService,
    stream::MergeStream,
};
use area_store::store::{AreaPath, AreaStore};
use area_store::Path;
use arrow_deps::arrow::{
    error::{ArrowError, Result as ArrowResult},
    record_batch::RecordBatch,
};
use arrow_deps::datafusion::physical_plan::{
    common::{collect, AbortOnDropMany},
    SendableRecordBatchStream,
};
use async_trait::async_trait;
use flight_fusion_ipc::{CommandReadDataset, CommandWriteIntoDataset, ResultDoPutUpdate, SaveMode};
use futures::channel::mpsc;
use futures::SinkExt;
use futures::StreamExt;
use std::sync::Arc;
use tokio::task::JoinHandle;

#[async_trait]
impl DoPutHandler<CommandWriteIntoDataset> for FlightFusionService {
    async fn handle_do_put(
        &self,
        ticket: CommandWriteIntoDataset,
        input: SendableRecordBatchStream,
    ) -> Result<ResultDoPutUpdate> {
        if let Some(source) = ticket.source {
            let location: AreaPath = source.into();
            let batches = collect(input).await?;
            let _adds = self
                .area_store
                .put_batches(
                    batches,
                    &location.into(),
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

#[async_trait]
impl DoGetHandler<CommandReadDataset> for FlightFusionService {
    async fn execute_do_get(
        &self,
        ticket: CommandReadDataset,
    ) -> Result<SendableRecordBatchStream> {
        if let Some(table) = ticket.source {
            let location: AreaPath = table.into();
            let files = self
                .area_store
                .get_location_files(&location.clone())
                .await?;
            let schema = self.area_store.get_schema(&location.into()).await?;

            let column_indices = if ticket.column_names.is_empty() {
                None
            } else {
                Some(
                    ticket
                        .column_names
                        .iter()
                        .map(|c| schema.index_of(c))
                        .collect::<std::result::Result<Vec<_>, _>>()?,
                )
            };

            let (sender, receiver) = mpsc::channel::<ArrowResult<RecordBatch>>(files.len());
            let mut join_handles = Vec::with_capacity(files.len());

            for file_path in files {
                join_handles.push(spawn_execution(
                    sender.clone(),
                    self.area_store.clone(),
                    file_path,
                    column_indices.clone(),
                ));
            }

            let projected_schema = match column_indices {
                Some(indices) => Arc::new(schema.project(&indices)?),
                None => schema,
            };

            Ok(Box::pin(MergeStream::new(
                projected_schema,
                receiver,
                AbortOnDropMany(join_handles),
            )))
        } else {
            Err(FusionServiceError::InputError(
                "missing table reference".to_string(),
            ))
        }
    }
}

pub(crate) fn spawn_execution(
    mut output: mpsc::Sender<ArrowResult<RecordBatch>>,
    area_store: Arc<AreaStore>,
    path: Path,
    column_indices: Option<Vec<usize>>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut stream = match area_store.open_file(&path.into(), column_indices).await {
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

#[cfg(test)]
mod tests {
    use crate::handlers::DoGetHandler;
    use crate::handlers::DoPutHandler;
    use crate::test_utils::{get_fusion_handler, get_input_stream};
    use area_store::store::AreaPath;
    use arrow_deps::datafusion::physical_plan::common::collect;
    use flight_fusion_ipc::{
        area_source_reference::Table as TableReference, AreaSourceReference, AreaTableLocation,
        CommandReadDataset, CommandWriteIntoDataset, SaveMode,
    };

    #[tokio::test]
    async fn test_put_table() {
        let root = tempfile::tempdir().unwrap();
        let plan = get_input_stream(None, false);
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

        let _response = handler.handle_do_put(request.clone(), plan).await.unwrap();

        assert!(table_dir.is_dir())
    }

    #[tokio::test]
    async fn test_put_table_append_overwrite() {
        let root = tempfile::tempdir().unwrap();
        let plan = get_input_stream(None, false);
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

        let _response = handler.handle_do_put(request.clone(), plan).await.unwrap();

        assert!(table_dir.is_dir());

        let table_location: AreaPath = table_ref.clone().into();
        let files = handler
            .area_store
            .get_location_files(&table_location.clone())
            .await
            .unwrap();
        assert!(files.len() == 1);

        let plan = get_input_stream(None, false);
        let _response = handler.handle_do_put(request.clone(), plan).await.unwrap();
        let files = handler
            .area_store
            .get_location_files(&table_location.clone())
            .await
            .unwrap();
        assert!(files.len() == 2);

        let request = CommandWriteIntoDataset {
            source: Some(table_ref.clone()),
            save_mode: SaveMode::Overwrite.into(),
        };

        let plan = get_input_stream(None, false);
        let _response = handler.handle_do_put(request.clone(), plan).await.unwrap();
        let files = handler
            .area_store
            .get_location_files(&table_location.clone())
            .await
            .unwrap();
        assert!(files.len() == 1)
    }

    #[tokio::test]
    async fn test_put_get() {
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

        let get_request = CommandReadDataset {
            source: Some(table_ref.clone()),
            ..CommandReadDataset::default()
        };

        let response = handler.execute_do_get(get_request.clone()).await.unwrap();
        let data = collect(response).await.unwrap();

        assert_eq!(data[0].schema(), ref_schema)
    }

    #[tokio::test]
    async fn test_get_columns() {
        let root = tempfile::tempdir().unwrap();
        let plan = get_input_stream(None, false);
        let ref_schema = plan.schema().clone();
        let handler = get_fusion_handler(root.path());

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

        let cols = ref_schema
            .fields()
            .iter()
            .map(|c| c.name().clone())
            .collect::<Vec<_>>();

        let get_request = CommandReadDataset {
            source: Some(table_ref.clone()),
            column_names: vec![cols[0].clone()],
        };

        let response = handler.execute_do_get(get_request.clone()).await.unwrap();
        let data = collect(response).await.unwrap();

        assert_eq!(
            data[0].schema(),
            std::sync::Arc::new(ref_schema.project(&[0]).unwrap())
        )
    }
}
