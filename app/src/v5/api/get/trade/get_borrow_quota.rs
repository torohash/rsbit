use crate::{
    v5::api::{
        BybitApi,
        get::Get,
    },
    utils::deserialize_f64,
};

use serde::{
    Serialize,
    Deserialize,
};
use serde_json::Value;
use anyhow::Result;

const PATH: &'static str = "/v5/order/spot-borrow-check";

impl BybitApi {
    /// Retrieves the borrow quota information based on the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for retrieving borrow quota.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::v5::api::{
    ///     get::trade::get_borrow_quota::{
    ///         GetBorrowQuotaParameters,
    ///         GetBorrowQuotaCategory
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = GetBorrowQuotaParameters::new(GetBorrowQuotaCategory::Spot, "BTCUSDT".to_string(), "Buy".to_string());
    ///     let response = api.get_borrow_quota(params).await;
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
    pub async fn get_borrow_quota(&self, params: GetBorrowQuotaParameters) -> Result<GetBorrowQuotaResponse> {
        self.get(PATH, Some(params), true).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GetBorrowQuotaCategory {
    Spot,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBorrowQuotaParameters {
    category: GetBorrowQuotaCategory,
    symbol: String,
    side: String,
}

impl GetBorrowQuotaParameters {
    /// Creates a new instance of `GetBorrowQuotaParameters` with the specified category.
    ///
    /// # Arguments
    ///
    /// * `category` - The category of the borrow quota.
    /// * `symbol` - The symbol of the borrow quota.
    /// * `side` - The side of the borrow quota.
    ///
    /// # Returns
    ///
    /// A new instance of `GetBorrowQuotaParameters`.
    pub fn new(category: GetBorrowQuotaCategory, symbol: String, side: String) -> Self {
        Self {
            category,
            symbol,
            side
        }
    }

}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBorrowQuotaResponse {
    ret_code: i32,
    ret_msg: String,
    result: BorrowQuotaResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetBorrowQuotaResponse {
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

    pub fn result(&self) -> &BorrowQuotaResult {
        &self.result
    }

    pub fn set_result(&mut self, result: BorrowQuotaResult) {
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
pub struct BorrowQuotaResult {
    symbol: String,
    #[serde(deserialize_with = "deserialize_f64")]
    max_trade_qty: f64,
    side: String,
    #[serde(deserialize_with = "deserialize_f64")]
    spot_max_trade_amount: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    max_trade_amount: f64,
    borrow_coin: String,
    #[serde(deserialize_with = "deserialize_f64")]
    spot_max_trade_qty: f64,
}
impl BorrowQuotaResult {
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn max_trade_qty(&self) -> f64 {
        self.max_trade_qty
    }

    pub fn set_max_trade_qty(&mut self, max_trade_qty: f64) {
        self.max_trade_qty = max_trade_qty;
    }

    pub fn side(&self) -> &str {
        &self.side
    }

    pub fn set_side(&mut self, side: String) {
        self.side = side;
    }

    pub fn spot_max_trade_amount(&self) -> f64 {
        self.spot_max_trade_amount
    }

    pub fn set_spot_max_trade_amount(&mut self, spot_max_trade_amount: f64) {
        self.spot_max_trade_amount = spot_max_trade_amount;
    }

    pub fn max_trade_amount(&self) -> f64 {
        self.max_trade_amount
    }

    pub fn set_max_trade_amount(&mut self, max_trade_amount: f64) {
        self.max_trade_amount = max_trade_amount;
    }

    pub fn borrow_coin(&self) -> &str {
        &self.borrow_coin
    }

    pub fn set_borrow_coin(&mut self, borrow_coin: String) {
        self.borrow_coin = borrow_coin;
    }

    pub fn spot_max_trade_qty(&self) -> f64 {
        self.spot_max_trade_qty
    }

    pub fn set_spot_max_trade_qty(&mut self, spot_max_trade_qty: f64) {
        self.spot_max_trade_qty = spot_max_trade_qty;
    }

}