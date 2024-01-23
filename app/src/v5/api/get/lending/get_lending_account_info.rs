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

const PATH: &'static str = "/v5/lending/account";

impl BybitApi {
    pub async fn get_lending_account_info(&self, params: GetLendingAccountInfoParameters) -> Result<GetLendingAccountInfoResponse> {
        self.get(PATH, Some(params), true).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLendingAccountInfoParameters {
    coin: String,
}

impl GetLendingAccountInfoParameters {
    pub fn new(coin: String) -> Self {
        Self {
            coin
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLendingAccountInfoResponse {
    ret_code: i32,
    ret_msg: String,
    result: LendingAccountInfoResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetLendingAccountInfoResponse {
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

    pub fn result(&self) -> &LendingAccountInfoResult {
        &self.result
    }

    pub fn set_result(&mut self, result: LendingAccountInfoResult) {
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
pub struct LendingAccountInfoResult {
    coin: String,
    #[serde(deserialize_with = "deserialize_f64")]
    principal_interest: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    principal_qty: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    principal_total: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    quantity: f64,
}
impl LendingAccountInfoResult {
    pub fn coin(&self) -> &str {
        &self.coin
    }

    pub fn set_coin(&mut self, coin: String) {
        self.coin = coin;
    }

    pub fn principal_interest(&self) -> f64 {
        self.principal_interest
    }

    pub fn set_principal_interest(&mut self, principal_interest: f64) {
        self.principal_interest = principal_interest;
    }

    pub fn principal_qty(&self) -> f64 {
        self.principal_qty
    }

    pub fn set_principal_qty(&mut self, principal_qty: f64) {
        self.principal_qty = principal_qty;
    }

    pub fn principal_total(&self) -> f64 {
        self.principal_total
    }

    pub fn set_principal_total(&mut self, principal_total: f64) {
        self.principal_total = principal_total;
    }

    pub fn quantity(&self) -> f64 {
        self.quantity
    }

    pub fn set_quantity(&mut self, quantity: f64) {
        self.quantity = quantity;
    }
}