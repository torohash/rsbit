use crate::{
    v5::api::{
        BybitApi,
        post::Post,
    },
    utils::deserialize_f64,
};

use serde::{
    Serialize,
    Deserialize,
};
use serde_json::Value;
use anyhow::Result;

const PATH: &'static str = "/v5/position/set-risk-limit";

impl BybitApi {
    /// set risk limit.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for set risk limit.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::v5::api::{
    ///     post::position::set_risk_limit::{
    ///         SetRiskLimitParameters,
    ///         SetRiskLimitCategory
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = SetRiskLimitParameters::new(
    ///         SetRiskLimitCategory::Linear,
    ///         "BTCUSDT".to_string(),
    ///         1,
    ///     );
    ///     let response = api.set_risk_limit(params).await;
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
    pub async fn set_risk_limit(&self, params: SetRiskLimitParameters) -> Result<SetRiskLimitResponse> {
        self.post(PATH, Some(params)).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SetRiskLimitCategory {
    Linear,
    Inverse,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetRiskLimitParameters {
    category: SetRiskLimitCategory,
    symbol: String,
    risk_id: u64,
    position_idx: Option<u8>
}

impl SetRiskLimitParameters {
    /// Creates a new instance of `SetRiskLimitParameters`.
    ///
    /// # Arguments
    ///
    /// * `category` - The category.
    /// * `symbol` - The symbol.
    /// * `risk_id` - The risk id.
    /// * `position_idx` - The position idx.
    ///
    /// # Returns
    ///
    /// A new instance of `SetRiskLimitParameters`.
    pub fn new(category: SetRiskLimitCategory, symbol: String, risk_id: u64) -> Self {
        Self {
            category,
            symbol,
            risk_id,
            position_idx: None,
        }
    }
        
    /// Sets the position idx for the set risk limit.
    ///
    /// # Arguments
    ///
    /// * `position_idx` - The position idx to set.
    ///
    /// # Returns
    ///
    /// The modified `SetRiskLimitParameters` instance.
    pub fn with_position_idx(mut self, position_idx: u8) -> Self {
        self.position_idx = Some(position_idx);
        self
    }

}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetRiskLimitResponse {
    ret_code: i32,
    ret_msg: String,
    result: SetRiskLimitResult,
    ret_ext_info: Value,
    time: u64,
}
impl SetRiskLimitResponse {
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

    pub fn result(&self) -> &SetRiskLimitResult {
        &self.result
    }

    pub fn set_result(&mut self, result: SetRiskLimitResult) {
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
pub struct SetRiskLimitResult {
    risk_id: u64,
    #[serde(deserialize_with = "deserialize_f64")]
    risk_limit_value: f64,
    category: String,
}

impl SetRiskLimitResult {
    pub fn risk_id(&self) -> u64 {
        self.risk_id
    }

    pub fn set_risk_id(&mut self, risk_id: u64) {
        self.risk_id = risk_id;
    }

    pub fn risk_limit_value(&self) -> f64 {
        self.risk_limit_value
    }

    pub fn set_risk_limit_value(&mut self, risk_limit_value: f64) {
        self.risk_limit_value = risk_limit_value;
    }

    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn set_category(&mut self, category: String) {
        self.category = category;
    }
}