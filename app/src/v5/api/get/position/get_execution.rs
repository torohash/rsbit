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

const PATH: &'static str = "/v5/execution/list";

impl BybitApi {
    /// Retrieves the execution based on the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for retrieving execution.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::v5::api::{
    ///     get::position::get_execution::{
    ///         GetExecutionParameters,
    ///         GetExecutionCategory
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = GetExecutionParameters::new(GetExecutionCategory::Linear);
    ///     let response = api.get_execution(params).await;
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
    pub async fn get_execution(&self, params: GetExecutionParameters) -> Result<GetExecutionResponse> {
        self.get(PATH, Some(params), true).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GetExecutionCategory {
    Linear,
    Inverse,
    Option,
    Spot,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExecutionParameters {
    category: GetExecutionCategory,
    symbol: Option<String>,
    order_id: Option<String>,
    order_link_id: Option<String>,
    base_coin: Option<String>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    exec_type: Option<String>,
    limit: Option<u32>,
    cursor: Option<String>,
}

impl GetExecutionParameters {
    /// Creates a new instance of `GetExecutionParameters` with the specified category.
    ///
    /// # Arguments
    ///
    /// * `category` - The category of the execution.
    ///
    /// # Returns
    ///
    /// A new instance of `GetExecutionParameters`.
    pub fn new(category: GetExecutionCategory) -> Self {
        Self {
            category,
            symbol: None,
            order_id: None,
            order_link_id: None,
            base_coin: None,
            start_time: None,
            end_time: None,
            exec_type: None,
            limit: None,
            cursor: None,
        }
    }

    /// Sets the symbol for the execution parameters.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The symbol to set.
    ///
    /// # Returns
    ///
    /// The modified `GetExecutionParameters` instance.
    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    /// Sets the order id for the execution parameters.
    ///
    /// # Arguments
    ///
    /// * `order id` - The order id to set.
    ///
    /// # Returns
    ///
    /// The modified `GetExecutionParameters` instance.
    pub fn with_order_id(mut self, order_id: String) -> Self {
        self.order_id = Some(order_id);
        self
    }

    /// Sets the order link id for the execution parameters.
    ///
    /// # Arguments
    ///
    /// * `order link id` - The order link id to set.
    ///
    /// # Returns
    ///
    /// The modified `GetExecutionParameters` instance.
    pub fn with_order_link_id(mut self, order_link_id: String) -> Self {
        self.order_link_id = Some(order_link_id);
        self
    }

    /// Sets the base coin for the execution parameters.
    ///
    /// # Arguments
    ///
    /// * `base coin` - The base coin to set.
    ///
    /// # Returns
    ///
    /// The modified `GetExecutionParameters` instance.
    pub fn with_base_coin(mut self, base_coin: String) -> Self {
        self.base_coin = Some(base_coin);
        self
    }

    /// Sets the start time for the execution parameters.
    ///
    /// # Arguments
    ///
    /// * `start time` - The start time to set.
    ///
    /// # Returns
    ///
    /// The modified `GetExecutionParameters` instance.
    pub fn with_start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// Sets the end time for the execution parameters.
    ///
    /// # Arguments
    ///
    /// * `end time` - The end time to set.
    ///
    /// # Returns
    ///
    /// The modified `GetExecutionParameters` instance.
    pub fn with_end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// Sets the exec type for the execution parameters.
    ///
    /// # Arguments
    ///
    /// * `exec type` - The exec type to set.
    ///
    /// # Returns
    ///
    /// The modified `GetExecutionParameters` instance.
    pub fn with_exec_type(mut self, exec_type: String) -> Self {
        self.exec_type = Some(exec_type);
        self
    }

    /// Sets the limit for the execution parameters.
    ///
    /// # Arguments
    ///
    /// * `limit` - The limit to set.
    ///
    /// # Returns
    ///
    /// The modified `GetExecutionParameters` instance.
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the cursor for the execution parameters.
    ///
    /// # Arguments
    ///
    /// * `cursor` - The cursor to set.
    ///
    /// # Returns
    ///
    /// The modified `GetExecutionParameters` instance.
    pub fn with_cursor(mut self, cursor: String) -> Self {
        self.cursor = Some(cursor);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExecutionResponse {
    ret_code: i32,
    ret_msg: String,
    result: ExecutionResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetExecutionResponse {
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

    pub fn result(&self) -> &ExecutionResult {
        &self.result
    }

    pub fn set_result(&mut self, result: ExecutionResult) {
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
pub struct ExecutionResult {
    category: String,
    next_page_cursor: String,
    list: Vec<Execution>
}
impl ExecutionResult {
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

    pub fn list(&self) -> &Vec<Execution> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<Execution>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Execution {
    symbol: String,
    order_type: String,
    #[serde(deserialize_with = "deserialize_option_f64")]
    underlying_price: Option<f64>,
    order_link_id: Option<String>,
    side: String,
    #[serde(deserialize_with = "deserialize_option_f64")]
    index_price: Option<f64>,
    order_id: String,
    stop_order_type: String,
    leaves_qty: String,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    exec_time: u64,
    is_maker: bool,
    #[serde(deserialize_with = "deserialize_f64")]
    exec_fee: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    fee_rate: f64,
    exec_id: String,
    trade_iv: Option<String>,
    block_trade_id: Option<String>,
    #[serde(deserialize_with = "deserialize_f64")]
    mark_price: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    exec_price: f64,
    mark_iv: Option<String>,
    #[serde(deserialize_with = "deserialize_f64")]
    order_qty: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    order_price: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    exec_value: f64,
    exec_type: String,
    #[serde(deserialize_with = "deserialize_f64")]
    exec_qty: f64,
    #[serde(deserialize_with = "deserialize_option_f64")]
    closed_size: Option<f64>,
    seq: u64,
}

impl Execution {
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn order_type(&self) -> &str {
        &self.order_type
    }

    pub fn set_order_type(&mut self, order_type: String) {
        self.order_type = order_type;
    }

    pub fn underlying_price(&self) -> Option<f64> {
        self.underlying_price
    }

    pub fn set_underlying_price(&mut self, underlying_price: Option<f64>) {
        self.underlying_price = underlying_price;
    }

    pub fn order_link_id(&self) -> Option<&str> {
        self.order_link_id.as_deref()
    }

    pub fn set_order_link_id(&mut self, order_link_id: Option<String>) {
        self.order_link_id = order_link_id;
    }

    pub fn side(&self) -> &str {
        &self.side
    }

    pub fn set_side(&mut self, side: String) {
        self.side = side;
    }

    pub fn index_price(&self) -> Option<f64> {
        self.index_price
    }

    pub fn set_index_price(&mut self, index_price: Option<f64>) {
        self.index_price = index_price;
    }

    pub fn order_id(&self) -> &str {
        &self.order_id
    }

    pub fn set_order_id(&mut self, order_id: String) {
        self.order_id = order_id;
    }

    pub fn stop_order_type(&self) -> &str {
        &self.stop_order_type
    }

    pub fn set_stop_order_type(&mut self, stop_order_type: String) {
        self.stop_order_type = stop_order_type;
    }

    pub fn leaves_qty(&self) -> &str {
        &self.leaves_qty
    }

    pub fn set_leaves_qty(&mut self, leaves_qty: String) {
        self.leaves_qty = leaves_qty;
    }

    pub fn exec_time(&self) -> u64 {
        self.exec_time
    }

    pub fn set_exec_time(&mut self, exec_time: u64) {
        self.exec_time = exec_time;
    }

    pub fn is_maker(&self) -> bool {
        self.is_maker
    }

    pub fn set_is_maker(&mut self, is_maker: bool) {
        self.is_maker = is_maker;
    }

    pub fn exec_fee(&self) -> f64 {
        self.exec_fee
    }

    pub fn set_exec_fee(&mut self, exec_fee: f64) {
        self.exec_fee = exec_fee;
    }

    pub fn fee_rate(&self) -> f64 {
        self.fee_rate
    }

    pub fn set_fee_rate(&mut self, fee_rate: f64) {
        self.fee_rate = fee_rate;
    }

    pub fn exec_id(&self) -> &str {
        &self.exec_id
    }

    pub fn set_exec_id(&mut self, exec_id: String) {
        self.exec_id = exec_id;
    }

    pub fn trade_iv(&self) -> Option<&str> {
        self.trade_iv.as_deref()
    }

    pub fn set_trade_iv(&mut self, trade_iv: Option<String>) {
        self.trade_iv = trade_iv;
    }

    pub fn block_trade_id(&self) -> Option<&str> {
        self.block_trade_id.as_deref()
    }

    pub fn set_block_trade_id(&mut self, block_trade_id: Option<String>) {
        self.block_trade_id = block_trade_id;
    }

    pub fn mark_price(&self) -> f64 {
        self.mark_price
    }

    pub fn set_mark_price(&mut self, mark_price: f64) {
        self.mark_price = mark_price;
    }

    pub fn exec_price(&self) -> f64 {
        self.exec_price
    }

    pub fn set_exec_price(&mut self, exec_price: f64) {
        self.exec_price = exec_price;
    }

    pub fn mark_iv(&self) -> Option<&str> {
        self.mark_iv.as_deref()
    }

    pub fn set_mark_iv(&mut self, mark_iv: Option<String>) {
        self.mark_iv = mark_iv;
    }

    pub fn order_qty(&self) -> f64 {
        self.order_qty
    }

    pub fn set_order_qty(&mut self, order_qty: f64) {
        self.order_qty = order_qty;
    }

    pub fn order_price(&self) -> f64 {
        self.order_price
    }

    pub fn set_order_price(&mut self, order_price: f64) {
        self.order_price = order_price;
    }

    pub fn exec_value(&self) -> f64 {
        self.exec_value
    }

    pub fn set_exec_value(&mut self, exec_value: f64) {
        self.exec_value = exec_value;
    }

    pub fn exec_type(&self) -> &str {
        &self.exec_type
    }

    pub fn set_exec_type(&mut self, exec_type: String) {
        self.exec_type = exec_type;
    }

    pub fn exec_qty(&self) -> f64 {
        self.exec_qty
    }

    pub fn set_exec_qty(&mut self, exec_qty: f64) {
        self.exec_qty = exec_qty;
    }

    pub fn closed_size(&self) -> Option<f64> {
        self.closed_size
    }

    pub fn set_closed_size(&mut self, closed_size: Option<f64>) {
        self.closed_size = closed_size;
    }

    pub fn seq(&self) -> u64 {
        self.seq
    }

    pub fn set_seq(&mut self, seq: u64) {
        self.seq = seq;
    }

}