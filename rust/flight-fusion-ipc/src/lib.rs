pub mod errors;
mod macros;
#[rustfmt::skip]
mod gen {
    include!("flight_fusion.ipc.v1alpha1.rs");
}
pub mod passport;
pub mod utils;

pub use errors::*;
pub use gen::*;
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

impl RequestFor for CommandRegisterSource {
    type Reply = ResultActionStatus;
}

impl RequestFor for PutMemoryTableRequest {
    type Reply = ResultDoPutUpdate;
}

impl RequestFor for CommandWriteIntoDataset {
    type Reply = ResultDoPutUpdate;
}

impl RequestFor for DeltaOperationRequest {
    type Reply = DeltaOperationResponse;
}
