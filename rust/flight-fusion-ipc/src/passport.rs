use crate::{
    errors::{FlightFusionError, Result},
    flight_fusion_ipc::*,
    setters,
};
use ring::{hmac, rand};
use std::sync::Arc;

pub enum ValidationResult {
    Valid,
    Invalid,
}

#[async_trait::async_trait]
pub trait KeyStore {
    async fn get_key(&self, key_name: String) -> Result<hmac::Key>;
}

pub struct StaticKeyStore {
    key: hmac::Key,
}

impl StaticKeyStore {
    pub fn new() -> Self {
        let rng = rand::SystemRandom::new();
        let key = hmac::Key::generate(hmac::HMAC_SHA256, &rng).unwrap();
        Self { key }
    }
}

#[async_trait::async_trait]
impl KeyStore for StaticKeyStore {
    async fn get_key(&self, _key_name: String) -> Result<hmac::Key> {
        Ok(self.key.clone())
    }
}

pub struct PassportHandler {
    store: Arc<dyn KeyStore>,
}

impl PassportHandler {
    pub fn new() -> Self {
        let store = Arc::new(StaticKeyStore::new());
        Self { store }
    }

    pub async fn builder(&self) -> Result<PassportBuilder> {
        let key = self.store.get_key("default".to_string()).await?;
        Ok(PassportBuilder::new(key))
    }

    pub async fn verify(&self, passport: Passport) -> Result<()> {
        if let Some(user_info) = passport.user_info {
            let buf = serialize_message(user_info)?;
            let integrity = passport.user_integrity.unwrap();
            let key = self.store.get_key(integrity.key_name).await?;
            hmac::verify(&key, &buf, &integrity.hmac)
                .map_err(|_| FlightFusionError::Generic("Validation failed".to_string()))?;
        };

        if let Some(device_info) = passport.device_info {
            let buf = serialize_message(device_info)?;
            let integrity = passport.device_integrity.unwrap();
            let key = self.store.get_key(integrity.key_name).await?;
            hmac::verify(&key, &buf, &integrity.hmac)
                .map_err(|_| FlightFusionError::Generic("Validation failed".to_string()))?;
        };

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PassportBuilder {
    key: hmac::Key,
    header: Option<Header>,
    user_info: Option<UserInfo>,
    device_info: Option<DeviceInfo>,
}

impl PassportBuilder {
    pub fn new(key: hmac::Key) -> Self {
        Self {
            key,
            header: None,
            user_info: None,
            device_info: None,
        }
    }

    setters! {
        header: Header => Some(header),
        user_info: UserInfo => Some(user_info),
        device_info: DeviceInfo => Some(device_info),
    }

    pub fn build(&self) -> Passport {
        let user_integrity = self
            .user_info
            .clone()
            .map(|info| message_integrity(info, &self.key));

        let device_integrity = self
            .device_info
            .clone()
            .map(|info| message_integrity(info, &self.key));

        Passport {
            header: self.header.clone(),
            user_info: self.user_info.clone(),
            device_info: self.device_info.clone(),
            user_integrity,
            device_integrity,
        }
    }
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

pub fn serialize_message<T: prost::Message>(msg: T) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    buf.reserve(msg.encoded_len());
    msg.encode(&mut buf)?;
    Ok(buf)
}

pub fn deserialize_message<T: prost::Message + Default>(buf: &[u8]) -> Result<T> {
    Ok(T::decode(buf)?)
}

fn message_integrity<T: prost::Message>(msg: T, hmac_key: &hmac::Key) -> Integrity {
    let mut buf = Vec::new();
    buf.reserve(msg.encoded_len());
    msg.encode(&mut buf).unwrap();

    let sig = hmac::sign(hmac_key, &buf);
    Integrity {
        version: 1,
        key_name: "default".to_string(),
        hmac: sig.as_ref().to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn passport_roundtrip() {
        let handler = PassportHandler::new();
        let builder = handler.builder().await.unwrap();

        let builder = builder.user_info(UserInfo {
            customer_id: 1,
            authentication_level: PassportAuthenticationLevel::High.into(),
            ..UserInfo::default()
        });

        let mut passport = builder.build();

        let result = handler.verify(passport.clone()).await;
        assert!(!result.is_err());

        passport.user_info = Some(UserInfo {
            customer_id: 2,
            authentication_level: PassportAuthenticationLevel::High.into(),
            ..UserInfo::default()
        });

        let result = handler.verify(passport.clone()).await;
        assert!(result.is_err());
    }
}
