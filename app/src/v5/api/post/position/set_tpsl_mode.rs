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

const PATH: &'static str = "/v5/position/set-tpsl-mode";

impl BybitApi {
    /// set tpsl mode.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for set tpsl mode.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::v5::api::{
    ///     post::position::set_tpsl_mode::{
    ///         SetTpslModeParameters,
    ///         SetTpslModeCategory
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = SetTpslModeParameters::new(
    ///         SetTpslModeCategory::Linear,
    ///         "BTCUSDT".to_string(),
    ///         "Full".to_string(),
    ///     );
    ///     let response = api.set_tpsl_mode(params).await;
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
    pub async fn set_tpsl_mode(&self, params: SetTpslModeParameters) -> Result<SetTpslModeResponse> {
        self.post(PATH, Some(params)).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SetTpslModeCategory {
    Linear,
    Inverse,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTpslModeParameters {
    category: SetTpslModeCategory,
    symbol: String,
    tp_sl_mode: String,  
}

impl SetTpslModeParameters {
    /// Creates a new instance of `SetTpslModeParameters`.
    ///
    /// # Arguments
    ///
    /// * `category` - The category.
    /// * `symbol` - The symbol.
    /// * `tp_sl_mode` - The tp_sl_mode.
    ///
    /// # Returns
    ///
    /// A new instance of `SetTpslModeParameters`.
    pub fn new(category: SetTpslModeCategory, symbol: String, tp_sl_mode: String) -> Self {
        Self {
            category,
            symbol,
            tp_sl_mode,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTpslModeResponse {
    ret_code: i32,
    ret_msg: String,
    result: SetTpslModeResult,
    ret_ext_info: Value,
    time: u64,
}
impl SetTpslModeResponse {
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

    pub fn result(&self) -> &SetTpslModeResult {
        &self.result
    }

    pub fn set_result(&mut self, result: SetTpslModeResult) {
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
pub struct SetTpslModeResult {
    tp_sl_mode: String,
}

impl SetTpslModeResult {
    pub fn tp_sl_mode(&self) -> &str {
        &self.tp_sl_mode
    }

    pub fn set_tp_sl_mode(&mut self, tp_sl_mode: String) {
        self.tp_sl_mode = tp_sl_mode;
    }
}