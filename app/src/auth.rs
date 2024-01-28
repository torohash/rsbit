use crate::{
    error::ApiKeyError::{
        self,
        NotFoundApiSecret,
    },
    v5::api::BybitApi,
    v5::ws::BybitWS,
};

use ring::hmac::{
    Key,
    HMAC_SHA256,
    self,
};
use anyhow::Result;

pub trait Auth {
    fn require_api_secret(&self) -> Result<&str, ApiKeyError>;
    fn create_signature(&self, message: &str) -> Result<String>;
}

impl Auth for BybitApi {
    fn require_api_secret(&self) -> Result<&str, ApiKeyError> {
        match self.api_secret() {
            Some(api_secret) => Ok(api_secret),
            None => Err(NotFoundApiSecret)
        }
    }
    fn create_signature(&self, message: &str) -> Result<String> {
        let api_secret = self.require_api_secret()?;
        let key = Key::new(HMAC_SHA256, api_secret.as_bytes());
        let signature = hmac::sign(&key, message.as_bytes());
        Ok(hex::encode(signature.as_ref()))
    }
}

impl Auth for BybitWS {
    fn require_api_secret(&self) -> Result<&str, ApiKeyError> {
        match self.api_secret() {
            Some(api_secret) => Ok(api_secret),
            None => Err(NotFoundApiSecret)
        }
    }
    fn create_signature(&self, message: &str) -> Result<String> {
        let api_secret = self.require_api_secret()?;
        let key = hmac::Key::new(hmac::HMAC_SHA256, api_secret.as_bytes());
        let signature = hmac::sign(&key, message.as_bytes());
        Ok(hex::encode(signature.as_ref()))
    }
}