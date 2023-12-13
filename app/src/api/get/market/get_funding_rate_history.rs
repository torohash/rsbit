use crate::{
    api::{
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

const PATH: &'static str = "/v5/market/funding/history";

impl BybitApi {
    /// Retrieves the funding rate history from the Bybit API.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the API request.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::api::{
    ///     get::market::get_funding_rate_history::{
    ///         GetFundingRateHistoryParameters,
    ///         GetFundingRateHistoryCategory
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = GetFundingRateHistoryParameters::new(GetFundingRateHistoryCategory::Linear, "BTCUSDT".to_string());
    ///     let response = api.get_funding_rate_history(params).await;
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
    pub async fn get_funding_rate_history(&self, params: GetFundingRateHistoryParameters) -> Result<GetFundingRateHistoryResponse> {
        self.get(PATH, Some(params), false).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GetFundingRateHistoryCategory {
    Linear,
    Inverse,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetFundingRateHistoryParameters {
    category: GetFundingRateHistoryCategory,
    symbol: String,
    start_time: Option<u64>,
    end_time: Option<u64>,
    limit: Option<u32>,
}

impl GetFundingRateHistoryParameters {
    /// Creates a new instance of `GetFundingRateHistoryParameters`.
    ///
    /// # Arguments
    ///
    /// * `category` - The category of the funding rate history.
    /// * `symbol` - The symbol for which the funding rate history is requested.
    ///
    /// # Returns
    ///
    /// A new instance of `GetFundingRateHistoryParameters`.
    pub fn new(category: GetFundingRateHistoryCategory, symbol: String) -> Self {
        Self {
            category,
            symbol,
            start_time: None,
            end_time: None,
            limit: None,
        }
    }

    /// Sets the start time for the funding rate history.
    ///
    /// # Arguments
    ///
    /// * `start_time` - The start time in UNIX timestamp format.
    ///
    /// # Returns
    ///
    /// The updated `GetFundingRateHistoryParameters` instance.
    pub fn with_start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// Sets the end time for the funding rate history.
    ///
    /// # Arguments
    ///
    /// * `end_time` - The end time in UNIX timestamp format.
    ///
    /// # Returns
    ///
    /// The updated `GetFundingRateHistoryParameters` instance.
    pub fn with_end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// Sets the limit for the number of funding rate history records to retrieve.
    ///
    /// # Arguments
    ///
    /// * `limit` - The maximum number of records to retrieve.
    ///
    /// # Returns
    ///
    /// The updated `GetFundingRateHistoryParameters` instance.
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateHistoryResponse {
    ret_code: i32,
    ret_msg: String,
    result: FundingRateHistoryResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetFundingRateHistoryResponse {
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

    pub fn result(&self) -> &FundingRateHistoryResult {
        &self.result
    }

    pub fn set_result(&mut self, result: FundingRateHistoryResult) {
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
pub struct FundingRateHistoryResult {
    category: String,
    list: Vec<FundingRateHistory>
}
impl FundingRateHistoryResult {
    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn set_category(&mut self, category: String) {
        self.category = category;
    }

    pub fn list(&self) -> &Vec<FundingRateHistory> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<FundingRateHistory>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateHistory {
    symbol: String,
    #[serde(deserialize_with = "deserialize_f64")]
    funding_rate: f64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    funding_rate_timestamp: u64,
}
impl FundingRateHistory {
    pub fn symbol(&self) -> &String {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn funding_rate(&self) -> f64 {
        self.funding_rate
    }

    pub fn set_funding_rate(&mut self, funding_rate: f64) {
        self.funding_rate = funding_rate;
    }

    pub fn funding_rate_timestamp(&self) -> u64 {
        self.funding_rate_timestamp
    }

    pub fn set_funding_rate_timestamp(&mut self, funding_rate_timestamp: u64) {
        self.funding_rate_timestamp = funding_rate_timestamp;
    }
}