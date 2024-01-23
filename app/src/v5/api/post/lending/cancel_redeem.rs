use crate::{
    v5::api::{
        BybitApi,
        post::Post,
    },
    utils::deserialize_string_to_u64,
};

use serde::{
    Serialize,
    Deserialize,
};
use serde_json::Value;
use anyhow::Result;

const PATH: &'static str = "/v5/lending/redeem-cancel";

impl BybitApi {
    pub async fn cancel_redeem(&self, params: CancelRedeemParameters) -> Result<CancelRedeemResponse> {
        self.post(PATH, Some(params)).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelRedeemParameters {
    coin: Option<String>,
    order_id: Option<String>,
    serial_no: Option<String>
}

impl CancelRedeemParameters {
    pub fn new() -> Self {
        Self {
            coin: None,
            order_id: None,
            serial_no: None,
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

    pub fn with_serial_no(mut self, serial_no: String) -> Self {
        self.serial_no = Some(serial_no);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelRedeemResponse {
    ret_code: i32,
    ret_msg: String,
    result: CancelRedeemResult,
    ret_ext_info: Value,
    time: u64,
}
impl CancelRedeemResponse {
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

    pub fn result(&self) -> &CancelRedeemResult {
        &self.result
    }

    pub fn set_result(&mut self, result: CancelRedeemResult) {
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
pub struct CancelRedeemResult {
    order_id: String,
    serial_no: String,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    updated_time: u64,
}

impl CancelRedeemResult {
    pub fn order_id(&self) -> &str {
        &self.order_id
    }
    
    pub fn set_order_id(&mut self, order_id: String) {
        self.order_id = order_id;
    }

    pub fn serial_no(&self) -> &str {
        &self.serial_no
    }

    pub fn set_serial_no(&mut self, serial_no: String) {
        self.serial_no = serial_no;
    }

    pub fn updated_time(&self) -> u64 {
        self.updated_time
    }

    pub fn set_updated_time(&mut self, updated_time: u64) {
        self.updated_time = updated_time;
    }
}
