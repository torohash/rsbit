use crate::{
    v5::api::{
        BybitApi,
        get::Get,
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

const PATH: &'static str = "/v5/position/list";

impl BybitApi {
    /// Retrieves the position info based on the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for retrieving position info.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::v5::api::{
    ///     get::position::get_position_info::{
    ///         GetPositionInfoParameters,
    ///         GetPositionInfoCategory
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = GetPositionInfoParameters::new(GetPositionInfoCategory::Linear).with_symbol("BTCUSDT".to_string());
    ///     let response = api.get_position_info(params).await;
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
    pub async fn get_position_info(&self, params: GetPositionInfoParameters) -> Result<GetPositionInfoResponse> {
        self.get(PATH, Some(params), true).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GetPositionInfoCategory {
    Linear,
    Inverse,
    Option,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionInfoParameters {
    category: GetPositionInfoCategory,
    symbol: Option<String>,
    base_coin: Option<String>,
    settle_coin: Option<String>,
    limit: Option<u32>,
    cursor: Option<String>,
}

impl GetPositionInfoParameters {
    /// Creates a new instance of `GetPositionInfoParameters` with the specified category.
    ///
    /// # Arguments
    ///
    /// * `category` - The category of the position info.
    ///
    /// # Returns
    ///
    /// A new instance of `GetPositionInfoParameters`.
    pub fn new(category: GetPositionInfoCategory) -> Self {
        Self {
            category,
            symbol: None,
            base_coin: None,
            settle_coin: None,
            limit: None,
            cursor: None,
        }
    }

    /// Sets the symbol for the position info parameters.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The symbol to set.
    ///
    /// # Returns
    ///
    /// The modified `GetPositionInfoParameters` instance.
    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    /// Sets the base coin for the position info parameters.
    ///
    /// # Arguments
    ///
    /// * `base_coin` - The base_coin to set.
    ///
    /// # Returns
    ///
    /// The modified `GetOPositionInfoParameters` instance.
    pub fn with_base_coin(mut self, base_coin: String) -> Self {
        self.base_coin = Some(base_coin);
        self
    }

    /// Sets the settle coin for the position info parameters.
    ///
    /// # Arguments
    ///
    /// * `settle_coin` - The settle coin to set.
    ///
    /// # Returns
    ///
    /// The modified `GetPositionInfoParameters` instance.
    pub fn with_settle_coin(mut self, settle_coin: String) -> Self {
        self.settle_coin = Some(settle_coin);
        self
    }

    /// Sets the limit for the position info parameters.
    ///
    /// # Arguments
    ///
    /// * `limit` - The limit to set.
    ///
    /// # Returns
    ///
    /// The modified `GetPositionInfoParameters` instance.
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the cursor for the position info parameters.
    ///
    /// # Arguments
    ///
    /// * `cursor` - The cursor to set.
    ///
    /// # Returns
    ///
    /// The modified `GetPositionInfoParameters` instance.
    pub fn with_cursor(mut self, cursor: String) -> Self {
        self.cursor = Some(cursor);
        self
    }

}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionInfoResponse {
    ret_code: i32,
    ret_msg: String,
    result: PositionInfoResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetPositionInfoResponse {
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

    pub fn result(&self) -> &PositionInfoResult {
        &self.result
    }

    pub fn set_result(&mut self, result: PositionInfoResult) {
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
pub struct PositionInfoResult {
    category: String,
    next_page_cursor: String,
    list: Vec<PositionInfo>
}
impl PositionInfoResult {
    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn set_category(&mut self, category: String) {
        self.category = category;
    }

    pub fn next_page_cursor(&self) -> &str {
        &self.next_page_cursor
    }

    pub fn set_next_page_cursor(&mut self, next_page_cursor: String) {
        self.next_page_cursor = next_page_cursor;
    }

    pub fn list(&self) -> &Vec<PositionInfo> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<PositionInfo>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionInfo {
    position_idx: u8,
    risk_id: u64,
    #[serde(deserialize_with = "deserialize_f64")]
    risk_limit_value: f64,
    symbol: String,
    side: Option<String>,
    #[serde(deserialize_with = "deserialize_f64")]
    avg_price: f64,
    #[serde(deserialize_with = "deserialize_option_f64")]
    position_value: Option<f64>,
    trade_mode: u8,
    auto_add_margin: u8,
    position_status: String,
    #[serde(deserialize_with = "deserialize_f64")]
    leverage: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    mark_price: f64,
    liq_price: String,
    #[serde(deserialize_with = "deserialize_option_f64")]
    bust_price: Option<f64>,
    #[serde(deserialize_with = "deserialize_f64")]
    position_m_m: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    position_i_m: f64,
    tpsl_mode: String,
    #[serde(deserialize_with = "deserialize_option_f64")]
    take_profit: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    stop_loss: Option<f64>,
    #[serde(deserialize_with = "deserialize_f64")]
    trailing_stop: f64,
    #[serde(deserialize_with = "deserialize_option_f64")]
    unrealised_pnl: Option<f64>,
    #[serde(deserialize_with = "deserialize_f64")]
    cum_realised_pnl: f64,
    seq: u64,
    is_reduce_only: bool,
    mmr_sys_updated_time: String,
    leverage_sys_updated_time: String,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    created_time: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    updated_time: u64,
}
impl PositionInfo {
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

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn side(&self) -> Option<&str> {
        self.side.as_deref()
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

    pub fn position_value(&self) -> Option<f64> {
        self.position_value
    }

    pub fn set_position_value(&mut self, position_value: f64) {
        self.position_value = Some(position_value);
    }

    pub fn trade_mode(&self) -> u8 {
        self.trade_mode
    }

    pub fn set_trade_mode(&mut self, trade_mode: u8) {
        self.trade_mode = trade_mode;
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

    pub fn leverage(&self) -> f64 {
        self.leverage
    }

    pub fn set_leverage(&mut self, leverage: f64) {
        self.leverage = leverage;
    }

    pub fn mark_price(&self) -> f64 {
        self.mark_price
    }

    pub fn set_mark_price(&mut self, mark_price: f64) {
        self.mark_price = mark_price;
    }

    pub fn liq_price(&self) -> &str {
        &self.liq_price
    }

    pub fn set_liq_price(&mut self, liq_price: String) {
        self.liq_price = liq_price;
    }

    pub fn bust_price(&self) -> Option<f64> {
        self.bust_price
    }

    pub fn set_bust_price(&mut self, bust_price: f64) {
        self.bust_price = Some(bust_price);
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

    pub fn tpsl_mode(&self) -> &str {
        &self.tpsl_mode
    }

    pub fn set_tpsl_mode(&mut self, tpsl_mode: String) {
        self.tpsl_mode = tpsl_mode;
    }

    pub fn take_profit(&self) -> Option<f64> {
        self.take_profit
    }

    pub fn set_take_profit(&mut self, take_profit: f64) {
        self.take_profit = Some(take_profit);
    }

    pub fn stop_loss(&self) -> Option<f64> {
        self.stop_loss
    }

    pub fn set_stop_loss(&mut self, stop_loss: f64) {
        self.stop_loss = Some(stop_loss);
    }

    pub fn trailing_stop(&self) -> f64 {
        self.trailing_stop
    }

    pub fn set_trailing_stop(&mut self, trailing_stop: f64) {
        self.trailing_stop = trailing_stop;
    }

    pub fn unrealised_pnl(&self) -> Option<f64> {
        self.unrealised_pnl
    }

    pub fn set_unrealised_pnl(&mut self, unrealised_pnl: f64) {
        self.unrealised_pnl = Some(unrealised_pnl);
    }

    pub fn cum_realised_pnl(&self) -> f64 {
        self.cum_realised_pnl
    }

    pub fn set_cum_realised_pnl(&mut self, cum_realised_pnl: f64) {
        self.cum_realised_pnl = cum_realised_pnl;
    }

    pub fn seq(&self) -> u64 {
        self.seq
    }

    pub fn set_seq(&mut self, seq: u64) {
        self.seq = seq;
    }

    pub fn is_reduce_only(&self) -> bool {
        self.is_reduce_only
    }

    pub fn set_is_reduce_only(&mut self, is_reduce_only: bool) {
        self.is_reduce_only = is_reduce_only;
    }

    pub fn mmr_sys_updated_time(&self) -> &str {
        &self.mmr_sys_updated_time
    }

    pub fn set_mmr_sys_updated_time(&mut self, mmr_sys_updated_time: String) {
        self.mmr_sys_updated_time = mmr_sys_updated_time;
    }

    pub fn leverage_sys_updated_time(&self) -> &str {
        &self.leverage_sys_updated_time
    }

    pub fn set_leverage_sys_updated_time(&mut self, leverage_sys_updated_time: String) {
        self.leverage_sys_updated_time = leverage_sys_updated_time;
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