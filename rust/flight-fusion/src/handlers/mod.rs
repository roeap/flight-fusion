use crate::error::Result;
use area_store::store::AreaStore;
use arrow_deps::datafusion::{physical_plan::ExecutionPlan, prelude::*};
use arrow_flight::FlightData;
use async_trait::async_trait;
use flight_fusion_ipc::RequestFor;
use futures::Stream;
pub use object_store::{path::ObjectStorePath, ObjectStoreApi};
use std::pin::Pin;
use std::sync::Arc;
use tonic::Status;

pub mod actions;
mod delta;
pub mod do_get;
pub mod do_put;
mod utils;

pub type BoxedFlightStream<T> =
    Pin<Box<dyn Stream<Item = std::result::Result<T, Status>> + Send + Sync + 'static>>;

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
    async fn handle_do_get(&self, req: T) -> Result<BoxedFlightStream<FlightData>>;
}

#[async_trait]
pub trait DoPutHandler<T>: Sync + Send
where
    T: RequestFor,
{
    async fn handle_do_put(&self, req: T, input: Arc<dyn ExecutionPlan>) -> Result<T::Reply>;
}
