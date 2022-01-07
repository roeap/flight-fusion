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

impl RequestFor for DropDatasetRequest {
    type Reply = DropDatasetResponse;
}

impl RequestFor for RegisterDatasetRequest {
    type Reply = RegisterDatasetResponse;
}

impl RequestFor for PutMemoryTableRequest {
    type Reply = PutMemoryTableResponse;
}

impl RequestFor for PutRemoteTableRequest {
    type Reply = DoPutUpdateResult;
}

impl RequestFor for DeltaOperationRequest {
    type Reply = DeltaOperationResponse;
}
