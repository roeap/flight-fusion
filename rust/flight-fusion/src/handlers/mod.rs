use crate::error::Result;
use arrow_deps::datafusion::{
    physical_plan::{ExecutionPlan, SendableRecordBatchStream},
    prelude::*,
};
use async_trait::async_trait;
use flight_fusion_ipc::RequestFor;
use std::sync::Arc;

pub mod actions;
mod delta;
pub mod do_get;
pub mod do_put;

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
    async fn handle_do_put(&self, req: T, input: Arc<dyn ExecutionPlan>) -> Result<T::Reply>;
}
