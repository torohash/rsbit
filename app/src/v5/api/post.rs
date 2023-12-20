pub mod trade;
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
trait Post: BybitApiRequest {
    async fn post<T: Serialize + Send, P: DeserializeOwned>(&self, endpoint: &str, params: Option<T>) -> Result<P>;
}

#[async_trait]
impl Post for BybitApi {
    async fn post<T: Serialize + Send, P: DeserializeOwned>(&self, endpoint: &str, params: Option<T>) -> Result<P> {
        let body = self.send_api_request(endpoint, params, true, RequestType::Post).await?;
        self.deserialize_response(&body)
    }
}