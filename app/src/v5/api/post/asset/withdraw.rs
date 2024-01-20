use crate::{
    v5::api::{
        BybitApi,
        post::Post,
    },
    utils::serialize_as_string,
};

use serde::{
    Serialize,
    Deserialize,
};
use serde_json::Value;
use anyhow::Result;

const PATH: &'static str = "/v5/asset/withdraw/create";

impl BybitApi {
    pub async fn withdraw(&self, params: WithdrawParameters) -> Result<WithdrawResponse> {
        self.post(PATH, Some(params)).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawParameters {
    coin: String,
    chain: Option<String>,
    address: String,
    tag: Option<String>,
    #[serde(serialize_with = "serialize_as_string")]
    amount: f64,
    timestamp: u64,
    force_chain: Option<u8>,
    account_type: String,
    fee_type: Option<u8>,
    request_id: Option<String>,
}

impl WithdrawParameters {
    pub fn new(coin: String, address: String, amount: f64, timestamp: u64, account_type: String) -> Self {
        Self {
            coin,
            chain: None,
            address,
            tag: None,
            amount,
            timestamp,
            force_chain: None,
            account_type,
            fee_type: None,
            request_id: None,
        }
    }

    pub fn with_chain(mut self, chain: String) -> Self {
        self.chain = Some(chain);
        self
    }

    pub fn with_tag(mut self, tag: String) -> Self {
        self.tag = Some(tag);
        self
    }

    pub fn with_force_chain(mut self, force_chain: u8) -> Self {
        self.force_chain = Some(force_chain);
        self
    }

    pub fn with_fee_type(mut self, fee_type: u8) -> Self {
        self.fee_type = Some(fee_type);
        self
    }

    pub fn with_request_id(mut self, request_id: String) -> Self {
        self.request_id = Some(request_id);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawResponse {
    ret_code: i32,
    ret_msg: String,
    result: WithdrawResult,
    ret_ext_info: Value,
    time: u64,
}
impl WithdrawResponse {
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

    pub fn result(&self) -> &WithdrawResult {
        &self.result
    }

    pub fn set_result(&mut self, result: WithdrawResult) {
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
pub struct WithdrawResult {
    id: String,
}

impl WithdrawResult {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }
}
