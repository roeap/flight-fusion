pub mod error;
mod macros;
#[rustfmt::skip]
mod gen {
    include!("flight_fusion.ipc.v1alpha1.rs");
}
pub mod utils;

pub use error::*;
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

impl RequestFor for CommandWriteIntoDataset {
    type Reply = ResultDoPutUpdate;
}

impl RequestFor for DeltaOperationRequest {
    type Reply = DeltaOperationResponse;
}
