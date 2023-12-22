use crate::{
    v5::api::{
        BybitApi,
        post::Post,
    },
    utils::serialize_as_string,
};

use serde::{
    Serialize,
    Deserialize,
};
use serde_json::Value;
use anyhow::Result;

const PATH: &'static str = "/v5/position/switch-isolated";

impl BybitApi {
    /// switch cross isolated margin.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for switch cross isolated margin.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::v5::api::{
    ///     post::position::switch_cross_isolated_margin::{
    ///         SwitchCrossIsolatedMarginParameters,
    ///         SwitchCrossIsolatedMarginCategory
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = SwitchCrossIsolatedMarginParameters::new(
    ///         SwitchCrossIsolatedMarginCategory::Linear,
    ///         "BTCUSDT".to_string(),
///             0,
    ///         10.0,
    ///         10.0,
    ///     );
    ///     let response = api.switch_cross_isolated_margin(params).await;
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
    pub async fn switch_cross_isolated_margin(&self, params: SwitchCrossIsolatedMarginParameters) -> Result<SwitchCrossIsolatedMarginResponse> {
        self.post(PATH, Some(params)).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SwitchCrossIsolatedMarginCategory {
    Linear,
    Inverse,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchCrossIsolatedMarginParameters {
    category: SwitchCrossIsolatedMarginCategory,
    symbol: String,
    #[serde(serialize_with = "serialize_as_string")]
    trade_mode: u8,
    #[serde(serialize_with = "serialize_as_string")]
    buy_leverage: f64,
    #[serde(serialize_with = "serialize_as_string")]
    sell_leverage: f64,
}

impl SwitchCrossIsolatedMarginParameters {
    /// Creates a new instance of `SwitchCrossIsolatedMarginParameters`.
    ///
    /// # Arguments
    ///
    /// * `category` - The category.
    /// * `symbol` - The symbol.
    /// * `trade_mode` - The trade_mode.
    /// * `buy_leverage` - The buy_leverage.
    /// * `sell_leverage` - The sell_leverage.
    ///
    /// # Returns
    ///
    /// A new instance of `SwitchCrossIsolatedMarginParameters`.
    pub fn new(category: SwitchCrossIsolatedMarginCategory, symbol: String, trade_mode: u8, buy_leverage: f64, sell_leverage: f64) -> Self {
        Self {
            category,
            symbol,
            trade_mode,
            buy_leverage,
            sell_leverage,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchCrossIsolatedMarginResponse {
    ret_code: i32,
    ret_msg: String,
    result: Value,
    ret_ext_info: Value,
    time: u64,
}
impl SwitchCrossIsolatedMarginResponse {
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

    pub fn result(&self) -> &Value {
        &self.result
    }

    pub fn set_result(&mut self, result: Value) {
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