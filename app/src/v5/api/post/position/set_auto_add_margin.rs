use crate::v5::api::{
    BybitApi,
    post::Post,
};

use serde::{
    Serialize,
    Deserialize,
};
use serde_json::Value;
use anyhow::Result;

const PATH: &'static str = "/v5/position/set-auto-add-margin";

impl BybitApi {
    /// set auto_add_margin.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for set auto_add_margin.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::v5::api::{
    ///     post::position::set_auto_add_margin::{
    ///         SetAutoAddMarginParameters,
    ///         SetAutoAddMarginCategory
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = SetAutoAddMarginParameters::new(
    ///         SetAutoAddMarginCategory::Linear,
    ///         "BTCUSDT".to_string(),
    ///         1,
    ///     );
    ///     let response = api.set_auto_add_margin(params).await;
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
    pub async fn set_auto_add_margin(&self, params: SetAutoAddMarginParameters) -> Result<SetAutoAddMarginResponse> {
        self.post(PATH, Some(params)).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SetAutoAddMarginCategory {
    Linear,
    Inverse,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAutoAddMarginParameters {
    category: SetAutoAddMarginCategory,
    symbol: String,
    auto_add_margin: u8,
    position_idx: Option<u8>,
}

impl SetAutoAddMarginParameters {
    /// Creates a new instance of `SetAutoAddMarginParameters`.
    ///
    /// # Arguments
    ///
    /// * `category` - The category of the leverage.
    /// * `symbol` - The symbol of the leverage.
    /// * `auto_add_margin` - The auto_add_margin of the leverage.
    /// * `position_idx` - The position_idx of the leverage.
    ///
    /// # Returns
    ///
    /// A new instance of `SetAutoAddMarginParameters`.
    pub fn new(category: SetAutoAddMarginCategory, symbol: String, auto_add_margin: u8) -> Self {
        Self {
            category,
            symbol,
            auto_add_margin,
            position_idx: None,
        }
    }

    /// Sets the position_idx for the set auto add margin.
    ///
    /// # Arguments
    ///
    /// * `position_idx` - The position_idx to set.
    ///
    /// # Returns
    ///
    /// The modified `SwitchPositionModeParameters` instance.
    pub fn with_position_idx(mut self, position_idx: u8) -> Self {
        self.position_idx = Some(position_idx);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAutoAddMarginResponse {
    ret_code: i32,
    ret_msg: String,
    result: Value,
    ret_ext_info: Value,
    time: u64,
}
impl SetAutoAddMarginResponse {
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