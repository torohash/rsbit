use crate::v5::api::{
    BybitApi,
    get::Get,
};

use serde::{
    Serialize,
    Deserialize,
};
use serde_json::Value;
use anyhow::Result;

const PATH: &'static str = "/v5/asset/deposit/query-sub-member-address";

impl BybitApi {
    pub async fn get_sub_deposit_address(&self, params: GetSubDepositAddressParameters) -> Result<GetSubDepositAddressResponse> {
        self.get(PATH, Some(params), true).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubDepositAddressParameters {
    coin: String,
    chain_type: String,
    sub_member_id: String,
}

impl GetSubDepositAddressParameters {
    pub fn new(coin: String, chain_type: String, sub_member_id: String) -> Self {
        Self {
            coin,
            chain_type,
            sub_member_id,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubDepositAddressResponse {
    ret_code: i32,
    ret_msg: String,
    result: SubDepositAddressResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetSubDepositAddressResponse {
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

    pub fn result(&self) -> &SubDepositAddressResult {
        &self.result
    }

    pub fn set_result(&mut self, result: SubDepositAddressResult) {
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
pub struct SubDepositAddressResult {
    coin: String,
    chains: SubDepositAddress
}
impl SubDepositAddressResult {
    pub fn coin(&self) -> &str {
        &self.coin
    }

    pub fn set_coin(&mut self, coin: String) {
        self.coin = coin;
    }

    pub fn chains(&self) -> &SubDepositAddress {
        &self.chains
    }

    pub fn set_chains(&mut self, chains: SubDepositAddress) {
        self.chains = chains;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubDepositAddress {
    chain_type: String,
    address_deposit: String,
    tag_deposit: String,
    chain: String,
    batch_release_limit: String,
}

impl SubDepositAddress {
    pub fn chain_type(&self) -> &str {
        &self.chain_type
    }

    pub fn set_chain_type(&mut self, chain_type: String) {
        self.chain_type = chain_type;
    }

    pub fn address_deposit(&self) -> &str {
        &self.address_deposit
    }

    pub fn set_address_deposit(&mut self, address_deposit: String) {
        self.address_deposit = address_deposit;
    }

    pub fn tag_deposit(&self) -> &str {
        &self.tag_deposit
    }

    pub fn set_tag_deposit(&mut self, tag_deposit: String) {
        self.tag_deposit = tag_deposit;
    }

    pub fn chain(&self) -> &str {
        &self.chain
    }

    pub fn set_chain(&mut self, chain: String) {
        self.chain = chain;
    }

    pub fn batch_release_limit(&self) -> &str {
        &self.batch_release_limit
    }

    pub fn set_batch_release_limit(&mut self, batch_release_limit: String) {
        self.batch_release_limit = batch_release_limit;
    }
}