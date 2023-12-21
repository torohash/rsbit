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

const PATH: &'static str = "/v5/position/set-leverage";

impl BybitApi {
    /// set leverage.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for set leverage.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::v5::api::{
    ///     post::position::set_leverage::{
    ///         SetLeverageParameters,
    ///         SetLeverageCategory
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = SetLeverageParameters::new(
    ///         SetLeverageCategory::Linear,
    ///         "BTCUSDT".to_string(),
    ///         10.0,
    ///         10.0,
    ///     );
    ///     let response = api.set_leverage(params).await;
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
    pub async fn set_leverage(&self, params: SetLeverageParameters) -> Result<SetLeverageResponse> {
        self.post(PATH, Some(params)).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SetLeverageCategory {
    Linear,
    Inverse,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLeverageParameters {
    category: SetLeverageCategory,
    symbol: String,
    #[serde(serialize_with = "serialize_as_string")]
    buy_leverage: f64,
    #[serde(serialize_with = "serialize_as_string")]
    sell_leverage: f64,
}

impl SetLeverageParameters {
    /// Creates a new instance of `SetLeverageParameters`.
    ///
    /// # Arguments
    ///
    /// * `category` - The category of the leverage.
    /// * `symbol` - The symbol of the leverage.
    /// * `buy_leverage` - The buy_leverage of the leverage.
    /// * `sell_leverage` - The sell_leverage of the leverage.
    ///
    /// # Returns
    ///
    /// A new instance of `SetLeverageParameters`.
    pub fn new(category: SetLeverageCategory, symbol: String, buy_leverage: f64, sell_leverage: f64) -> Self {
        Self {
            category,
            symbol,
            buy_leverage,
            sell_leverage,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLeverageResponse {
    ret_code: i32,
    ret_msg: String,
    result: Value,
    ret_ext_info: Value,
    time: u64,
}
impl SetLeverageResponse {
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