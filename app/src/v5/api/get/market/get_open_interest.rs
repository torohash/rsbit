use crate::{
    v5::api::{
        BybitApi,
        get::Get,
    },
    utils::{
        deserialize_f64,
        deserialize_string_to_u64,
    },
};

use serde::{
    Serialize,
    Deserialize,
};
use serde_json::Value;
use anyhow::Result;

const PATH: &'static str = "/v5/market/open-interest";

impl BybitApi {
    /// Retrieves the open interest data for the market.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the request.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::v5::api::{
    ///     get::market::get_open_interest::{
    ///         GetOpenInterestParameters,
    ///         GetOpenInterestCategory
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = GetOpenInterestParameters::new(GetOpenInterestCategory::Linear, "BTCUSDT".to_string(), "5min".to_string());
    ///     let response = api.get_open_interest(params).await;
    ///     match response {
    ///         Ok(info) => {
    ///             // Handle the data
    ///         },
    ///         Err(err) => {
    ///             // Handle the error
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn get_open_interest(&self, params: GetOpenInterestParameters) -> Result<GetOpenInterestResponse> {
        self.get(PATH, Some(params), false).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GetOpenInterestCategory {
    Linear,
    Inverse,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenInterestParameters {
    category: GetOpenInterestCategory,
    symbol: String,
    interval_time: String,
    start_time: Option<u64>,
    end_time: Option<u64>,
    limit: Option<u32>,
    cursor: Option<String>,
}

impl GetOpenInterestParameters {
    /// Creates a new instance of `GetOpenInterestParameters`.
    ///
    /// # Arguments
    ///
    /// * `category` - The category of the open interest.
    /// * `symbol` - The symbol of the open interest.
    /// * `interval_time` - The interval time of the open interest.
    ///
    /// # Returns
    ///
    /// A new instance of `GetOpenInterestParameters`.
    pub fn new(category: GetOpenInterestCategory, symbol: String, interval_time: String) -> Self {
        Self {
            category,
            symbol,
            interval_time,
            start_time: None,
            end_time: None,
            limit: None,
            cursor: None,
        }
    }

    /// Sets the start time for the open interest query.
    ///
    /// # Arguments
    ///
    /// * `start_time` - The start time in UNIX timestamp format.
    ///
    /// # Returns
    ///
    /// The modified `GetOpenInterestParameters` instance.
    pub fn with_start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// Sets the end time for the open interest query.
    ///
    /// # Arguments
    ///
    /// * `end_time` - The end time in UNIX timestamp format.
    ///
    /// # Returns
    ///
    /// The modified `GetOpenInterestParameters` instance.
    pub fn with_end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// Sets the limit for the open interest query.
    ///
    /// # Arguments
    ///
    /// * `limit` - The maximum number of results to return.
    ///
    /// # Returns
    ///
    /// The modified `GetOpenInterestParameters` instance.
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the cursor for the open interest query.
    ///
    /// # Arguments
    ///
    /// * `cursor` - The cursor for pagination.
    ///
    /// # Returns
    ///
    /// The modified `GetOpenInterestParameters` instance.
    pub fn with_cursor(mut self, cursor: String) -> Self {
        self.cursor = Some(cursor);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenInterestResponse {
    ret_code: i32,
    ret_msg: String,
    result: OpenInterestResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetOpenInterestResponse {
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

    pub fn result(&self) -> &OpenInterestResult {
        &self.result
    }

    pub fn set_result(&mut self, result: OpenInterestResult) {
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
pub struct OpenInterestResult {
    category: String,
    symbol: String,
    list: Vec<OpenInterest>
}
impl OpenInterestResult {
    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn set_category(&mut self, category: String) {
        self.category = category;
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn list(&self) -> &Vec<OpenInterest> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<OpenInterest>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterest {
    #[serde(deserialize_with = "deserialize_f64")]
    open_interest: f64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    timestamp: u64,
}
impl OpenInterest {
    pub fn open_interest(&self) -> f64 {
        self.open_interest
    }

    pub fn set_open_interest(&mut self, open_interest: f64) {
        self.open_interest = open_interest;
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn set_timestamp(&mut self, timestamp: u64) {
        self.timestamp = timestamp;
    }
}