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
use chrono::Utc;


pub trait BybitApiRequest: Auth {
    fn build_headers(&self, payload: &str) -> Result<HeaderMap>;
    fn build_url(&self, endpoint: &str, params: &str) -> Result<Url, ParseError>;
    fn require_api_key(&self) -> Result<&str, ApiKeyError>;
}

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

    fn build_url(&self, endpoint: &str, query: &str) -> Result<Url, ParseError> {
        let mut url = Url::parse(self.base_url())?.join(endpoint)?;
        url.set_query(Some(&query));
        Ok(url)
    }

    fn require_api_key(&self) -> Result<&str, ApiKeyError> {
        match self.api_key() {
            Some(api_key) => Ok(api_key),
            None => Err(NotFoundApiKey)
        }
    }
}