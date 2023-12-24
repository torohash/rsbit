use crate::{
    v5::api::{
        BybitApi,
        post::Post,
    },
    utils::{
        deserialize_f64,
        deserialize_string_to_u64,
        deserialize_option_f64,
    },
};

use serde::{
    Serialize,
    Deserialize,
};
use serde_json::Value;
use anyhow::Result;

const PATH: &'static str = "/v5/position/add-margin";

impl BybitApi {
    /// add or reduce margin.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for add or reduce margin.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::v5::api::{
    ///     post::position::add_or_reduce_margin::{
    ///         AddOrReduceMarginParameters,
    ///         AddOrReduceMarginCategory
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = AddOrReduceMarginParameters::new(
    ///         AddOrReduceMarginCategory::Linear,
    ///         "BTCUSDT".to_string(),
    ///         "1".to_string(),
    ///     );
    ///     let response = api.add_or_reduce_margin(params).await;
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
    pub async fn add_or_reduce_margin(&self, params: AddOrReduceMarginParameters) -> Result<AddOrReduceMarginResponse> {
        self.post(PATH, Some(params)).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AddOrReduceMarginCategory {
    Linear,
    Inverse,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddOrReduceMarginParameters {
    category: AddOrReduceMarginCategory,
    symbol: String,
    margin: String,
    position_idx: Option<u8>,
}

impl AddOrReduceMarginParameters {
    /// Creates a new instance of `AddOrReduceMarginParameters`.
    ///
    /// # Arguments
    ///
    /// * `category` - The category.
    /// * `symbol` - The symbol.
    /// * `margin` - The margin.
    ///
    /// # Returns
    ///
    /// A new instance of `AddOrReduceMarginParameters`.
    pub fn new(category: AddOrReduceMarginCategory, symbol: String, margin: String) -> Self {
        Self {
            category,
            symbol,
            margin,
            position_idx: None,
        }
    }

    /// Sets the position_idx for the add or reduce margin.
    ///
    /// # Arguments
    ///
    /// * `position_idx` - The position_idx to set.
    ///
    /// # Returns
    ///
    /// The modified `AddOrReduceMarginParameters` instance.
    pub fn with_position_idx(mut self, position_idx: u8) -> Self {
        self.position_idx = Some(position_idx);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddOrReduceMarginResponse {
    ret_code: i32,
    ret_msg: String,
    result: AddOrReduceMarginResult,
    ret_ext_info: Value,
    time: u64,
}
impl AddOrReduceMarginResponse {
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

    pub fn result(&self) -> &AddOrReduceMarginResult {
        &self.result
    }

    pub fn set_result(&mut self, result: AddOrReduceMarginResult) {
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
pub struct AddOrReduceMarginResult {
    category: String,
    symbol: String,
    position_idx: u8,
    risk_id: u64,
    #[serde(deserialize_with = "deserialize_f64")]
    risk_limit_value: f64,
    side: Option<String>,
    #[serde(deserialize_with = "deserialize_f64")]
    avg_price: f64,
    liq_price: String,
    #[serde(deserialize_with = "deserialize_option_f64")]
    bust_price: Option<f64>,
    #[serde(deserialize_with = "deserialize_f64")]
    mark_price: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    position_value: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    leverage: f64,
    auto_add_margin: u8,
    position_status: String,
    #[serde(deserialize_with = "deserialize_f64")]
    position_m_m: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    position_i_m: f64,
    #[serde(deserialize_with = "deserialize_option_f64")]
    take_profit: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    stop_loss: Option<f64>,
    #[serde(deserialize_with = "deserialize_f64")]
    trailing_stop: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    unrealised_pnl: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    cum_realised_pnl: f64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    created_time: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    updated_time: u64,
}

impl AddOrReduceMarginResult {
    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn set_category(&mut self, category: String) {
        self.category = category;
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn position_idx(&self) -> u8 {
        self.position_idx
    }

    pub fn set_position_idx(&mut self, position_idx: u8) {
        self.position_idx = position_idx;
    }

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

    pub fn side(&self) -> &Option<String> {
        &self.side
    }

    pub fn set_side(&mut self, side: Option<String>) {
        self.side = side;
    }

    pub fn avg_price(&self) -> f64 {
        self.avg_price
    }

    pub fn set_avg_price(&mut self, avg_price: f64) {
        self.avg_price = avg_price;
    }

    pub fn liq_price(&self) -> &str {
        &self.liq_price
    }

    pub fn set_liq_price(&mut self, liq_price: String) {
        self.liq_price = liq_price;
    }

    pub fn bust_price(&self) -> &Option<f64> {
        &self.bust_price
    }

    pub fn set_bust_price(&mut self, bust_price: Option<f64>) {
        self.bust_price = bust_price;
    }

    pub fn mark_price(&self) -> f64 {
        self.mark_price
    }

    pub fn set_mark_price(&mut self, mark_price: f64) {
        self.mark_price = mark_price;
    }

    pub fn position_value(&self) -> f64 {
        self.position_value
    }

    pub fn set_position_value(&mut self, position_value: f64) {
        self.position_value = position_value;
    }

    pub fn leverage(&self) -> f64 {
        self.leverage
    }

    pub fn set_leverage(&mut self, leverage: f64) {
        self.leverage = leverage;
    }

    pub fn auto_add_margin(&self) -> u8 {
        self.auto_add_margin
    }

    pub fn set_auto_add_margin(&mut self, auto_add_margin: u8) {
        self.auto_add_margin = auto_add_margin;
    }

    pub fn position_status(&self) -> &str {
        &self.position_status
    }

    pub fn set_position_status(&mut self, position_status: String) {
        self.position_status = position_status;
    }

    pub fn position_m_m(&self) -> f64 {
        self.position_m_m
    }

    pub fn set_position_m_m(&mut self, position_m_m: f64) {
        self.position_m_m = position_m_m;
    }

    pub fn position_i_m(&self) -> f64 {
        self.position_i_m
    }

    pub fn set_position_i_m(&mut self, position_i_m: f64) {
        self.position_i_m = position_i_m;
    }

    pub fn take_profit(&self) -> &Option<f64> {
        &self.take_profit
    }

    pub fn set_take_profit(&mut self, take_profit: Option<f64>) {
        self.take_profit = take_profit;
    }

    pub fn stop_loss(&self) -> &Option<f64> {
        &self.stop_loss
    }

    pub fn set_stop_loss(&mut self, stop_loss: Option<f64>) {
        self.stop_loss = stop_loss;
    }

    pub fn trailing_stop(&self) -> f64 {
        self.trailing_stop
    }

    pub fn set_trailing_stop(&mut self, trailing_stop: f64) {
        self.trailing_stop = trailing_stop;
    }

    pub fn unrealised_pnl(&self) -> f64 {
        self.unrealised_pnl
    }

    pub fn set_unrealised_pnl(&mut self, unrealised_pnl: f64) {
        self.unrealised_pnl = unrealised_pnl;
    }

    pub fn cum_realised_pnl(&self) -> f64 {
        self.cum_realised_pnl
    }

    pub fn set_cum_realised_pnl(&mut self, cum_realised_pnl: f64) {
        self.cum_realised_pnl = cum_realised_pnl;
    }

    pub fn created_time(&self) -> u64 {
        self.created_time
    }

    pub fn set_created_time(&mut self, created_time: u64) {
        self.created_time = created_time;
    }

    pub fn updated_time(&self) -> u64 {
        self.updated_time
    }

    pub fn set_updated_time(&mut self, updated_time: u64) {
        self.updated_time = updated_time;
    }

}