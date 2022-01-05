pub mod errors;
#[rustfmt::skip]
mod flight_fusion_ipc;
mod macros;
pub mod passport;
pub mod utils;

pub use crate::flight_fusion_ipc::*;
pub use errors::*;
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
    type Reply = PutRemoteTableResponse;
}

impl RequestFor for DeltaOperationRequest {
    type Reply = DeltaOperationResponse;
}
