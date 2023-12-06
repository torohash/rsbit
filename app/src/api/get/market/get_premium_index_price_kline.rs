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

const PATH: &'static str = "/v5/market/premium-index-price-kline";

impl BybitApi {
    /// Retrieves the market's premium index price kline.
    ///
    /// This method asynchronously fetches the premium index price kline for the specified parameters.
    /// It returns a `Result` containing the `GetPremiumIndexPriceKlineResponse` if successful.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for fetching the index price kline.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::api::{
    ///     get::market::get_premium_index_price_kline::{
    ///         GetPremiumIndexPriceKlineParameters,
    ///         GetPremiumIndexPriceKlineCategory
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = GetPremiumIndexPriceKlineParameters::new(GetPremiumIndexPriceKlineCategory::Linear, "BTCUSDT".to_string(), "1".to_string());
    ///     let response = api.get_premium_index_price_kline(params).await;
    ///     match response {
    ///         Ok(info) => {
    ///             // Handle the premium index price kline data
    ///         },
    ///         Err(err) => {
    ///             // Handle the error
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn get_premium_index_price_kline(&self, params: GetPremiumIndexPriceKlineParameters) -> Result<GetPremiumIndexPriceKlineResponse> {
        self.get(PATH, Some(params), false).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GetPremiumIndexPriceKlineCategory {
    Linear,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetPremiumIndexPriceKlineParameters {
    category: GetPremiumIndexPriceKlineCategory,
    symbol: String,
    interval: String,
    start: Option<u64>,
    end: Option<u64>,
    limit: Option<u32>,
}

impl GetPremiumIndexPriceKlineParameters {
    /// Creates a new instance of `GetPremiumIndexPriceKlineParameters` with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `category` - The category of the premium index price kline.
    /// * `symbol` - The symbol of the premium index price kline.
    /// * `interval` - The interval of the premium index price kline.
    ///
    /// # Returns
    ///
    /// A new instance of `GetPremiumIndexPriceKlineParameters`.
    pub fn new(category: GetPremiumIndexPriceKlineCategory, symbol: String, interval: String) -> Self {
        Self {
            category,
            symbol,
            interval,
            start: None,
            end: None,
            limit: None,
        }
    }

    /// Sets the start time for the premium index price kline data.
    ///
    /// # Arguments
    ///
    /// * `start` - The start time in milliseconds.
    ///
    /// # Returns
    ///
    /// The modified GetPremiumIndexPriceKlineParameters instance.
    pub fn with_start(mut self, start: u64) -> Self {
        self.start = Some(start);
        self
    }

    /// Sets the end time for the premium index price kline data.
    ///
    /// # Arguments
    ///
    /// * `end` - The end time in milliseconds.
    ///
    /// # Returns
    ///
    /// The modified GetPremiumIndexPriceKlineParameters instance.
    pub fn with_end(mut self, end: u64) -> Self {
        self.end = Some(end);
        self
    }

    /// Sets the limit for the number of premium index price kline data to retrieve.
    ///
    /// # Arguments
    ///
    /// * `limit` - The limit of premium index price kline data.
    ///
    /// # Returns
    ///
    /// The modified GetPremiumIndexPriceKlineParameters instance.
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPremiumIndexPriceKlineResponse {
    ret_code: i32,
    ret_msg: String,
    result: PremiumIndexPriceKlineResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetPremiumIndexPriceKlineResponse {
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

    pub fn result(&self) -> &PremiumIndexPriceKlineResult {
        &self.result
    }

    pub fn set_result(&mut self, result: PremiumIndexPriceKlineResult) {
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
pub struct PremiumIndexPriceKlineResult {
    symbol: String,
    category: String,
    list: Vec<PremiumIndexPriceKline>
}
impl PremiumIndexPriceKlineResult {
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn set_category(&mut self, category: String) {
        self.category = category;
    }

    pub fn list(&self) -> &Vec<PremiumIndexPriceKline> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<PremiumIndexPriceKline>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PremiumIndexPriceKline {
    #[serde(rename = "0", deserialize_with = "deserialize_string_to_u64")]
    timestamp: u64,
    #[serde(rename = "1", deserialize_with = "deserialize_f64")]
    open: f64,
    #[serde(rename = "2", deserialize_with = "deserialize_f64")]
    high: f64,
    #[serde(rename = "3", deserialize_with = "deserialize_f64")]
    low: f64,
    #[serde(rename = "4", deserialize_with = "deserialize_f64")]
    close: f64,
}
impl PremiumIndexPriceKline {
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn set_timestamp(&mut self, timestamp: u64) {
        self.timestamp = timestamp;
    }

    pub fn open(&self) -> f64 {
        self.open
    }

    pub fn set_open(&mut self, open: f64) {
        self.open = open;
    }

    pub fn high(&self) -> f64 {
        self.high
    }

    pub fn set_high(&mut self, high: f64) {
        self.high = high;
    }

    pub fn low(&self) -> f64 {
        self.low
    }

    pub fn set_low(&mut self, low: f64) {
        self.low = low;
    }

    pub fn close(&self) -> f64 {
        self.close
    }

    pub fn set_close(&mut self, close: f64) {
        self.close = close;
    }
}