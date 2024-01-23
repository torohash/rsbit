use crate::{
    v5::api::{
        BybitApi,
        post::Post,
    },
    utils::{
        serialize_as_string,
        deserialize_string_to_u64,
        deserialize_f64,
    },
};

use serde::{
    Serialize,
    Deserialize,
};
use serde_json::Value;
use anyhow::Result;

const PATH: &'static str = "/v5/lending/purchase";

impl BybitApi {
    pub async fn deposit_funds(&self, params: DepositFundsParameters) -> Result<DepositFundsResponse> {
        self.post(PATH, Some(params)).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositFundsParameters {
    coin: String,
    #[serde(serialize_with = "serialize_as_string")]
    quantity: f64,
    serial_no: Option<String>
}

impl DepositFundsParameters {
    pub fn new(coin: String, quantity: f64) -> Self {
        Self {
            coin,
            quantity,
            serial_no: None,
        }
    }

    pub fn with_serial_no(mut self, serial_no: String) -> Self {
        self.serial_no = Some(serial_no);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositFundsResponse {
    ret_code: i32,
    ret_msg: String,
    result: DepositFundsResult,
    ret_ext_info: Value,
    time: u64,
}
impl DepositFundsResponse {
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

    pub fn result(&self) -> &DepositFundsResult {
        &self.result
    }

    pub fn set_result(&mut self, result: DepositFundsResult) {
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
pub struct DepositFundsResult {
    coin: String,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    created_time: u64,
    order_id: String,
    #[serde(deserialize_with = "deserialize_f64")]
    quantity: f64,
    serial_no: String,
    status: String,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    updated_time: u64,
}

impl DepositFundsResult {
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
