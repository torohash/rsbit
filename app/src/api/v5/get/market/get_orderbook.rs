use crate::{
    api::{
        BybitApi,
        v5::get::Get,
    },
    utils::deserialize_f64,
};

use serde::{
    Serialize,
    Deserialize,
};
use serde_json::Value;
use anyhow::Result;

const PATH: &'static str = "/v5/market/orderbook";

impl BybitApi {
    /// Retrieves the market's orderbook.
    ///
    /// This method asynchronously fetches the orderbook for the specified parameters.
    /// It returns a `Result` containing the `GetOrderbookResponse` if successful.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for fetching the index price kline.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::api::{
    ///     v5::get::market::get_orderbook::{
    ///         GetOrderbookParameters,
    ///         GetOrderbookCategory
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = GetOrderbookParameters::new(GetOrderbookCategory::Linear, "BTCUSDT".to_string());
    ///     let response = api.get_orderbook(params).await;
    ///     match response {
    ///         Ok(info) => {
    ///             // Handle the orderbook
    ///         },
    ///         Err(err) => {
    ///             // Handle the error
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn get_orderbook(&self, params: GetOrderbookParameters) -> Result<GetOrderbookResponse> {
        self.get(PATH, Some(params), false).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GetOrderbookCategory {
    Linear,
    Spot,
    Inverse,
    Option,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetOrderbookParameters {
    category: GetOrderbookCategory,
    symbol: String,
    limit: Option<u32>,
}

impl GetOrderbookParameters {
    /// Creates a new instance of `GetOrderbookParameters` with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `category` - The category of the orderbook.
    /// * `symbol` - The symbol of the orderbook.
    ///
    /// # Returns
    ///
    /// A new instance of `GetOrderbookParameters`.
    pub fn new(category: GetOrderbookCategory, symbol: String) -> Self {
        Self {
            category,
            symbol,
            limit: None,
        }
    }

    /// Sets the limit for the number of orderbook to retrieve.
    ///
    /// # Arguments
    ///
    /// * `limit` - The limit of orderbook.
    ///
    /// # Returns
    ///
    /// The modified GetOrderbookParameters instance.
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderbookResponse {
    ret_code: i32,
    ret_msg: String,
    result: OrderbookResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetOrderbookResponse {
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

    pub fn result(&self) -> &OrderbookResult {
        &self.result
    }

    pub fn set_result(&mut self, result: OrderbookResult) {
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

#[derive(Debug, Deserialize, Clone)]
pub struct OrderbookResult {
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "b")]
    pub bids: Vec<Order>,
    #[serde(rename = "a")]
    pub asks: Vec<Order>,
    #[serde(rename = "u")]
    update_id: u64,
    ts: u64,
}
impl OrderbookResult {
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn bids(&self) -> &Vec<Order> {
        &self.bids
    }

    pub fn set_bids(&mut self, bids: Vec<Order>) {
        self.bids = bids;
    }

    pub fn asks(&self) -> &Vec<Order> {
        &self.asks
    }

    pub fn set_asks(&mut self, asks: Vec<Order>) {
        self.asks = asks;
    }

    pub fn update_id(&self) -> u64 {
        self.update_id
    }

    pub fn set_update_id(&mut self, update_id: u64) {
        self.update_id = update_id;
    }

    pub fn ts(&self) -> u64 {
        self.ts
    }

    pub fn set_ts(&mut self, ts: u64) {
        self.ts = ts;
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Order {
    #[serde(rename = "0", deserialize_with = "deserialize_f64")]
    price: f64,
    #[serde(rename = "1", deserialize_with = "deserialize_f64")]
    size: f64,
}
impl Order {
    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn set_price(&mut self, price: f64) {
        self.price = price;
    }

    pub fn size(&self) -> f64 {
        self.size
    }

    pub fn set_size(&mut self, size: f64) {
        self.size = size;
    }
}
