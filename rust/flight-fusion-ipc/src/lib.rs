pub mod error;
mod macros;
#[rustfmt::skip]
pub mod gen {
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

impl RequestFor for CommandWriteIntoDataset {
    type Reply = ResultDoPutUpdate;
}

impl RequestFor for DeltaOperationRequest {
    type Reply = DeltaOperationResponse;
}

impl From<AreaSourceReference> for FlightDescriptor {
    fn from(value: AreaSourceReference) -> FlightDescriptor {
        FlightDescriptor {
            r#type: DescriptorType::Cmd.into(),
            cmd: value.encode_to_vec(),
            ..FlightDescriptor::default()
        }
    }
}

impl From<AreaTableLocation> for AreaSourceReference {
    fn from(value: AreaTableLocation) -> AreaSourceReference {
        AreaSourceReference {
            table: Some(area_source_reference::Table::Location(value)),
        }
    }
}

impl TryFrom<FlightDescriptor> for AreaSourceReference {
    type Error = prost::DecodeError;

    fn try_from(value: FlightDescriptor) -> std::result::Result<Self, Self::Error> {
        match DescriptorType::from_i32(value.r#type) {
            Some(DescriptorType::Cmd) => {
                let request_data = AreaSourceReference::decode(&mut value.cmd.as_ref())?;
                Ok(request_data)
            }
            _ => Err(prost::DecodeError::new(
                "no `cmd` message found in descriptor".to_string(),
            )),
        }
    }
}

impl From<AreaTableLocation> for FlightDescriptor {
    fn from(value: AreaTableLocation) -> FlightDescriptor {
        let source: AreaSourceReference = value.into();
        FlightDescriptor {
            r#type: DescriptorType::Cmd.into(),
            cmd: source.encode_to_vec(),
            ..FlightDescriptor::default()
        }
    }
}
