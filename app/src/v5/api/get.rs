pub mod market;
pub mod trade;
pub mod position;
pub mod account;
use crate::v5::api::{
    request::{
        BybitApiRequest,
        RequestType
    },
    BybitApi,
};
use anyhow::Result;
use async_trait::async_trait;
use serde::{
    Serialize,
    de::DeserializeOwned,
};

#[async_trait]
trait Get: BybitApiRequest {
    async fn get<T: Serialize + Send, P: DeserializeOwned>(&self, endpoint: &str, params: Option<T>, private: bool) -> Result<P>;
}

#[async_trait]
impl Get for BybitApi {
    async fn get<T: Serialize + Send, P: DeserializeOwned>(&self, endpoint: &str, params: Option<T>, private: bool) -> Result<P> {
        let body = self.send_api_request(endpoint, params, private, RequestType::Get).await?;
        self.deserialize_response(&body)
    }
}


