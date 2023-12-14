use crate::{
    v5::api::{
        BybitApi,
        get::Get,
    },
    utils::deserialize_string_to_u64,
};
use serde::Deserialize;
use serde_json::Value;
use anyhow::Result;

const PATH: &'static str = "/v5/market/time";

impl BybitApi {
    /// Retrieves the server time from the Bybit API.
    ///
    /// This method sends a GET request to the specified endpoint to fetch the server time from the Bybit server.
    /// It returns a `Result` containing the `GetBybitServerTimeResponse` if the request is successful, or an error if it fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::v5::api::BybitApi;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let response = api.get_bybit_server_time().await;
    ///     match response {
    ///         Ok(server_time) => {
    ///             println!("Server time: {}", server_time.result().time_second());
    ///         }
    ///         Err(err) => {
    ///             eprintln!("Failed to fetch server time: {:?}", err);
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn get_bybit_server_time(&self) -> Result<GetBybitServerTimeResponse> {
        self.get(PATH, None::<()>, false).await
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBybitServerTimeResponse {
    ret_code: i32,
    ret_msg: String,
    result: BybitServerTimeResult,
    ret_ext_info: Value,
    time: u64,
}


impl GetBybitServerTimeResponse {
    pub fn ret_code(&self) -> i32 {
        self.ret_code
    }

    pub fn set_ret_code(&mut self, ret_code: i32) {
        self.ret_code = ret_code;
    }

    pub fn ret_msg(&self) -> &str {
        &self.ret_msg
    }

    pub fn set_ret_msg(&mut self, ret_msg: String) {
        self.ret_msg = ret_msg;
    }

    pub fn result(&self) -> &BybitServerTimeResult {
        &self.result
    }

    pub fn set_result(&mut self, result: BybitServerTimeResult) {
        self.result = result;
    }

    pub fn ret_ext_info(&self) -> &Value {
        &self.ret_ext_info
    }

    pub fn set_ret_ext_info(&mut self, ret_ext_info: Value) {
        self.ret_ext_info = ret_ext_info;
    }

    pub fn time(&self) -> u64 {
        self.time
    }

    pub fn set_time(&mut self, time: u64) {
        self.time = time;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BybitServerTimeResult {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    time_second: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    time_nano: u64,
}
impl BybitServerTimeResult {
    pub fn time_second(&self) -> u64 {
        self.time_second
    }

    pub fn set_time_second(&mut self, time_second: u64) {
        self.time_second = time_second;
    }

    pub fn time_nano(&self) -> u64 {
        self.time_nano
    }

    pub fn set_time_nano(&mut self, time_nano: u64) {
        self.time_nano = time_nano;
    }
}
