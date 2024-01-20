use crate::v5::api::{
    BybitApi,
    post::Post,
};

use serde::{
    Serialize,
    Deserialize,
};
use serde_json::Value;
use anyhow::Result;

const PATH: &'static str = "/v5/asset/withdraw/cancel";

impl BybitApi {
    pub async fn cancel_withdraw(&self, params: CancelWithdrawParameters) -> Result<CancelWithdrawResponse> {
        self.post(PATH, Some(params)).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelWithdrawParameters {
    id: String
}

impl CancelWithdrawParameters {
    pub fn new(id: String) -> Self {
        Self {
            id
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelWithdrawResponse {
    ret_code: i32,
    ret_msg: String,
    result: CancelWithdrawResult,
    ret_ext_info: Value,
    time: u64,
}
impl CancelWithdrawResponse {
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

    pub fn result(&self) -> &CancelWithdrawResult {
        &self.result
    }

    pub fn set_result(&mut self, result: CancelWithdrawResult) {
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
pub struct CancelWithdrawResult {
    status: u8,
}

impl CancelWithdrawResult {
    pub fn status(&self) -> u8 {
        self.status
    }

    pub fn set_status(&mut self, status: u8) {
        self.status = status;
    }
}
