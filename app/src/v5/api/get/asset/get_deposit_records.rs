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

const PATH: &'static str = "/v5/asset/deposit/query-record";

impl BybitApi {
    pub async fn get_deposit_records(&self, params: GetDepositRecordsParameters) -> Result<GetDepositRecordsResponse> {
        self.get(PATH, Some(params), true).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepositRecordsParameters {
    coin: String,
    start_time: Option<u64>,
    end_time: Option<u64>,
    limit: Option<i32>,
    cursor: Option<String>,
}

impl GetDepositRecordsParameters {
    pub fn new(coin: String) -> Self {
        Self {
            coin,
            start_time: None,
            end_time: None,
            limit: None,
            cursor: None,
        }
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
pub struct GetDepositRecordsResponse {
    ret_code: i32,
    ret_msg: String,
    result: DepositRecordsResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetDepositRecordsResponse {
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

    pub fn result(&self) -> &DepositRecordsResult {
        &self.result
    }

    pub fn set_result(&mut self, result: DepositRecordsResult) {
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
pub struct DepositRecordsResult {
    rows: Vec<DepositRecord>,
    next_page_cursor: Option<String>,
}
impl DepositRecordsResult {
    pub fn rows(&self) -> &Vec<DepositRecord> {
        &self.rows
    }

    pub fn set_rows(&mut self, rows: Vec<DepositRecord>) {
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
pub struct DepositRecord {
    coin: String,
    chain: String,
    #[serde(deserialize_with = "deserialize_f64")]
    amount: f64,
    tx_id: String,
    status: i64,
    to_address: String,
    tag: String,
    #[serde(deserialize_with = "deserialize_option_f64")]
    deposit_fee: Option<f64>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    success_at: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    confirmations: u64,
    tx_index: String,
    block_hash: String,
    batch_release_limit: String,
    deposit_type: String,
}

impl DepositRecord {
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

    pub fn deposit_fee(&self) -> Option<f64> {
        self.deposit_fee
    }

    pub fn set_deposit_fee(&mut self, deposit_fee: Option<f64>) {
        self.deposit_fee = deposit_fee;
    }

    pub fn success_at(&self) -> u64 {
        self.success_at
    }

    pub fn set_success_at(&mut self, success_at: u64) {
        self.success_at = success_at;
    }

    pub fn confirmations(&self) -> u64 {
        self.confirmations
    }

    pub fn set_confirmations(&mut self, confirmations: u64) {
        self.confirmations = confirmations;
    }

    pub fn tx_index(&self) -> &str {
        &self.tx_index
    }

    pub fn set_tx_index(&mut self, tx_index: String) {
        self.tx_index = tx_index;
    }

    pub fn block_hash(&self) -> &str {
        &self.block_hash
    }

    pub fn set_block_hash(&mut self, block_hash: String) {
        self.block_hash = block_hash;
    }

    pub fn batch_release_limit(&self) -> &str {
        &self.batch_release_limit
    }

    pub fn set_batch_release_limit(&mut self, batch_release_limit: String) {
        self.batch_release_limit = batch_release_limit;
    }

    pub fn deposit_type(&self) -> &str {
        &self.deposit_type
    }

    pub fn set_deposit_type(&mut self, deposit_type: String) {
        self.deposit_type = deposit_type;
    }

}