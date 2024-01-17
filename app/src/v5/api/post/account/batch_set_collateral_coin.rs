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

const PATH: &'static str = "/v5/account/set-collateral-switch-batch";

impl BybitApi {
    pub async fn batch_set_collateral_coin(&self, params: BatchSetCollateralCoinParameters) -> Result<BatchSetCollateralCoinResponse> {
        self.post(PATH, Some(params)).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSetCollateralCoinParameters {
    request: Vec<BatchSetCollateralCoinRequestParameters>,
}

impl BatchSetCollateralCoinParameters {
    pub fn new(request: Vec<BatchSetCollateralCoinRequestParameters>) -> Self {
        Self {
            request
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSetCollateralCoinRequestParameters {
    coin: String,
    collateral_switch: String,
}

impl BatchSetCollateralCoinRequestParameters {
    pub fn new(coin: String, collateral_switch: String) -> Self {
        Self {
            coin,
            collateral_switch,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSetCollateralCoinResponse {
    ret_code: i32,
    ret_msg: String,
    result: BatchSetCollateralCoinResult,
    ret_ext_info: Value,
    time: u64,
}
impl BatchSetCollateralCoinResponse {
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

    pub fn result(&self) -> &BatchSetCollateralCoinResult {
        &self.result
    }

    pub fn set_result(&mut self, result: BatchSetCollateralCoinResult) {
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
pub struct BatchSetCollateralCoinResult {
    list: Vec<BatchSetCollateralCoin>
}

impl BatchSetCollateralCoinResult {
    pub fn list(&self) -> &Vec<BatchSetCollateralCoin> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<BatchSetCollateralCoin>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSetCollateralCoin {
    coin: String,
    collateral_switch: String,
}

impl BatchSetCollateralCoin {
    pub fn coin(&self) -> &str {
        &self.coin
    }

    pub fn set_coin(&mut self, coin: String) {
        self.coin = coin;
    }

    pub fn collateral_switch(&self) -> &str {
        &self.collateral_switch
    }

    pub fn set_collateral_switch(&mut self, collateral_switch: String) {
        self.collateral_switch = collateral_switch;
    }
}