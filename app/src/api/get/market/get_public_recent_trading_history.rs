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

const PATH: &'static str = "/v5/market/recent-trade";

impl BybitApi {
    /// Retrieves the recent trading history from the public API.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the API request.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::api::{
    ///     get::market::get_public_recent_trading_history::{
    ///         GetPublicRecentTradingHistoryParameters,
    ///         GetPublicRecentTradingHistoryCategory
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = GetPublicRecentTradingHistoryParameters::new(GetPublicRecentTradingHistoryCategory::Linear);
    ///     let response = api.get_public_recent_trading_history(params).await;
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
    pub async fn get_public_recent_trading_history(&self, params: GetPublicRecentTradingHistoryParameters) -> Result<GetPublicRecentTradingHistoryResponse> {
        self.get(PATH, Some(params), false).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GetPublicRecentTradingHistoryCategory {
    Linear,
    Inverse,
    Spot,
    Option,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetPublicRecentTradingHistoryParameters {
    category: GetPublicRecentTradingHistoryCategory,
    symbol: Option<String>,
    base_coin: Option<String>,
    option_type: Option<String>,
    limit: Option<u32>,
}

impl GetPublicRecentTradingHistoryParameters {
    /// Creates a new instance of `GetPublicRecentTradingHistoryParameters` with the specified category.
    ///
    /// # Arguments
    ///
    /// * `category` - The category of the trading history.
    ///
    /// # Returns
    ///
    /// A new instance of `GetPublicRecentTradingHistoryParameters`.
    pub fn new(category: GetPublicRecentTradingHistoryCategory) -> Self {
        Self {
            category,
            symbol: None,
            base_coin: None,
            option_type: None,
            limit: None,
        }
    }

    /// Sets the symbol for the trading history.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The symbol to set.
    ///
    /// # Returns
    ///
    /// The modified `GetPublicRecentTradingHistoryParameters` instance.
    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    /// Sets the base coin for the trading history.
    ///
    /// # Arguments
    ///
    /// * `base_coin` - The base coin to set.
    ///
    /// # Returns
    ///
    /// The modified `GetPublicRecentTradingHistoryParameters` instance.
    pub fn with_base_coin(mut self, base_coin: String) -> Self {
        self.base_coin = Some(base_coin);
        self
    }

    /// Sets the limit for the trading history.
    ///
    /// # Arguments
    ///
    /// * `limit` - The limit to set.
    ///
    /// # Returns
    ///
    /// The modified `GetPublicRecentTradingHistoryParameters` instance.
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPublicRecentTradingHistoryResponse {
    ret_code: i32,
    ret_msg: String,
    result: PublicRecentTradingHistoryResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetPublicRecentTradingHistoryResponse {
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

    pub fn result(&self) -> &PublicRecentTradingHistoryResult {
        &self.result
    }

    pub fn set_result(&mut self, result: PublicRecentTradingHistoryResult) {
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
pub struct PublicRecentTradingHistoryResult {
    category: String,
    list: Vec<PublicRecentTradingHistory>
}
impl PublicRecentTradingHistoryResult {
    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn set_category(&mut self, category: String) {
        self.category = category;
    }

    pub fn list(&self) -> &Vec<PublicRecentTradingHistory> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<PublicRecentTradingHistory>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicRecentTradingHistory {
    exec_id: String,
    symbol: String,
    #[serde(deserialize_with = "deserialize_f64")]
    price: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    size: f64,
    side: String,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    time: u64,
    is_block_trade: bool
}
impl PublicRecentTradingHistory {
    pub fn exec_id(&self) -> &str {
        &self.exec_id
    }

    pub fn set_exec_id(&mut self, exec_id: String) {
        self.exec_id = exec_id;
    }
    
    pub fn symbol(&self) -> &String {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

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

    pub fn side(&self) -> &str {
        &self.side
    }

    pub fn set_side(&mut self, side: String) {
        self.side = side;
    }

    pub fn time(&self) -> u64 {
        self.time
    }

    pub fn set_time(&mut self, time: u64) {
        self.time = time;
    }

    pub fn is_block_trade(&self) -> bool {
        self.is_block_trade
    }

    pub fn set_is_block_trade(&mut self, is_block_trade: bool) {
        self.is_block_trade = is_block_trade;
    }
    
}