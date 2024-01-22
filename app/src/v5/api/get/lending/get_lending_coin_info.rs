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

const PATH: &'static str = "/v5/lending/info";

impl BybitApi {
    pub async fn get_lending_coin_info(&self, params: GetLendingCoinInfoParameters) -> Result<GetLendingCoinInfoResponse> {
        self.get(PATH, Some(params), true).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLendingCoinInfoParameters {
    coin: Option<String>,
}

impl GetLendingCoinInfoParameters {
    pub fn new() -> Self {
        Self {
            coin: None
        }
    }

    pub fn with_coin(mut self, coin: String) -> Self {
        self.coin = Some(coin);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLendingCoinInfoResponse {
    ret_code: i32,
    ret_msg: String,
    result: LendingCoinInfoResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetLendingCoinInfoResponse {
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

    pub fn result(&self) -> &LendingCoinInfoResult {
        &self.result
    }

    pub fn set_result(&mut self, result: LendingCoinInfoResult) {
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
pub struct LendingCoinInfoResult {
    list: Vec<LendingCoinInfo>
}
impl LendingCoinInfoResult {
    pub fn list(&self) -> &Vec<LendingCoinInfo> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<LendingCoinInfo>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LendingCoinInfo {
    #[serde(deserialize_with = "deserialize_f64")]
    actual_apy: f64,
    coin: String,
    #[serde(deserialize_with = "deserialize_f64")]
    loan_to_pool_ratio: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    max_redeem_qty: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    min_purchase_qty: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    precision: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    rate: f64,
}

impl LendingCoinInfo {
    pub fn actual_apy(&self) -> f64 {
        self.actual_apy
    }

    pub fn set_actual_apy(&mut self, actual_apy: f64) {
        self.actual_apy = actual_apy;
    }

    pub fn coin(&self) -> &str {
        &self.coin
    }

    pub fn set_coin(&mut self, coin: String) {
        self.coin = coin;
    }

    pub fn loan_to_pool_ratio(&self) -> f64 {
        self.loan_to_pool_ratio
    }

    pub fn set_loan_to_pool_ratio(&mut self, loan_to_pool_ratio: f64) {
        self.loan_to_pool_ratio = loan_to_pool_ratio;
    }

    pub fn max_redeem_qty(&self) -> f64 {
        self.max_redeem_qty
    }

    pub fn set_max_redeem_qty(&mut self, max_redeem_qty: f64) {
        self.max_redeem_qty = max_redeem_qty;
    }

    pub fn min_purchase_qty(&self) -> f64 {
        self.min_purchase_qty
    }

    pub fn set_min_purchase_qty(&mut self, min_purchase_qty: f64) {
        self.min_purchase_qty = min_purchase_qty;
    }

    pub fn precision(&self) -> f64 {
        self.precision
    }

    pub fn set_precision(&mut self, precision: f64) {
        self.precision = precision;
    }

    pub fn rate(&self) -> f64 {
        self.rate
    }

    pub fn set_rate(&mut self, rate: f64) {
        self.rate = rate;
    }
}