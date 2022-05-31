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

impl Into<FlightDescriptor> for AreaSourceReference {
    fn into(self) -> FlightDescriptor {
        FlightDescriptor {
            r#type: DescriptorType::Cmd.into(),
            cmd: self.encode_to_vec(),
            ..FlightDescriptor::default()
        }
    }
}

impl Into<AreaSourceReference> for AreaTableLocation {
    fn into(self) -> AreaSourceReference {
        AreaSourceReference {
            table: Some(area_source_reference::Table::Location(self)),
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

impl Into<FlightDescriptor> for AreaTableLocation {
    fn into(self) -> FlightDescriptor {
        let source: AreaSourceReference = self.into();
        FlightDescriptor {
            r#type: DescriptorType::Cmd.into(),
            cmd: source.encode_to_vec(),
            ..FlightDescriptor::default()
        }
    }
}
