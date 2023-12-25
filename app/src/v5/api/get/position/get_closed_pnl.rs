use crate::{
    v5::api::{
        BybitApi,
        get::Get,
    },
    utils::{
        deserialize_f64,
        deserialize_string_to_u64,
    },
};

use serde::{
    Serialize,
    Deserialize,
};
use serde_json::Value;
use anyhow::Result;

const PATH: &'static str = "/v5/position/closed-pnl";

impl BybitApi {
    /// Retrieves the closed pnl based on the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for retrieving closed pnl.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::v5::api::{
    ///     get::position::get_closed_pnl::{
    ///         GetClosedPnlParameters,
    ///         GetClosedPnlCategory
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = GetClosedPnlParameters::new(GetClosedPnlCategory::Linear);
    ///     let response = api.get_closed_pnl(params).await;
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
    pub async fn get_closed_pnl(&self, params: GetClosedPnlParameters) -> Result<GetClosedPnlResponse> {
        self.get(PATH, Some(params), true).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GetClosedPnlCategory {
    Linear,
    Inverse,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetClosedPnlParameters {
    category: GetClosedPnlCategory,
    symbol: Option<String>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    limit: Option<u32>,
    cursor: Option<String>,
}

impl GetClosedPnlParameters {
    /// Creates a new instance of `GetClosedPnlParameters` with the specified category.
    ///
    /// # Arguments
    ///
    /// * `category` - The category of the execution.
    ///
    /// # Returns
    ///
    /// A new instance of `GetClosedPnlParameters`.
    pub fn new(category: GetClosedPnlCategory) -> Self {
        Self {
            category,
            symbol: None,
            start_time: None,
            end_time: None,
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
pub struct GetClosedPnlResponse {
    ret_code: i32,
    ret_msg: String,
    result: ClosedPnlResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetClosedPnlResponse {
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

    pub fn result(&self) -> &ClosedPnlResult {
        &self.result
    }

    pub fn set_result(&mut self, result: ClosedPnlResult) {
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
pub struct ClosedPnlResult {
    category: String,
    next_page_cursor: String,
    list: Vec<ClosedPnl>
}
impl ClosedPnlResult {
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

    pub fn list(&self) -> &Vec<ClosedPnl> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<ClosedPnl>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosedPnl {
    symbol: String,
    order_type: String,
    leverage: String,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    updated_time: u64,
    side: String,
    order_id: String,
    #[serde(deserialize_with = "deserialize_f64")]
    closed_pnl: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    avg_entry_price: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    qty: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    cum_entry_value: f64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    created_time: u64,
    #[serde(deserialize_with = "deserialize_f64")]
    order_price: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    closed_size: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    avg_exit_price: f64,
    exec_type: String,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    fill_count: u64,
    #[serde(deserialize_with = "deserialize_f64")]
    cum_exit_value: f64,
}

impl ClosedPnl {
    
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

    pub fn leverage(&self) -> &str {
        &self.leverage
    }

    pub fn set_leverage(&mut self, leverage: String) {
        self.leverage = leverage;
    }

    pub fn updated_time(&self) -> u64 {
        self.updated_time
    }

    pub fn set_updated_time(&mut self, updated_time: u64) {
        self.updated_time = updated_time;
    }

    pub fn side(&self) -> &str {
        &self.side
    }

    pub fn set_side(&mut self, side: String) {
        self.side = side;
    }

    pub fn order_id(&self) -> &str {
        &self.order_id
    }

    pub fn set_order_id(&mut self, order_id: String) {
        self.order_id = order_id;
    }

    pub fn closed_pnl(&self) -> f64 {
        self.closed_pnl
    }

    pub fn set_closed_pnl(&mut self, closed_pnl: f64) {
        self.closed_pnl = closed_pnl;
    }

    pub fn avg_entry_price(&self) -> f64 {
        self.avg_entry_price
    }

    pub fn set_avg_entry_price(&mut self, avg_entry_price: f64) {
        self.avg_entry_price = avg_entry_price;
    }

    pub fn qty(&self) -> f64 {
        self.qty
    }

    pub fn set_qty(&mut self, qty: f64) {
        self.qty = qty;
    }

    pub fn cum_entry_value(&self) -> f64 {
        self.cum_entry_value
    }

    pub fn set_cum_entry_value(&mut self, cum_entry_value: f64) {
        self.cum_entry_value = cum_entry_value;
    }

    pub fn created_time(&self) -> u64 {
        self.created_time
    }

    pub fn set_created_time(&mut self, created_time: u64) {
        self.created_time = created_time;
    }

    pub fn order_price(&self) -> f64 {
        self.order_price
    }

    pub fn set_order_price(&mut self, order_price: f64) {
        self.order_price = order_price;
    }

    pub fn closed_size(&self) -> f64 {
        self.closed_size
    }

    pub fn set_closed_size(&mut self, closed_size: f64) {
        self.closed_size = closed_size;
    }

    pub fn avg_exit_price(&self) -> f64 {
        self.avg_exit_price
    }

    pub fn set_avg_exit_price(&mut self, avg_exit_price: f64) {
        self.avg_exit_price = avg_exit_price;
    }

    pub fn exec_type(&self) -> &str {
        &self.exec_type
    }

    pub fn set_exec_type(&mut self, exec_type: String) {
        self.exec_type = exec_type;
    }

    pub fn fill_count(&self) -> u64 {
        self.fill_count
    }

    pub fn set_fill_count(&mut self, fill_count: u64) {
        self.fill_count = fill_count;
    }

    pub fn cum_exit_value(&self) -> f64 {
        self.cum_exit_value
    }

    pub fn set_cum_exit_value(&mut self, cum_exit_value: f64) {
        self.cum_exit_value = cum_exit_value;
    }

}