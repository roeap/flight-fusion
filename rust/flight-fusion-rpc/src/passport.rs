use crate::flight_fusion_rpc;
use prost::Message;

pub trait PassportIntrospector {
    fn get_customer_id() -> String;
    fn get_account_owner_id() -> String;
    fn get_esn() -> String;
    fn get_device_type_id() -> String;
    fn get_passport_as_string() -> String;
}

pub fn passport_builder() -> flight_fusion_rpc::Passport {
    let mut passport = flight_fusion_rpc::Passport::default();
    let mut user_info = flight_fusion_rpc::UserInfo::default();

    user_info.customer_id = 1;
    user_info.authentication_level = flight_fusion_rpc::PassportAuthenticationLevel::High.into();

    passport.user_info = Some(user_info);
    passport
}

pub fn serialize_passport(passport: flight_fusion_rpc::Passport) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(passport.encoded_len());
    passport.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_passport(buf: &[u8]) -> Result<flight_fusion_rpc::Passport, prost::DecodeError> {
    flight_fusion_rpc::Passport::decode(buf)
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
        let mut action = flight_fusion_rpc::DropDatasetRequest::default();
        action.name = String::from("table");

        let mut action_request = flight_fusion_rpc::FlightActionRequest::default();
        action_request.action = Some(flight_fusion_rpc::flight_action_request::Action::Drop(
            action,
        ));

        println!("{:?}", action_request)
    }
}
