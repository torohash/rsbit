pub mod market;
use crate::api::{
    request::BybitApiRequest,
    BybitApi,
};
use anyhow::Result;
use async_trait::async_trait;
use serde::{
    Serialize,
    Deserialize,
    de::DeserializeOwned,
};

#[async_trait]
trait Get: BybitApiRequest {
    async fn get<T: Serialize + Send, P: DeserializeOwned>(&self, endpoint: &str, params: Option<T>, private: bool) -> Result<P>;
    async fn send_api_request<T: Serialize + Send>(&self, endpoint: &str, params: Option<T>, private: bool) -> Result<String>;
    fn deserialize_response<P: DeserializeOwned>(&self, body: &str) -> Result<P>;
}

#[async_trait]
impl Get for BybitApi {
    async fn get<T: Serialize + Send, P: DeserializeOwned>(&self, endpoint: &str, params: Option<T>, private: bool) -> Result<P> {
        let body = self.send_api_request(endpoint, params, private).await?;
        self.deserialize_response(&body)
    }
    
    async fn send_api_request<T: Serialize + Send>(&self, endpoint: &str, params: Option<T>, private: bool) -> Result<String> {
        let query = serde_qs::to_string(&params)?;
        let url = self.build_url(endpoint, &query)?;
        let client = reqwest::Client::new();
        let mut request = client.get(url);
    
        if private {
            let headers = self.build_headers(&query)?;
            request = request.headers(headers);
        }
    
        let response = request.send().await?;
        match response.status().as_u16() {
            200 => {
                let body = response.text().await?;
                Ok(body)
            },
            _ => {
                Err(anyhow::Error::msg(format!(
                    "Request error status code {}",
                    response.status().as_u16(),
                )))
            },
        }
    }

    fn deserialize_response<P: DeserializeOwned>(&self, body: &str) -> Result<P> {
        let get_response: GetResponse = serde_json::from_str(body)?;
        match get_response.ret_code {
            0 => {
                let response: P = serde_json::from_str(body)?;
                Ok(response)
            },
            _ => {
                Err(anyhow::Error::msg(format!(
                    "Response error ret_code: {}, ret_msg: {}",
                    get_response.ret_code,
                    get_response.ret_msg
                )))
            }
        }
    }
    
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GetResponse {
    pub ret_code: i32,
    pub ret_msg: String,
}