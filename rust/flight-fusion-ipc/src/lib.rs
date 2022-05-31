pub mod error;
mod macros;
#[rustfmt::skip]
mod gen {
    include!("gen/flight_fusion.ipc.v1alpha1.rs");
}
pub mod utils;

use arrow_flight::{flight_descriptor::DescriptorType, FlightDescriptor};
pub use error::*;
pub use gen::*;
use prost::Message;
pub use utils::*;

pub trait RequestFor {
    type Reply;
}

impl RequestFor for CommandDropSource {
    type Reply = ResultActionStatus;
}

impl RequestFor for CommandSetMetadata {
    type Reply = ResultActionStatus;
}

impl RequestFor for CommandWriteIntoDataset {
    type Reply = ResultDoPutUpdate;
}

impl RequestFor for DeltaOperationRequest {
    type Reply = DeltaOperationResponse;
}
