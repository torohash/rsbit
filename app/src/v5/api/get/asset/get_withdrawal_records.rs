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

const PATH: &'static str = "/v5/asset/withdraw/query-record";

impl BybitApi {
    pub async fn get_withdrawal_records(&self, params: GetWithdrawalRecordsParameters) -> Result<GetWithdrawalRecordsResponse> {
        self.get(PATH, Some(params), true).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWithdrawalRecordsParameters {
    withdraw_id: Option<String>,
    tx_id: Option<String>,
    coin: Option<String>,
    withdraw_type: Option<String>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    limit: Option<i32>,
    cursor: Option<String>,
}

impl GetWithdrawalRecordsParameters {
    pub fn new() -> Self {
        Self {
            withdraw_id: None,
            tx_id: None,
            coin: None,
            withdraw_type: None,
            start_time: None,
            end_time: None,
            limit: None,
            cursor: None,
        }
    }

    pub fn with_withdraw_id(mut self, withdraw_id: String) -> Self {
        self.withdraw_id = Some(withdraw_id);
        self
    }

    pub fn with_tx_id(mut self, tx_id: String) -> Self {
        self.tx_id = Some(tx_id);
        self
    }

    pub fn with_coin(mut self, coin: String) -> Self {
        self.coin = Some(coin);
        self
    }

    pub fn with_withdraw_type(mut self, withdraw_type: String) -> Self {
        self.withdraw_type = Some(withdraw_type);
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

    pub fn withdraw_id(&self) -> &Option<String> {
        &self.withdraw_id
    }

    pub fn set_withdraw_id(&mut self, withdraw_id: Option<String>) {
        self.withdraw_id = withdraw_id;
    }

    pub fn tx_id(&self) -> &Option<String> {
        &self.tx_id
    }

    pub fn set_tx_id(&mut self, tx_id: Option<String>) {
        self.tx_id = tx_id;
    }

    pub fn coin(&self) -> &Option<String> {
        &self.coin
    }

    pub fn set_coin(&mut self, coin: Option<String>) {
        self.coin = coin;
    }

    pub fn withdraw_type(&self) -> &Option<String> {
        &self.withdraw_type
    }

    pub fn set_withdraw_type(&mut self, withdraw_type: Option<String>) {
        self.withdraw_type = withdraw_type;
    }

    pub fn start_time(&self) -> &Option<u64> {
        &self.start_time
    }

    pub fn set_start_time(&mut self, start_time: Option<u64>) {
        self.start_time = start_time;
    }

    pub fn end_time(&self) -> &Option<u64> {
        &self.end_time
    }

    pub fn set_end_time(&mut self, end_time: Option<u64>) {
        self.end_time = end_time;
    }

    pub fn limit(&self) -> &Option<i32> {
        &self.limit
    }

    pub fn set_limit(&mut self, limit: Option<i32>) {
        self.limit = limit;
    }

    pub fn cursor(&self) -> &Option<String> {
        &self.cursor
    }

    pub fn set_cursor(&mut self, cursor: Option<String>) {
        self.cursor = cursor;
    }

}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWithdrawalRecordsResponse {
    ret_code: i32,
    ret_msg: String,
    result: WithdrawalRecordsResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetWithdrawalRecordsResponse {
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

    pub fn result(&self) -> &WithdrawalRecordsResult {
        &self.result
    }

    pub fn set_result(&mut self, result: WithdrawalRecordsResult) {
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
pub struct WithdrawalRecordsResult {
    rows: Vec<WithdrawalRecord>,
    next_page_cursor: Option<String>,
}
impl WithdrawalRecordsResult {
    pub fn rows(&self) -> &Vec<WithdrawalRecord> {
        &self.rows
    }

    pub fn set_rows(&mut self, rows: Vec<WithdrawalRecord>) {
        self.rows = rows;
    }

    pub fn next_page_cursor(&self) -> &Option<String> {
        &self.next_page_cursor
    }

    pub fn set_next_page_cursor(&mut self, next_page_cursor: Option<String>) {
        self.next_page_cursor = next_page_cursor;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalRecord {
    coin: String,
    chain: String,
    #[serde(deserialize_with = "deserialize_f64")]
    amount: f64,
    tx_id: String,
    status: i64,
    to_address: String,
    tag: String,
    #[serde(deserialize_with = "deserialize_option_f64")]
    withdraw_fee: Option<f64>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    create_time: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    update_time: u64,
    withdraw_id: String,
    withdraw_type: i32,
}

impl WithdrawalRecord {
    pub fn coin(&self) -> &str {
        &self.coin
    }

    pub fn set_coin(&mut self, coin: String) {
        self.coin = coin;
    }

    pub fn chain(&self) -> &str {
        &self.chain
    }

    pub fn set_chain(&mut self, chain: String) {
        self.chain = chain;
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn set_amount(&mut self, amount: f64) {
        self.amount = amount;
    }

    pub fn tx_id(&self) -> &str {
        &self.tx_id
    }

    pub fn set_tx_id(&mut self, tx_id: String) {
        self.tx_id = tx_id;
    }

    pub fn status(&self) -> i64 {
        self.status
    }

    pub fn set_status(&mut self, status: i64) {
        self.status = status;
    }

    pub fn to_address(&self) -> &str {
        &self.to_address
    }

    pub fn set_to_address(&mut self, to_address: String) {
        self.to_address = to_address;
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn set_tag(&mut self, tag: String) {
        self.tag = tag;
    }

    pub fn withdraw_fee(&self) -> &Option<f64> {
        &self.withdraw_fee
    }

    pub fn set_withdraw_fee(&mut self, withdraw_fee: Option<f64>) {
        self.withdraw_fee = withdraw_fee;
    }

    pub fn create_time(&self) -> u64 {
        self.create_time
    }

    pub fn set_create_time(&mut self, create_time: u64) {
        self.create_time = create_time;
    }

    pub fn update_time(&self) -> u64 {
        self.update_time
    }

    pub fn set_update_time(&mut self, update_time: u64) {
        self.update_time = update_time;
    }

    pub fn withdraw_id(&self) -> &str {
        &self.withdraw_id
    }

    pub fn set_withdraw_id(&mut self, withdraw_id: String) {
        self.withdraw_id = withdraw_id;
    }

    pub fn withdraw_type(&self) -> i32 {
        self.withdraw_type
    }

    pub fn set_withdraw_type(&mut self, withdraw_type: i32) {
        self.withdraw_type = withdraw_type;
    }

}