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

const PATH: &'static str = "/v5/market/account-ratio";

impl BybitApi {
    /// Retrieves the long/short ratio for a specific market.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the API request.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::v5::api::{
    ///     get::market::get_long_short_ratio::{
    ///         GetLongShortRatioParameters,
    ///         GetLongShortRatioCategory
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = GetLongShortRatioParameters::new(GetLongShortRatioCategory::Linear, "BTCUSDT".to_string(), "5min".to_string());
    ///     let response = api.get_long_short_ratio(params).await;
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
    pub async fn get_long_short_ratio(&self, params: GetLongShortRatioParameters) -> Result<GetLongShortRatioResponse> {
        self.get(PATH, Some(params), false).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GetLongShortRatioCategory {
    Linear,
    Inverse,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLongShortRatioParameters {
    category: GetLongShortRatioCategory,
    symbol: String,
    period: String,
    limit: Option<u32>
}

impl GetLongShortRatioParameters {
    /// Creates a new instance of `GetLongShortRatioParameters`.
    ///
    /// # Arguments
    ///
    /// * `category` - The category of the long-short ratio.
    /// * `symbol` - The symbol of the market.
    /// * `period` - The time period for the long-short ratio.
    ///
    /// # Returns
    ///
    /// A new instance of `GetLongShortRatioParameters`.
    pub fn new(category: GetLongShortRatioCategory, symbol: String, period: String) -> Self {
        Self {
            category,
            symbol,
            period,
            limit: None,
        }
    }

    /// Sets the limit for the number of results to be returned.
    ///
    /// # Arguments
    ///
    /// * `limit` - The limit for the number of results.
    ///
    /// # Returns
    ///
    /// The modified `GetLongShortRatioParameters` instance with the limit set.
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLongShortRatioResponse {
    ret_code: i32,
    ret_msg: String,
    result: LongShortRatioResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetLongShortRatioResponse {
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

    pub fn result(&self) -> &LongShortRatioResult {
        &self.result
    }

    pub fn set_result(&mut self, result: LongShortRatioResult) {
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
pub struct LongShortRatioResult {
    list: Vec<LongShortRatio>
}
impl LongShortRatioResult {
    pub fn list(&self) -> &Vec<LongShortRatio> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<LongShortRatio>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LongShortRatio {
    symbol: String,
    #[serde(deserialize_with = "deserialize_f64")]
    buy_ratio: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    sell_ratio: f64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    timestamp: u64,
}
impl LongShortRatio {
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn buy_ratio(&self) -> f64 {
        self.buy_ratio
    }

    pub fn set_buy_ratio(&mut self, buy_ratio: f64) {
        self.buy_ratio = buy_ratio;
    }

    pub fn sell_ratio(&self) -> f64 {
        self.sell_ratio
    }

    pub fn set_sell_ratio(&mut self, sell_ratio: f64) {
        self.sell_ratio = sell_ratio;
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn set_timestamp(&mut self, timestamp: u64) {
        self.timestamp = timestamp;
    }
}