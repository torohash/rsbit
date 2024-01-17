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

const PATH: &'static str = "/v5/account/fee-rate";

impl BybitApi {
    pub async fn get_fee_rate(&self, params: GetFeeRateParameters) -> Result<GetFeeRateResponse> {
        self.get(PATH, Some(params), true).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GetFeeRateCategory {
    Linear,
    Inverse,
    Spot,
    Option,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFeeRateParameters {
    category: GetFeeRateCategory,
    symbol: Option<String>,
    base_coin: Option<String>,
}

impl GetFeeRateParameters {
    pub fn new(category: GetFeeRateCategory) -> Self {
        Self {
            category,
            symbol: None,
            base_coin: None,
        }
    }

    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn with_base_coin(mut self, base_coin: String) -> Self {
        self.base_coin = Some(base_coin);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFeeRateResponse {
    ret_code: i32,
    ret_msg: String,
    result: FeeRateResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetFeeRateResponse {
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

    pub fn result(&self) -> &FeeRateResult {
        &self.result
    }

    pub fn set_result(&mut self, result: FeeRateResult) {
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
pub struct FeeRateResult {
    list: Vec<FeeRate>
}
impl FeeRateResult {
    pub fn list(&self) -> &Vec<FeeRate> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<FeeRate>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeRate {
    symbol: String,
    #[serde(deserialize_with = "deserialize_f64")]
    taker_fee_rate: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    maker_fee_rate: f64,
}
impl FeeRate {
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn taker_fee_rate(&self) -> f64 {
        self.taker_fee_rate
    }

    pub fn set_taker_fee_rate(&mut self, taker_fee_rate: f64) {
        self.taker_fee_rate = taker_fee_rate;
    }

    pub fn maker_fee_rate(&self) -> f64 {
        self.maker_fee_rate
    }

    pub fn set_maker_fee_rate(&mut self, maker_fee_rate: f64) {
        self.maker_fee_rate = maker_fee_rate;
    }
}