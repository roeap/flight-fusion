use crate::flight_fusion_ipc;
use prost::Message;

pub trait PassportIntrospector {
    fn get_customer_id() -> String;
    fn get_account_owner_id() -> String;
    fn get_esn() -> String;
    fn get_device_type_id() -> String;
    fn get_passport_as_string() -> String;
}

pub fn passport_builder() -> flight_fusion_ipc::Passport {
    let mut passport = flight_fusion_ipc::Passport::default();
    let mut user_info = flight_fusion_ipc::UserInfo::default();

    user_info.customer_id = 1;
    user_info.authentication_level = flight_fusion_ipc::PassportAuthenticationLevel::High.into();

    passport.user_info = Some(user_info);
    passport
}

pub fn serialize_passport(passport: flight_fusion_ipc::Passport) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(passport.encoded_len());
    passport.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_passport(buf: &[u8]) -> Result<flight_fusion_ipc::Passport, prost::DecodeError> {
    flight_fusion_ipc::Passport::decode(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passport_roundtrip() {
        let passport = passport_builder();
        let serialized = serialize_passport(passport.clone());
        let deserialized = deserialize_passport(serialized.as_slice()).unwrap();
        assert_eq!(passport, deserialized);
    }

    #[test]
    fn resolve_action() {
        let mut action = flight_fusion_ipc::DropDatasetRequest::default();
        action.name = String::from("table");

        let mut action_request = flight_fusion_ipc::FlightActionRequest::default();
        action_request.action = Some(flight_fusion_ipc::flight_action_request::Action::Drop(
            action,
        ));

        println!("{:?}", action_request)
    }
}
