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

const PATH: &'static str = "/v5/position/switch-mode";

impl BybitApi {
    /// switch position mode.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for switch position mode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::v5::api::{
    ///     post::position::switch_position_mode::{
    ///         SwitchPositionModeParameters,
    ///         SwitchPositionModeCategory
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = SwitchPositionModeParameters::new(
    ///         SwitchPositionModeCategory::Linear,
    ///         0,
    ///     ).with_symbol("BTCUSDT".to_string());
    ///     let response = api.switch_position_mode(params).await;
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
    pub async fn switch_position_mode(&self, params: SwitchPositionModeParameters) -> Result<SwitchPositionModeResponse> {
        self.post(PATH, Some(params)).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SwitchPositionModeCategory {
    Linear,
    Inverse,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchPositionModeParameters {
    category: SwitchPositionModeCategory,
    symbol: Option<String>,
    coin: Option<String>,
    #[serde(serialize_with = "serialize_as_string")]
    mode: u8,
}

impl SwitchPositionModeParameters {
    /// Creates a new instance of `SwitchPositionModeParameters`.
    ///
    /// # Arguments
    ///
    /// * `category` - The category.
    /// * `symbol` - The symbol.
    /// * `coin` - The coin.
    /// * `mode` - The mode.
    ///
    /// # Returns
    ///
    /// A new instance of `SwitchPositionModeParameters`.
    pub fn new(category: SwitchPositionModeCategory, mode: u8) -> Self {
        Self {
            category,
            symbol: None,
            coin: None,
            mode,
        }
    }

        
    /// Sets the symbol for the switch position mode.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The symbol to set.
    ///
    /// # Returns
    ///
    /// The modified `SwitchPositionModeParameters` instance.
    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    /// Sets the coin for the switch position mode.
    ///
    /// # Arguments
    ///
    /// * `coin` - The coin to set.
    ///
    /// # Returns
    ///
    /// The modified `SwitchPositionModeParameters` instance.
    pub fn with_coin(mut self, coin: String) -> Self {
        self.coin = Some(coin);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchPositionModeResponse {
    ret_code: i32,
    ret_msg: String,
    result: Value,
    ret_ext_info: Value,
    time: u64,
}
impl SwitchPositionModeResponse {
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