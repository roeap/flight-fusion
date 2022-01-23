use arrow_deps::datafusion::{
    arrow::datatypes::SchemaRef,
    error::Result as DataFusionResult,
    physical_plan::{
        common::{compute_record_batch_statistics, SizedRecordBatchStream},
        Distribution, ExecutionPlan, Partitioning, SendableRecordBatchStream, Statistics,
    },
};
use arrow_deps::datafusion::{
    arrow::{
        datatypes::{Schema as ArrowSchema, SchemaRef as ArrowSchemaRef},
        error::{ArrowError, Result as ArrowResult},
        record_batch::RecordBatch,
    },
    execution::runtime_env::RuntimeEnv,
    physical_plan::{
        metrics::{BaselineMetrics, ExecutionPlanMetricsSet},
        RecordBatchStream,
    },
};
use arrow_flight::{flight_descriptor::DescriptorType, FlightData};
use async_trait::async_trait;
use core::any::Any;
use flight_fusion_ipc::{
    to_flight_fusion_err, FlightDoPutRequest, FlightFusionError, Result as FusionResult,
};
use futures::{stream::Stream, StreamExt, TryStreamExt};
use prost::Message;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tonic::Streaming;

/// Execution plan for processing request streams
pub struct FlightReceiverPlan {
    /// The original Tonic stream
    inner: Vec<Arc<RecordBatch>>,
    /// SChema of streamed batches
    schema: ArrowSchemaRef,
    /// the requests object
    ticket: FlightDoPutRequest,
    /// Execution metrics
    metrics: ExecutionPlanMetricsSet,
}

impl FlightReceiverPlan {
    pub async fn try_new(mut stream: Streaming<FlightData>) -> FusionResult<Self> {
        let flight_data = stream
            .message()
            .await
            .map_err(to_flight_fusion_err)?
            .ok_or_else(|| FlightFusionError::generic("Must send some FlightData"))?;

        let descriptor = flight_data
            .flight_descriptor
            .clone()
            .ok_or_else(|| FlightFusionError::generic("Must have a descriptor"))?;

        let ticket = match DescriptorType::from_i32(descriptor.r#type) {
            Some(DescriptorType::Cmd) => {
                let request_data = FlightDoPutRequest::decode(&mut descriptor.cmd.as_ref())
                    .map_err(|e| FlightFusionError::external(e.to_string()))?;
                Ok(request_data)
            }
            Some(DescriptorType::Path) => Err(FlightFusionError::input(
                "Put operation not implemented for path",
            )),
            _ => Err(FlightFusionError::input(
                "Proper descriptor must be provided",
            )),
        }?;

        let schema =
            Arc::new(ArrowSchema::try_from(&flight_data).map_err(|e| {
                FlightFusionError::ExternalError(format!("Invalid schema: {:?}", e))
            })?);

        let to_batch = |flight_data| {
            let dictionaries_by_field = vec![None; schema.fields().len()];
            Arc::new(
                arrow_flight::utils::flight_data_to_arrow_batch(
                    &flight_data,
                    schema.clone(),
                    &dictionaries_by_field,
                )
                .unwrap(),
            )
        };

        // TODO find a way to consume this stream lazily in execute function - maybe mutex?
        let batches = stream
            .try_collect::<Vec<_>>()
            .await
            .unwrap()
            .into_iter()
            .map(to_batch)
            .collect::<Vec<_>>();

        Ok(Self {
            inner: batches,
            schema,
            ticket,
            metrics: ExecutionPlanMetricsSet::new(),
        })
    }

    pub fn ticket(&self) -> &FlightDoPutRequest {
        &self.ticket
    }
}

impl std::fmt::Debug for FlightReceiverPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FlightReceiverPlan")
    }
}

#[async_trait]
impl ExecutionPlan for FlightReceiverPlan {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn schema(&self) -> SchemaRef {
        self.schema.clone()
    }

    fn children(&self) -> Vec<Arc<dyn ExecutionPlan>> {
        vec![]
    }

    /// Get the output partitioning of this plan
    fn output_partitioning(&self) -> Partitioning {
        Partitioning::UnknownPartitioning(1)
    }

    fn required_child_distribution(&self) -> Distribution {
        // TODO
        Distribution::SinglePartition
    }

    fn with_new_children(
        &self,
        _children: Vec<Arc<dyn ExecutionPlan>>,
    ) -> DataFusionResult<Arc<dyn ExecutionPlan>> {
        todo!()
    }

    async fn execute(
        &self,
        partition: usize,
        _runtime: Arc<RuntimeEnv>,
    ) -> DataFusionResult<SendableRecordBatchStream> {
        let baseline_metrics = BaselineMetrics::new(&self.metrics, partition);
        let elapsed_compute = baseline_metrics.elapsed_compute().clone();
        let _timer = elapsed_compute.timer(); // record on drop

        let stream =
            SizedRecordBatchStream::new(self.schema(), self.inner.clone(), baseline_metrics);
        Ok(Box::pin(stream))
    }

    fn statistics(&self) -> Statistics {
        compute_record_batch_statistics(&[], &self.schema(), None)
    }
}

pub struct FlightTonicRecordBatchStream {
    /// The original Tonic stream
    inner: Streaming<FlightData>,
    /// Schema
    schema: ArrowSchemaRef,
}

impl FlightTonicRecordBatchStream {
    /// Create an empty RecordBatchStream
    pub async fn _new(schema: ArrowSchemaRef, stream: Streaming<FlightData>) -> FusionResult<Self> {
        Ok(Self {
            inner: stream,
            schema,
        })
    }
}

impl RecordBatchStream for FlightTonicRecordBatchStream {
    fn schema(&self) -> ArrowSchemaRef {
        self.schema.clone()
    }
}

impl Stream for FlightTonicRecordBatchStream {
    type Item = ArrowResult<RecordBatch>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let schema = self.schema();
        match self.get_mut().inner.poll_next_unpin(cx) {
            Poll::Ready(Some(Ok(flight_data))) => {
                let dictionaries_by_field = vec![None; schema.fields().len()];
                let batch = arrow_flight::utils::flight_data_to_arrow_batch(
                    &flight_data,
                    schema,
                    &dictionaries_by_field,
                )
                .unwrap();
                Poll::Ready(Some(Ok(batch)))
            }
            Poll::Ready(Some(Err(_))) => Poll::Ready(Some(Err(ArrowError::IoError(
                "Failed to generate batch form flight stream".to_string(),
            )))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
