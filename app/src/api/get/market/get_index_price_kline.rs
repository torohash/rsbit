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

const PATH: &'static str = "/v5/market/index-price-kline";

impl BybitApi {
    /// Retrieves the market's index price kline.
    ///
    /// This method asynchronously fetches the index price kline for the specified parameters.
    /// It returns a `Result` containing the `GetIndexPriceKlineResponse` if successful.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for fetching the index price kline.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::api::{
    ///     get::market::get_index_price_kline::{
    ///         GetIndexPriceKlineParameters,
    ///         GetIndexPriceKlineCategory
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = GetIndexPriceKlineParameters::new(GetIndexPriceKlineCategory::Linear, "BTCUSDT".to_string(), "1".to_string());
    ///     let response = api.get_index_price_kline(params).await;
    ///     match response {
    ///         Ok(info) => {
    ///             // Handle the index price kline data
    ///         },
    ///         Err(err) => {
    ///             // Handle the error
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn get_index_price_kline(&self, params: GetIndexPriceKlineParameters) -> Result<GetIndexPriceKlineResponse> {
        self.get(PATH, Some(params), false).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GetIndexPriceKlineCategory {
    Linear,
    Inverse,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetIndexPriceKlineParameters {
    category: GetIndexPriceKlineCategory,
    symbol: String,
    interval: String,
    start: Option<u64>,
    end: Option<u64>,
    limit: Option<u32>,
}

impl GetIndexPriceKlineParameters {
    /// Creates a new instance of `GetIndexPriceKlineParameters` with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `category` - The category of the index price kline.
    /// * `symbol` - The symbol of the index price kline.
    /// * `interval` - The interval of the index price kline.
    ///
    /// # Returns
    ///
    /// A new instance of `GetIndexPriceKlineParameters`.
    pub fn new(category: GetIndexPriceKlineCategory, symbol: String, interval: String) -> Self {
        Self {
            category,
            symbol,
            interval,
            start: None,
            end: None,
            limit: None,
        }
    }

    /// Sets the start time for the kline data.
    ///
    /// # Arguments
    ///
    /// * `start` - The start time in milliseconds.
    ///
    /// # Returns
    ///
    /// The modified GetIndexPriceKlineParameters instance.
    pub fn with_start(mut self, start: u64) -> Self {
        self.start = Some(start);
        self
    }

    /// Sets the end time for the kline data.
    ///
    /// # Arguments
    ///
    /// * `end` - The end time in milliseconds.
    ///
    /// # Returns
    ///
    /// The modified GetIndexPriceKlineParameters instance.
    pub fn with_end(mut self, end: u64) -> Self {
        self.end = Some(end);
        self
    }

    /// Sets the limit for the number of kline data to retrieve.
    ///
    /// # Arguments
    ///
    /// * `limit` - The limit of kline data.
    ///
    /// # Returns
    ///
    /// The modified GetIndexPriceKlineParameters instance.
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIndexPriceKlineResponse {
    ret_code: i32,
    ret_msg: String,
    result: GetIndexPriceKlineResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetIndexPriceKlineResponse {
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

    pub fn result(&self) -> &GetIndexPriceKlineResult {
        &self.result
    }

    pub fn set_result(&mut self, result: GetIndexPriceKlineResult) {
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
pub struct GetIndexPriceKlineResult {
    symbol: String,
    category: String,
    list: Vec<IndexPriceKline>
}
impl GetIndexPriceKlineResult {
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

    pub fn list(&self) -> &Vec<IndexPriceKline> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<IndexPriceKline>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct IndexPriceKline {
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
impl IndexPriceKline {
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