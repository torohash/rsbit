use crate::{
    v5::api::{
        BybitApi,
        get::Get,
    },
    utils::deserialize_f64,
};

use serde::{
    Serialize,
    Deserialize,
};
use serde_json::Value;
use anyhow::Result;

const PATH: &'static str = "/v5/account/borrow-history";

impl BybitApi {
    pub async fn get_borrow_history(&self, params: GetBorrowHistoryParameters) -> Result<GetBorrowHistoryResponse> {
        self.get(PATH, Some(params), true).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBorrowHistoryParameters {
    currency: Option<String>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    limit: Option<i32>,
    cursor: Option<String>,
}

impl GetBorrowHistoryParameters {
    pub fn new() -> Self {
        Self {
            currency: None,
            start_time: None,
            end_time: None,
            limit: None,
            cursor: None,
        }
    }

    pub fn with_currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
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
pub struct GetBorrowHistoryResponse {
    ret_code: i32,
    ret_msg: String,
    result: BorrowHistoryResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetBorrowHistoryResponse {
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

    pub fn result(&self) -> &BorrowHistoryResult {
        &self.result
    }

    pub fn set_result(&mut self, result: BorrowHistoryResult) {
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
pub struct BorrowHistoryResult {
    next_page_cursor: Option<String>,
    list: Vec<BorrowHistory>
}
impl BorrowHistoryResult {
    pub fn next_page_cursor(&self) -> &Option<String> {
        &self.next_page_cursor
    }

    pub fn set_next_page_cursor(&mut self, next_page_cursor: Option<String>) {
        self.next_page_cursor = next_page_cursor;
    }

    pub fn list(&self) -> &Vec<BorrowHistory> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<BorrowHistory>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowHistory {
    #[serde(deserialize_with = "deserialize_f64")]
    borrow_amount: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    cost_exemption: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    free_borrowed_amount: f64,
    created_time: u64,
    #[serde(deserialize_with = "deserialize_f64")]
    interest_bearing_borrow_size: f64,
    currency: String,
    #[serde(deserialize_with = "deserialize_f64")]
    unrealised_loss: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    hourly_borrow_rate: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    borrow_cost: f64,
}

impl BorrowHistory {
    pub fn borrow_amount(&self) -> f64 {
        self.borrow_amount
    }

    pub fn set_borrow_amount(&mut self, borrow_amount: f64) {
        self.borrow_amount = borrow_amount;
    }

    pub fn cost_exemption(&self) -> f64 {
        self.cost_exemption
    }

    pub fn set_cost_exemption(&mut self, cost_exemption: f64) {
        self.cost_exemption = cost_exemption;
    }

    pub fn free_borrowed_amount(&self) -> f64 {
        self.free_borrowed_amount
    }

    pub fn set_free_borrowed_amount(&mut self, free_borrowed_amount: f64) {
        self.free_borrowed_amount = free_borrowed_amount;
    }

    pub fn created_time(&self) -> u64 {
        self.created_time
    }

    pub fn set_created_time(&mut self, created_time: u64) {
        self.created_time = created_time;
    }

    pub fn interest_bearing_borrow_size(&self) -> f64 {
        self.interest_bearing_borrow_size
    }

    pub fn set_interest_bearing_borrow_size(&mut self, interest_bearing_borrow_size: f64) {
        self.interest_bearing_borrow_size = interest_bearing_borrow_size;
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }

    pub fn set_currency(&mut self, currency: String) {
        self.currency = currency;
    }

    pub fn unrealised_loss(&self) -> f64 {
        self.unrealised_loss
    }

    pub fn set_unrealised_loss(&mut self, unrealised_loss: f64) {
        self.unrealised_loss = unrealised_loss;
    }

    pub fn hourly_borrow_rate(&self) -> f64 {
        self.hourly_borrow_rate
    }

    pub fn set_hourly_borrow_rate(&mut self, hourly_borrow_rate: f64) {
        self.hourly_borrow_rate = hourly_borrow_rate;
    }

    pub fn borrow_cost(&self) -> f64 {
        self.borrow_cost
    }

    pub fn set_borrow_cost(&mut self, borrow_cost: f64) {
        self.borrow_cost = borrow_cost;
    }

}