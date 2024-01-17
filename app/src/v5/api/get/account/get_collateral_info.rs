use crate::{
    v5::api::{
        BybitApi,
        get::Get,
    },
    utils::{
        deserialize_f64,
        deserialize_option_f64,
    }
};

use serde::{
    Serialize,
    Deserialize,
};
use serde_json::Value;
use anyhow::Result;

const PATH: &'static str = "/v5/account/collateral-info";

impl BybitApi {
    pub async fn get_collateral_info(&self, params: GetCollateralInfoParameters) -> Result<GetCollateralInfoResponse> {
        self.get(PATH, Some(params), true).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCollateralInfoParameters {
    currency: Option<String>,
}

impl GetCollateralInfoParameters {
    pub fn new() -> Self {
        Self {
            currency: None,
        }
    }

    pub fn with_currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }

}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCollateralInfoResponse {
    ret_code: i32,
    ret_msg: String,
    result: CollateralInfoResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetCollateralInfoResponse {
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

    pub fn result(&self) -> &CollateralInfoResult {
        &self.result
    }

    pub fn set_result(&mut self, result: CollateralInfoResult) {
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
pub struct CollateralInfoResult {
    list: Vec<CollateralInfo>
}
impl CollateralInfoResult {
    pub fn list(&self) -> &Vec<CollateralInfo> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<CollateralInfo>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollateralInfo {
    #[serde(deserialize_with = "deserialize_f64")]
    available_to_borrow: f64,
    #[serde(deserialize_with = "deserialize_option_f64")]
    free_borrowing_amount: Option<f64>,
    #[serde(deserialize_with = "deserialize_f64")]
    free_borrow_amount: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    max_borrowing_amount: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    hourly_borrow_rate: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    borrow_usage_rate: f64,
    collateral_switch: bool,
    #[serde(deserialize_with = "deserialize_f64")]
    borrow_amount: f64,
    borrowable: bool,
    currency: String,
    margin_collateral: bool,
    #[serde(deserialize_with = "deserialize_f64")]
    free_borrowing_limit: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    collateral_ratio: f64,    
}

impl CollateralInfo {
    pub fn available_to_borrow(&self) -> f64 {
        self.available_to_borrow
    }

    pub fn set_available_to_borrow(&mut self, available_to_borrow: f64) {
        self.available_to_borrow = available_to_borrow;
    }

    pub fn free_borrowing_amount(&self) -> Option<f64> {
        self.free_borrowing_amount
    }

    pub fn set_free_borrowing_amount(&mut self, free_borrowing_amount: Option<f64>) {
        self.free_borrowing_amount = free_borrowing_amount;
    }

    pub fn free_borrow_amount(&self) -> f64 {
        self.free_borrow_amount
    }

    pub fn set_free_borrow_amount(&mut self, free_borrow_amount: f64) {
        self.free_borrow_amount = free_borrow_amount;
    }

    pub fn max_borrowing_amount(&self) -> f64 {
        self.max_borrowing_amount
    }

    pub fn set_max_borrowing_amount(&mut self, max_borrowing_amount: f64) {
        self.max_borrowing_amount = max_borrowing_amount;
    }

    pub fn hourly_borrow_rate(&self) -> f64 {
        self.hourly_borrow_rate
    }

    pub fn set_hourly_borrow_rate(&mut self, hourly_borrow_rate: f64) {
        self.hourly_borrow_rate = hourly_borrow_rate;
    }

    pub fn borrow_usage_rate(&self) -> f64 {
        self.borrow_usage_rate
    }

    pub fn set_borrow_usage_rate(&mut self, borrow_usage_rate: f64) {
        self.borrow_usage_rate = borrow_usage_rate;
    }

    pub fn collateral_switch(&self) -> bool {
        self.collateral_switch
    }

    pub fn set_collateral_switch(&mut self, collateral_switch: bool) {
        self.collateral_switch = collateral_switch;
    }

    pub fn borrow_amount(&self) -> f64 {
        self.borrow_amount
    }

    pub fn set_borrow_amount(&mut self, borrow_amount: f64) {
        self.borrow_amount = borrow_amount;
    }

    pub fn borrowable(&self) -> bool {
        self.borrowable
    }

    pub fn set_borrowable(&mut self, borrowable: bool) {
        self.borrowable = borrowable;
    }

    pub fn currency(&self) -> &str {
        &self.currency
    }

    pub fn set_currency(&mut self, currency: String) {
        self.currency = currency;
    }

    pub fn margin_collateral(&self) -> bool {
        self.margin_collateral
    }

    pub fn set_margin_collateral(&mut self, margin_collateral: bool) {
        self.margin_collateral = margin_collateral;
    }

    pub fn free_borrowing_limit(&self) -> f64 {
        self.free_borrowing_limit
    }

    pub fn set_free_borrowing_limit(&mut self, free_borrowing_limit: f64) {
        self.free_borrowing_limit = free_borrowing_limit;
    }

    pub fn collateral_ratio(&self) -> f64 {
        self.collateral_ratio
    }

    pub fn set_collateral_ratio(&mut self, collateral_ratio: f64) {
        self.collateral_ratio = collateral_ratio;
    }

}