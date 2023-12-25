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

const PATH: &'static str = "/v5/position/confirm-pending-mmr";

impl BybitApi {
    pub async fn confirm_new_risk_limit(&self, params: ConfirmNewRiskLimitParameters) -> Result<ConfirmNewRiskLimitResponse> {
        self.post(PATH, Some(params)).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ConfirmNewRiskLimitCategory {
    Linear,
    Inverse,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmNewRiskLimitParameters {
    category: ConfirmNewRiskLimitCategory,
    symbol: String,
}

impl ConfirmNewRiskLimitParameters {
    pub fn new(category: ConfirmNewRiskLimitCategory, symbol: String) -> Self {
        Self {
            category,
            symbol,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmNewRiskLimitResponse {
    ret_code: i32,
    ret_msg: String,
    result: Value,
    ret_ext_info: Value,
    time: u64,
}
impl ConfirmNewRiskLimitResponse {
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

    pub fn result(&self) -> &Value {
        &self.result
    }

    pub fn set_result(&mut self, result: Value) {
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