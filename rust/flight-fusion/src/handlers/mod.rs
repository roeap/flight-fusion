use crate::error::Result;
use arrow_deps::datafusion::physical_plan::SendableRecordBatchStream;
use async_trait::async_trait;
use flight_fusion_ipc::RequestFor;

pub mod actions;
pub mod dataset;
mod delta;
pub mod query;

#[async_trait]
pub trait ActionHandler<T>: Sync + Send
where
    T: RequestFor,
{
    async fn handle_do_action(&self, req: T) -> Result<T::Reply>;
}

#[async_trait]
pub trait DoGetHandler<T>: Sync + Send
where
    T: prost::Message,
{
    async fn execute_do_get(&self, req: T) -> Result<SendableRecordBatchStream>;
}

#[async_trait]
pub trait DoPutHandler<T>: Sync + Send
where
    T: RequestFor,
{
    async fn handle_do_put(&self, req: T, input: SendableRecordBatchStream) -> Result<T::Reply>;
}
