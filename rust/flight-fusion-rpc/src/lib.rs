pub use flight_fusion_rpc::*;
mod flight_fusion_rpc;
pub mod passport;
pub mod errors;
pub use errors::*;

pub trait RequestFor {
    type Reply;
}

impl RequestFor for DropDatasetRequest {
    type Reply = DropDatasetResponse;
}

impl RequestFor for RegisterDatasetAction {
    type Reply = RegisterDatasetResponse;
}
