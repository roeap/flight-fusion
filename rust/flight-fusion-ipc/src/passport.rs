use crate::flight_fusion_ipc::*;
use prost::Message;

pub trait PassportIntrospector {
    fn get_customer_id() -> String;
    fn get_account_owner_id() -> String;
    fn get_esn() -> String;
    fn get_device_type_id() -> String;
    fn get_passport_as_string() -> String;
}

pub fn passport_builder() -> Passport {
    Passport {
        user_info: Some(UserInfo {
            customer_id: 1,
            authentication_level: PassportAuthenticationLevel::High.into(),
            ..UserInfo::default()
        }),
        ..Passport::default()
    }
}

pub fn serialize_passport(passport: Passport) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(passport.encoded_len());
    passport.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_passport(buf: &[u8]) -> Result<Passport, prost::DecodeError> {
    Passport::decode(buf)
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
}
