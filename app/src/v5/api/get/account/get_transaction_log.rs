use crate::{
    v5::api::{
        BybitApi,
        get::Get,
    },
    utils::{
        deserialize_f64,
        deserialize_option_f64,
    },
};

use serde::{
    Serialize,
    Deserialize,
};
use serde_json::Value;
use anyhow::Result;

const PATH: &'static str = "/v5/account/transaction-log";

impl BybitApi {
    pub async fn get_transaction_log(&self, params: GetTransactionLogParameters) -> Result<GetTransactionLogResponse> {
        self.get(PATH, Some(params), true).await
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum GetTransactionLogAccountType {
    UNIFIED,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GetTransactionLogCategory {
    Linear,
    Spot,
    Option,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionLogParameters {
    account_type: Option<GetTransactionLogAccountType>,
    category: Option<GetTransactionLogCategory>,
    currency: Option<String>,
    base_coin: Option<String>,
    #[serde(rename = "type")]
    type_field: Option<String>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    limit: Option<i32>,
    cursor: Option<String>,
}

impl GetTransactionLogParameters {
    pub fn new() -> Self {
        Self {
            account_type: None,
            category: None,
            currency: None,
            base_coin: None,
            type_field: None,
            start_time: None,
            end_time: None,
            limit: None,
            cursor: None,
        }
    }

    pub fn with_account_type(mut self, account_type: GetTransactionLogAccountType) -> Self {
        self.account_type = Some(account_type);
        self
    }

    pub fn with_category(mut self, category: GetTransactionLogCategory) -> Self {
        self.category = Some(category);
        self
    }

    pub fn with_currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }

    pub fn with_base_coin(mut self, base_coin: String) -> Self {
        self.base_coin = Some(base_coin);
        self
    }

    pub fn with_type_field(mut self, type_field: String) -> Self {
        self.type_field = Some(type_field);
        self
    }

    pub fn with_start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    pub fn with_end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    pub fn with_limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_cursor(mut self, cursor: String) -> Self {
        self.cursor = Some(cursor);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionLogResponse {
    ret_code: i32,
    ret_msg: String,
    result: TransactionLogResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetTransactionLogResponse {
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

    pub fn result(&self) -> &TransactionLogResult {
        &self.result
    }

    pub fn set_result(&mut self, result: TransactionLogResult) {
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
pub struct TransactionLogResult {
    next_page_cursor: Option<String>,
    list: Vec<TransactionLog>
}
impl TransactionLogResult {
    pub fn next_page_cursor(&self) -> &Option<String> {
        &self.next_page_cursor
    }

    pub fn set_next_page_cursor(&mut self, next_page_cursor: Option<String>) {
        self.next_page_cursor = next_page_cursor;
    }
    
    pub fn list(&self) -> &Vec<TransactionLog> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<TransactionLog>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionLog {
    id: String,
    symbol: String,
    side: String,
    #[serde(deserialize_with = "deserialize_f64")]
    funding: f64,
    order_link_id: String,
    order_id: String,
    #[serde(deserialize_with = "deserialize_f64")]
    fee: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    change: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    cash_flow: f64,
    transaction_time: String,
    #[serde(rename = "type")]
    type_field: String,
    #[serde(deserialize_with = "deserialize_f64")]
    fee_rate: f64,
    #[serde(deserialize_with = "deserialize_option_f64")]
    bonus_change: Option<f64>,
    #[serde(deserialize_with = "deserialize_f64")]
    size: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    qty: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    cash_balance: f64,
    currency: String,
    category: String,
    #[serde(deserialize_with = "deserialize_f64")]
    trade_price: f64,
    trade_id: String,
}

impl TransactionLog {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn side(&self) -> &str {
        &self.side
    }

    pub fn set_side(&mut self, side: String) {
        self.side = side;
    }

    pub fn funding(&self) -> f64 {
        self.funding
    }

    pub fn set_funding(&mut self, funding: f64) {
        self.funding = funding;
    }

    pub fn order_link_id(&self) -> &str {
        &self.order_link_id
    }

    pub fn set_order_link_id(&mut self, order_link_id: String) {
        self.order_link_id = order_link_id;
    }

    pub fn order_id(&self) -> &str {
        &self.order_id
    }

    pub fn set_order_id(&mut self, order_id: String) {
        self.order_id = order_id;
    }

    pub fn fee(&self) -> f64 {
        self.fee
    }

    pub fn set_fee(&mut self, fee: f64) {
        self.fee = fee;
    }

    pub fn change(&self) -> f64 {
        self.change
    }

    pub fn set_change(&mut self, change: f64) {
        self.change = change;
    }

    pub fn cash_flow(&self) -> f64 {
        self.cash_flow
    }

    pub fn set_cash_flow(&mut self, cash_flow: f64) {
        self.cash_flow = cash_flow;
    }

    pub fn transaction_time(&self) -> &str {
        &self.transaction_time
    }

    pub fn set_transaction_time(&mut self, transaction_time: String) {
        self.transaction_time = transaction_time;
    }

    pub fn type_field(&self) -> &str {
        &self.type_field
    }

    pub fn set_type_field(&mut self, type_field: String) {
        self.type_field = type_field;
    }

    pub fn fee_rate(&self) -> f64 {
        self.fee_rate
    }

    pub fn set_fee_rate(&mut self, fee_rate: f64) {
        self.fee_rate = fee_rate;
    }

    pub fn bonus_change(&self) -> Option<f64> {
        self.bonus_change
    }

    pub fn set_bonus_change(&mut self, bonus_change: f64) {
        self.bonus_change = Some(bonus_change);
    }

    pub fn size(&self) -> f64 {
        self.size
    }

    pub fn set_size(&mut self, size: f64) {
        self.size = size;
    }

    pub fn qty(&self) -> f64 {
        self.qty
    }

    pub fn set_qty(&mut self, qty: f64) {
        self.qty = qty;
    }

    pub fn cash_balance(&self) -> f64 {
        self.cash_balance
    }

    pub fn set_cash_balance(&mut self, cash_balance: f64) {
        self.cash_balance = cash_balance;
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }

    pub fn set_currency(&mut self, currency: String) {
        self.currency = currency;
    }

    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn set_category(&mut self, category: String) {
        self.category = category;
    }

    pub fn trade_price(&self) -> f64 {
        self.trade_price
    }

    pub fn set_trade_price(&mut self, trade_price: f64) {
        self.trade_price = trade_price;
    }

    pub fn trade_id(&self) -> &str {
        &self.trade_id
    }

    pub fn set_trade_id(&mut self, trade_id: String) {
        self.trade_id = trade_id;
    }
    
}