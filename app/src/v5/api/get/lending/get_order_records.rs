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

const PATH: &'static str = "/v5/lending/history-order";

impl BybitApi {
    pub async fn get_order_records(&self, params: GetOrderRecordsParameters) -> Result<GetOrderRecordsResponse> {
        self.get(PATH, Some(params), true).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderRecordsParameters {
    coin: Option<String>,
    order_id: Option<String>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    limit: Option<i32>,
    order_type: Option<String>,
}

impl GetOrderRecordsParameters {
    pub fn new() -> Self {
        Self {
            coin: None,
            order_id: None,
            start_time: None,
            end_time: None,
            limit: None,
            order_type: None,
        }
    }

    pub fn with_coin(mut self, coin: String) -> Self {
        self.coin = Some(coin);
        self
    }

    pub fn with_order_id(mut self, order_id: String) -> Self {
        self.order_id = Some(order_id);
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

    pub fn with_order_type(mut self, order_type: String) -> Self {
        self.order_type = Some(order_type);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderRecordsResponse {
    ret_code: i32,
    ret_msg: String,
    result: OrderRecordsResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetOrderRecordsResponse {
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

    pub fn result(&self) -> &OrderRecordsResult {
        &self.result
    }

    pub fn set_result(&mut self, result: OrderRecordsResult) {
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
pub struct OrderRecordsResult {
    list: Vec<OrderRecord>
}
impl OrderRecordsResult {
    pub fn list(&self) -> &Vec<OrderRecord> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<OrderRecord>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderRecord {
    coin: String,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    created_time: u64,
    order_id: String,
    order_type: String,
    #[serde(deserialize_with = "deserialize_f64")]
    quantity: f64,
    serial_no: String,
    status: String,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    updated_time: u64,
}

impl OrderRecord {
    pub fn coin(&self) -> &str {
        &self.coin
    }

    pub fn set_coin(&mut self, coin: String) {
        self.coin = coin;
    }

    pub fn created_time(&self) -> u64 {
        self.created_time
    }

    pub fn set_created_time(&mut self, created_time: u64) {
        self.created_time = created_time;
    }

    pub fn order_id(&self) -> &str {
        &self.order_id
    }

    pub fn set_order_id(&mut self, order_id: String) {
        self.order_id = order_id;
    }

    pub fn order_type(&self) -> &str {
        &self.order_type
    }

    pub fn set_order_type(&mut self, order_type: String) {
        self.order_type = order_type;
    }

    pub fn quantity(&self) -> f64 {
        self.quantity
    }

    pub fn set_quantity(&mut self, quantity: f64) {
        self.quantity = quantity;
    }

    pub fn serial_no(&self) -> &str {
        &self.serial_no
    }

    pub fn set_serial_no(&mut self, serial_no: String) {
        self.serial_no = serial_no;
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn set_status(&mut self, status: String) {
        self.status = status;
    }

    pub fn updated_time(&self) -> u64 {
        self.updated_time
    }

    pub fn set_updated_time(&mut self, updated_time: u64) {
        self.updated_time = updated_time;
    }
}