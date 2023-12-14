use crate::{
    v5::api::{
        BybitApi,
        get::Get,
    },
    utils::{
        deserialize_f64,
        deserialize_option_f64,
        deserialize_string_to_u64,
    },
};

use serde::{
    Serialize,
    Deserialize,
};
use serde_json::Value;
use anyhow::Result;

const PATH: &'static str = "/v5/order/history";

impl BybitApi {
    /// Retrieves the order history for the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the order history request.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the order history response if successful, or an error if the request fails.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use rsbit::v5::api::{
    ///     get::trade::get_order_history::{
    ///         GetOrderHistoryParameters,
    ///         GetOrderHistoryCategory
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = GetOrderHistoryParameters::new(GetOrderHistoryCategory::Linear);
    ///     let response = api.get_order_history(params).await;
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
    pub async fn get_order_history(&self, params: GetOrderHistoryParameters) -> Result<GetOrderHistoryResponse> {
        self.get(PATH, Some(params), true).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GetOrderHistoryCategory {
    Linear,
    Inverse,
    Option,
    Spot,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderHistoryParameters {
    category: GetOrderHistoryCategory,
    symbol: Option<String>,
    base_coin: Option<String>,
    settle_coin: Option<String>,
    order_id: Option<String>,
    order_link_id: Option<String>,
    order_filter: Option<String>,
    order_status: Option<String>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    limit: Option<u32>,
    cursor: Option<String>,
}

impl GetOrderHistoryParameters {
    /// Creates a new instance of `GetOrderHistoryParameters` with the specified category.
    ///
    /// # Arguments
    ///
    /// * `category` - The category of the risk limit.
    ///
    /// # Returns
    ///
    /// A new instance of `GetOrderHistoryParameters`.
    pub fn new(category: GetOrderHistoryCategory) -> Self {
        Self {
            category,
            symbol: None,
            base_coin: None,
            settle_coin: None,
            order_id: None,
            order_link_id: None,
            order_filter: None,
            order_status: None,
            start_time: None,
            end_time: None,
            limit: None,
            cursor: None,
        }
    }

    /// Sets the symbol for the order history parameters.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The symbol to set.
    ///
    /// # Returns
    ///
    /// The modified `GetOrderHistoryParameters` instance.
    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    /// Sets the base coin for the order history parameters.
    ///
    /// # Arguments
    ///
    /// * `base_coin` - The base coin to set.
    ///
    /// # Returns
    ///
    /// The modified `GetOrderHistoryParameters` instance.
    pub fn with_base_coin(mut self, base_coin: String) -> Self {
        self.base_coin = Some(base_coin);
        self
    }

    /// Sets the settle coin for the order history parameters.
    ///
    /// # Arguments
    ///
    /// * `settle_coin` - The settle coin to set.
    ///
    /// # Returns
    ///
    /// The modified `GetOrderHistoryParameters` instance.
    pub fn with_settle_coin(mut self, settle_coin: String) -> Self {
        self.settle_coin = Some(settle_coin);
        self
    }

    /// Sets the order id for the order history parameters.
    ///
    /// # Arguments
    ///
    /// * `order_id` - The order id to set.
    ///
    /// # Returns
    ///
    /// The modified `GetOrderHistoryParameters` instance.
    pub fn with_order_id(mut self, order_id: String) -> Self {
        self.order_id = Some(order_id);
        self
    }

    /// Sets the order link id for the order history parameters.
    ///
    /// # Arguments
    ///
    /// * `order_link_id` - The order link id to set.
    ///
    /// # Returns
    ///
    /// The modified `GetOrderHistoryParameters` instance.
    pub fn with_order_link_id(mut self, order_link_id: String) -> Self {
        self.order_link_id = Some(order_link_id);
        self
    }

    /// Sets the order filter for the order history parameters.
    ///
    /// # Arguments
    ///
    /// * `order_filter` - The order filter to set.
    ///
    /// # Returns
    ///
    /// The modified `GetOrderHistoryParameters` instance.
    pub fn with_order_filter(mut self, order_filter: String) -> Self {
        self.order_filter = Some(order_filter);
        self
    }

    /// Sets the order status for the order history parameters.
    ///
    /// # Arguments
    ///
    /// * `order_status` - The order status to set.
    ///
    /// # Returns
    ///
    /// The modified `GetOrderHistoryParameters` instance.
    pub fn with_order_status(mut self, order_status: String) -> Self {
        self.order_status = Some(order_status);
        self
    }

    /// Sets the start time for the order history parameters.
    ///
    /// # Arguments
    ///
    /// * `start_time` - The start time to set.
    ///
    /// # Returns
    ///
    /// The modified `GetOrderHistoryParameters` instance.
    pub fn with_start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// Sets the end time for the order history parameters.
    ///
    /// # Arguments
    ///
    /// * `end_time` - The end time to set.
    ///
    /// # Returns
    ///
    /// The modified `GetOrderHistoryParameters` instance.
    pub fn with_end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// Sets the limit for the order history parameters.
    ///
    /// # Arguments
    ///
    /// * `limit` - The limit to set.
    ///
    /// # Returns
    ///
    /// The modified `GetOrderHistoryParameters` instance.
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderHistoryResponse {
    ret_code: i32,
    ret_msg: String,
    result: OrderHistoryResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetOrderHistoryResponse {
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

    pub fn result(&self) -> &OrderHistoryResult {
        &self.result
    }

    pub fn set_result(&mut self, result: OrderHistoryResult) {
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
pub struct OrderHistoryResult {
    category: String,
    next_page_cursor: String,
    list: Vec<OrderHistory>
}
impl OrderHistoryResult {
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

    pub fn list(&self) -> &Vec<OrderHistory> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<OrderHistory>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderHistory {
    order_id: String,
    order_link_id: String,
    block_trade_id: String,
    symbol: String,
    #[serde(deserialize_with = "deserialize_f64")]
    price: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    qty: f64,
    side: String,
    is_leverage: String,
    position_idx: u64,
    order_status: String,
    cancel_type: String,
    reject_reason: String,
    #[serde(deserialize_with = "deserialize_option_f64")]
    avg_price: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    leaves_qty: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    leaves_value: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    cum_exec_qty: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    cum_exec_fee: Option<f64>,
    time_in_force: String,
    order_type: String,
    stop_order_type: String,
    order_iv: String,
    #[serde(deserialize_with = "deserialize_option_f64")]
    trigger_price: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    take_profit: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    stop_loss: Option<f64>,
    tp_trigger_by: String,
    sl_trigger_by: String,
    trigger_direction: f64,
    trigger_by: String,
    last_price_on_created: String,
    reduce_only: bool,
    close_on_trigger: bool,
    smp_type: String,
    smp_group: u64,
    smp_order_id: String,
    tpsl_mode: String,
    tp_limit_price: String,
    sl_limit_price: String,
    place_type: String,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    created_time: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    updated_time: u64,
}
impl OrderHistory {
    pub fn order_id(&self) -> &str {
        &self.order_id
    }

    pub fn set_order_id(&mut self, order_id: String) {
        self.order_id = order_id;
    }

    pub fn order_link_id(&self) -> &str {
        &self.order_link_id
    }

    pub fn set_order_link_id(&mut self, order_link_id: String) {
        self.order_link_id = order_link_id;
    }

    pub fn block_trade_id(&self) -> &str {
        &self.block_trade_id
    }

    pub fn set_block_trade_id(&mut self, block_trade_id: String) {
        self.block_trade_id = block_trade_id;
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn set_price(&mut self, price: f64) {
        self.price = price;
    }

    pub fn qty(&self) -> f64 {
        self.qty
    }

    pub fn set_qty(&mut self, qty: f64) {
        self.qty = qty;
    }

    pub fn side(&self) -> &str {
        &self.side
    }

    pub fn set_side(&mut self, side: String) {
        self.side = side;
    }

    pub fn is_leverage(&self) -> &str {
        &self.is_leverage
    }

    pub fn set_is_leverage(&mut self, is_leverage: String) {
        self.is_leverage = is_leverage;
    }

    pub fn position_idx(&self) -> u64 {
        self.position_idx
    }

    pub fn set_position_idx(&mut self, position_idx: u64) {
        self.position_idx = position_idx;
    }

    pub fn order_status(&self) -> &str {
        &self.order_status
    }

    pub fn set_order_status(&mut self, order_status: String) {
        self.order_status = order_status;
    }

    pub fn cancel_type(&self) -> &str {
        &self.cancel_type
    }

    pub fn set_cancel_type(&mut self, cancel_type: String) {
        self.cancel_type = cancel_type;
    }

    pub fn reject_reason(&self) -> &str {
        &self.reject_reason
    }

    pub fn set_reject_reason(&mut self, reject_reason: String) {
        self.reject_reason = reject_reason;
    }

    pub fn avg_price(&self) -> Option<f64> {
        self.avg_price
    }

    pub fn set_avg_price(&mut self, avg_price: f64) {
        self.avg_price = Some(avg_price);
    }

    pub fn leaves_qty(&self) -> Option<f64> {
        self.leaves_qty
    }

    pub fn set_leaves_qty(&mut self, leaves_qty: f64) {
        self.leaves_qty = Some(leaves_qty);
    }

    pub fn leaves_value(&self) -> Option<f64> {
        self.leaves_value
    }

    pub fn set_leaves_value(&mut self, leaves_value: f64) {
        self.leaves_value = Some(leaves_value);
    }

    pub fn cum_exec_qty(&self) -> Option<f64> {
        self.cum_exec_qty
    }

    pub fn set_cum_exec_qty(&mut self, cum_exec_qty: f64) {
        self.cum_exec_qty = Some(cum_exec_qty);
    }

    pub fn cum_exec_fee(&self) -> Option<f64> {
        self.cum_exec_fee
    }

    pub fn set_cum_exec_fee(&mut self, cum_exec_fee: f64) {
        self.cum_exec_fee = Some(cum_exec_fee);
    }

    pub fn time_in_force(&self) -> &str {
        &self.time_in_force
    }

    pub fn set_time_in_force(&mut self, time_in_force: String) {
        self.time_in_force = time_in_force;
    }

    pub fn order_type(&self) -> &str {
        &self.order_type
    }

    pub fn set_order_type(&mut self, order_type: String) {
        self.order_type = order_type;
    }

    pub fn stop_order_type(&self) -> &str {
        &self.stop_order_type
    }

    pub fn set_stop_order_type(&mut self, stop_order_type: String) {
        self.stop_order_type = stop_order_type;
    }

    pub fn order_iv(&self) -> &str {
        &self.order_iv
    }

    pub fn set_order_iv(&mut self, order_iv: String) {
        self.order_iv = order_iv;
    }

    pub fn trigger_price(&self) -> Option<f64> {
        self.trigger_price
    }

    pub fn set_trigger_price(&mut self, trigger_price: f64) {
        self.trigger_price = Some(trigger_price);
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

    pub fn tp_trigger_by(&self) -> &str {
        &self.tp_trigger_by
    }

    pub fn set_tp_trigger_by(&mut self, tp_trigger_by: String) {
        self.tp_trigger_by = tp_trigger_by;
    }

    pub fn sl_trigger_by(&self) -> &str {
        &self.sl_trigger_by
    }

    pub fn set_sl_trigger_by(&mut self, sl_trigger_by: String) {
        self.sl_trigger_by = sl_trigger_by;
    }

    pub fn trigger_direction(&self) -> f64 {
        self.trigger_direction
    }

    pub fn set_trigger_direction(&mut self, trigger_direction: f64) {
        self.trigger_direction = trigger_direction;
    }

    pub fn trigger_by(&self) -> &str {
        &self.trigger_by
    }

    pub fn set_trigger_by(&mut self, trigger_by: String) {
        self.trigger_by = trigger_by;
    }

    pub fn last_price_on_created(&self) -> &str {
        &self.last_price_on_created
    }

    pub fn set_last_price_on_created(&mut self, last_price_on_created: String) {
        self.last_price_on_created = last_price_on_created;
    }

    pub fn reduce_only(&self) -> bool {
        self.reduce_only
    }

    pub fn set_reduce_only(&mut self, reduce_only: bool) {
        self.reduce_only = reduce_only;
    }

    pub fn close_on_trigger(&self) -> bool {
        self.close_on_trigger
    }

    pub fn set_close_on_trigger(&mut self, close_on_trigger: bool) {
        self.close_on_trigger = close_on_trigger;
    }

    pub fn smp_type(&self) -> &str {
        &self.smp_type
    }

    pub fn set_smp_type(&mut self, smp_type: String) {
        self.smp_type = smp_type;
    }

    pub fn smp_group(&self) -> u64 {
        self.smp_group
    }

    pub fn set_smp_group(&mut self, smp_group: u64) {
        self.smp_group = smp_group;
    }

    pub fn smp_order_id(&self) -> &str {
        &self.smp_order_id
    }

    pub fn set_smp_order_id(&mut self, smp_order_id: String) {
        self.smp_order_id = smp_order_id;
    }

    pub fn tpsl_mode(&self) -> &str {
        &self.tpsl_mode
    }

    pub fn set_tpsl_mode(&mut self, tpsl_mode: String) {
        self.tpsl_mode = tpsl_mode;
    }

    pub fn tp_limit_price(&self) -> &str {
        &self.tp_limit_price
    }

    pub fn set_tp_limit_price(&mut self, tp_limit_price: String) {
        self.tp_limit_price = tp_limit_price;
    }

    pub fn sl_limit_price(&self) -> &str {
        &self.sl_limit_price
    }

    pub fn set_sl_limit_price(&mut self, sl_limit_price: String) {
        self.sl_limit_price = sl_limit_price;
    }

    pub fn place_type(&self) -> &str {
        &self.place_type
    }

    pub fn set_place_type(&mut self, place_type: String) {
        self.place_type = place_type;
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