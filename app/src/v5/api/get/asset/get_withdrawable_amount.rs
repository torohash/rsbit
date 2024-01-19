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

const PATH: &'static str = "/v5/asset/withdraw/withdrawable-amount";

impl BybitApi {
    pub async fn get_withdrawable_amount(&self, params: GetWithdrawableAmountParameters) -> Result<GetWithdrawableAmountResponse> {
        self.get(PATH, Some(params), true).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWithdrawableAmountParameters {
    coin: String,
}

impl GetWithdrawableAmountParameters {
    pub fn new(coin: String) -> Self {
        Self {
            coin,
        }
    }

    pub fn with_coin(mut self, coin: String) -> Self {
        self.coin = coin;
        self
    }

}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWithdrawableAmountResponse {
    ret_code: i32,
    ret_msg: String,
    result: WithdrawableAmountResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetWithdrawableAmountResponse {
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

    pub fn result(&self) -> &WithdrawableAmountResult {
        &self.result
    }

    pub fn set_result(&mut self, result: WithdrawableAmountResult) {
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
pub struct WithdrawableAmountResult {
    #[serde(deserialize_with = "deserialize_f64")]
    limit_amount_usd: f64,
    withdrawable_amount: WithdrawableAmount,
}
impl WithdrawableAmountResult {
    pub fn limit_amount_usd(&self) -> f64 {
        self.limit_amount_usd
    }

    pub fn set_limit_amount_usd(&mut self, limit_amount_usd: f64) {
        self.limit_amount_usd = limit_amount_usd;
    }

    pub fn withdrawable_amount(&self) -> &WithdrawableAmount {
        &self.withdrawable_amount
    }

    pub fn set_withdrawable_amount(&mut self, withdrawable_amount: WithdrawableAmount) {
        self.withdrawable_amount = withdrawable_amount;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawableAmount {
    #[serde(rename = "SPOT")]
    spot: Option<WithdrawableAmountDetail>,
    #[serde(rename = "FUND")]
    fund: Option<WithdrawableAmountDetail>,
}

impl WithdrawableAmount {
    pub fn spot(&self) -> &Option<WithdrawableAmountDetail> {
        &self.spot
    }

    pub fn set_spot(&mut self, spot: WithdrawableAmountDetail) {
        self.spot = Some(spot);
    }

    pub fn fund(&self) -> &Option<WithdrawableAmountDetail> {
        &self.fund
    }

    pub fn set_fund(&mut self, fund: WithdrawableAmountDetail) {
        self.fund = Some(fund);
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawableAmountDetail {
    coin: String,
    #[serde(deserialize_with = "deserialize_f64")]
    withdrawable_amount: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    available_balance: f64,
}

impl WithdrawableAmountDetail {
    pub fn coin(&self) -> &str {
        &self.coin
    }

    pub fn set_coin(&mut self, coin: String) {
        self.coin = coin;
    }

    pub fn withdrawable_amount(&self) -> f64 {
        self.withdrawable_amount
    }

    pub fn set_withdrawable_amount(&mut self, withdrawable_amount: f64) {
        self.withdrawable_amount = withdrawable_amount;
    }

    pub fn available_balance(&self) -> f64 {
        self.available_balance
    }

    pub fn set_available_balance(&mut self, available_balance: f64) {
        self.available_balance = available_balance;
    }
}