pub use flight_fusion_ipc::*;
pub mod errors;
mod flight_fusion_ipc;
pub mod passport;
pub use errors::*;
pub mod utils;
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
