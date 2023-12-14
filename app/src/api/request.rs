use crate::{
    error::ApiKeyError::{
        self,
        NotFoundApiKey
    },
    auth::Auth,
    api::BybitApi,
};
use anyhow::Result;
use reqwest::header::{
    HeaderMap,
    HeaderValue,
};
use url::{
    Url,
    ParseError,
};
use serde::{
    Serialize,
    Deserialize,
    de::DeserializeOwned,
};
use chrono::Utc;
use async_trait::async_trait;


#[async_trait]
pub trait BybitApiRequest: Auth {
    fn build_headers(&self, payload: &str) -> Result<HeaderMap>;
    fn build_url(&self, endpoint: &str, params: Option<&str>) -> Result<Url, ParseError>;
    fn require_api_key(&self) -> Result<&str, ApiKeyError>;
    async fn send_api_request<T: Serialize + Send>(&self, endpoint: &str, params: Option<T>, private: bool, request_type: RequestType) -> Result<String>;
    fn deserialize_response<P: DeserializeOwned>(&self, body: &str) -> Result<P>;
}

#[async_trait]
impl BybitApiRequest for BybitApi {
    fn build_headers(&self, payload: &str) -> Result<HeaderMap> {
        let timestamp = Utc::now().timestamp_millis().to_string();
        let recv_window = self.recv_window();
        let api_key = &self.require_api_key()?;
        
        let message = format!("{}{}{}{}", &timestamp, api_key, recv_window, payload);
        let signature = self.create_signature(&message)?;

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("X-BAPI-API-KEY", HeaderValue::from_str(api_key)?);
        headers.insert("X-BAPI-TIMESTAMP", HeaderValue::from_str(&timestamp)?);
        headers.insert("X-BAPI-RECV-WINDOW", HeaderValue::from_str(recv_window)?);
        headers.insert("X-BAPI-SIGN", HeaderValue::from_str(&signature)?);
        Ok(headers)
    }

    fn build_url(&self, endpoint: &str, query: Option<&str>) -> Result<Url, ParseError> {
        let mut url = Url::parse(self.base_url())?.join(endpoint)?;
        url.set_query(query);
        Ok(url)
    }

    fn require_api_key(&self) -> Result<&str, ApiKeyError> {
        match self.api_key() {
            Some(api_key) => Ok(api_key),
            None => Err(NotFoundApiKey)
        }
    }
    
    async fn send_api_request<T: Serialize + Send>(&self, endpoint: &str, params: Option<T>, private: bool, request_type: RequestType) -> Result<String> {

        let request = match request_type {
            RequestType::Get => {
                let query = serde_qs::to_string(&params)?;
                let url = self.build_url(endpoint, Some(&query))?;
                let client = reqwest::Client::new();
                let request = client.get(url);
                if private {
                    let headers = self.build_headers(&query)?;
                    request.headers(headers)
                } else {
                    request
                }
            },
            RequestType::Post => {
                let url = self.build_url(endpoint, None)?;
                let client = reqwest::Client::new();
                let request = client.post(url).json(&params);
                let payload = serde_json::to_string(&params)?;
                let headers = self.build_headers(&payload)?;
                request.headers(headers)
            },
        };
    
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
        let response: ApiResponse = serde_json::from_str(body)?;
        match response.ret_code {
            0 => {
                let response: P = serde_json::from_str(body)?;
                Ok(response)
            },
            _ => {
                Err(anyhow::Error::msg(format!(
                    "Response error ret_code: {}, ret_msg: {}",
                    response.ret_code,
                    response.ret_msg
                )))
            }
        }
    }
}

pub enum RequestType {
    Get,
    Post,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiResponse {
    pub ret_code: i32,
    pub ret_msg: String,
}