use crate::{
    v5::api::{
        BybitApi,
        get::Get,
    },
    utils::{
        deserialize_f64,
        deserialize_option_f64,
    },
};

use serde::{
    Serialize,
    Deserialize,
};
use serde_json::Value;
use anyhow::Result;

const PATH: &'static str = "/v5/asset/transfer/query-asset-info";

impl BybitApi {
    pub async fn get_asset_info(&self, params: GetAssetInfoParameters) -> Result<GetAssetInfoResponse> {
        self.get(PATH, Some(params), true).await
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum GetAssetInfoAccountType {
    SPOT,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAssetInfoParameters {
    account_type: GetAssetInfoAccountType,
    coin: Option<String>,
}

impl GetAssetInfoParameters {
    pub fn new(account_type: GetAssetInfoAccountType) -> Self {
        Self {
            account_type,
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
pub struct GetAssetInfoResponse {
    ret_code: i32,
    ret_msg: String,
    result: AssetInfoResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetAssetInfoResponse {
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

    pub fn result(&self) -> &AssetInfoResult {
        &self.result
    }

    pub fn set_result(&mut self, result: AssetInfoResult) {
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
pub struct AssetInfoResult {
    spot: AssetInfo
}
impl AssetInfoResult {
    pub fn spot(&self) -> &AssetInfo {
        &self.spot
    }

    pub fn set_spot(&mut self, spot: AssetInfo) {
        self.spot = spot;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetInfo {
    status: String,
    assets: Vec<Asset>,
}

impl AssetInfo {
    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn set_status(&mut self, status: String) {
        self.status = status;
    }

    pub fn assets(&self) -> &Vec<Asset> {
        &self.assets
    }

    pub fn set_assets(&mut self, assets: Vec<Asset>) {
        self.assets = assets;
    }

}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    coin: String,
    #[serde(deserialize_with = "deserialize_f64")]
    frozen: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    free: f64,
    #[serde(deserialize_with = "deserialize_option_f64")]
    withdraw: Option<f64>,
}

impl Asset {
    pub fn coin(&self) -> &str {
        &self.coin
    }

    pub fn set_coin(&mut self, coin: String) {
        self.coin = coin;
    }

    pub fn frozen(&self) -> f64 {
        self.frozen
    }

    pub fn set_frozen(&mut self, frozen: f64) {
        self.frozen = frozen;
    }

    pub fn free(&self) -> f64 {
        self.free
    }

    pub fn set_free(&mut self, free: f64) {
        self.free = free;
    }

    pub fn withdraw(&self) -> Option<f64> {
        self.withdraw
    }

    pub fn set_withdraw(&mut self, withdraw: Option<f64>) {
        self.withdraw = withdraw;
    }
}