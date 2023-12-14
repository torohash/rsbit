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

const PATH: &'static str = "/v5/market/risk-limit";

impl BybitApi {
    /// Retrieves the risk limit for the market.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the risk limit request.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::api::{
    ///     v5::get::market::get_risk_limit::{
    ///         GetRiskLimitParameters,
    ///         GetRiskLimitCategory,
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = GetRiskLimitParameters::new(GetRiskLimitCategory::Linear);
    ///     let response = api.get_risk_limit(params).await;
    ///     match response {
    ///         Ok(info) => {
    ///             // Handle the kline data
    ///         },
    ///         Err(err) => {
    ///             // Handle the error
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn get_risk_limit(&self, params: GetRiskLimitParameters) -> Result<GetRiskLimitResponse> {
        self.get(PATH, Some(params), false).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GetRiskLimitCategory {
    Linear,
    Inverse,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRiskLimitParameters {
    category: GetRiskLimitCategory,
    symbol: Option<String>,
}

impl GetRiskLimitParameters {
    /// Creates a new instance of `GetRiskLimitParameters` with the specified category.
    ///
    /// # Arguments
    ///
    /// * `category` - The category of the risk limit.
    ///
    /// # Returns
    ///
    /// A new instance of `GetRiskLimitParameters`.
    pub fn new(category: GetRiskLimitCategory) -> Self {
        Self {
            category,
            symbol: None,
        }
    }

    /// Sets the symbol for the risk limit parameters.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The symbol to set.
    ///
    /// # Returns
    ///
    /// The modified `GetRiskLimitParameters` instance.
    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRiskLimitResponse {
    ret_code: i32,
    ret_msg: String,
    result: RiskLimitResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetRiskLimitResponse {
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

    pub fn result(&self) -> &RiskLimitResult {
        &self.result
    }

    pub fn set_result(&mut self, result: RiskLimitResult) {
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
pub struct RiskLimitResult {
    category: String,
    list: Vec<RiskLimit>
}
impl RiskLimitResult {
    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn set_category(&mut self, category: String) {
        self.category = category;
    }

    pub fn list(&self) -> &Vec<RiskLimit> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<RiskLimit>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskLimit {
    id: i64,
    symbol: String,
    #[serde(deserialize_with = "deserialize_f64")]
    risk_limit_value: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    maintenance_margin: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    initial_margin: f64,
    is_lowest_risk: i64,
    #[serde(deserialize_with = "deserialize_f64")]
    max_leverage: f64,
}
impl RiskLimit {
    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn set_id(&mut self, id: i64) {
        self.id = id;
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn risk_limit_value(&self) -> f64 {
        self.risk_limit_value
    }

    pub fn set_risk_limit_value(&mut self, risk_limit_value: f64) {
        self.risk_limit_value = risk_limit_value;
    }

    pub fn maintenance_margin(&self) -> f64 {
        self.maintenance_margin
    }

    pub fn set_maintenance_margin(&mut self, maintenance_margin: f64) {
        self.maintenance_margin = maintenance_margin;
    }

    pub fn initial_margin(&self) -> f64 {
        self.initial_margin
    }

    pub fn set_initial_margin(&mut self, initial_margin: f64) {
        self.initial_margin = initial_margin;
    }

    pub fn is_lowest_risk(&self) -> i64 {
        self.is_lowest_risk
    }

    pub fn set_is_lowest_risk(&mut self, is_lowest_risk: i64) {
        self.is_lowest_risk = is_lowest_risk;
    }

    pub fn max_leverage(&self) -> f64 {
        self.max_leverage
    }

    pub fn set_max_leverage(&mut self, max_leverage: f64) {
        self.max_leverage = max_leverage;
    }
}