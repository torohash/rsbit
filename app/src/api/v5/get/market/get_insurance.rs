use crate::{
    api::{
        BybitApi,
        v5::get::Get,
    },
    utils::{
        deserialize_f64,
        deserialize_string_to_u64,
    },
};

use serde::{
    Serialize,
    Deserialize,
};
use serde_json::Value;
use anyhow::Result;

const PATH: &'static str = "/v5/market/insurance";

impl BybitApi {
    /// Retrieves insurance information.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the request.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::api::{
    ///     v5::get::market::get_insurance::GetInsuranceParameters,
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = GetInsuranceParameters::new();
    ///     let response = api.get_insurance(params).await;
    ///     match response {
    ///         Ok(info) => {
    ///             // Handle the data
    ///         },
    ///         Err(err) => {
    ///             // Handle the error
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn get_insurance(&self, params: GetInsuranceParameters) -> Result<GetInsuranceResponse> {
        self.get(PATH, Some(params), false).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInsuranceParameters {
    coin: Option<String>
}

impl GetInsuranceParameters {
    /// Creates a new instance of `GetInsuranceParameters`.
    pub fn new() -> Self {
        Self {
            coin: None,
        }
    }

    /// Sets the coin for the insurance parameters.
    ///
    /// # Arguments
    ///
    /// * `coin` - A string representing the coin.
    ///
    /// # Returns
    ///
    /// The updated `GetInsuranceParameters` instance.
    pub fn with_coin(mut self, coin: String) -> Self {
        self.coin = Some(coin);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInsuranceResponse {
    ret_code: i32,
    ret_msg: String,
    result: InsuranceResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetInsuranceResponse {
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

    pub fn result(&self) -> &InsuranceResult {
        &self.result
    }

    pub fn set_result(&mut self, result: InsuranceResult) {
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
pub struct InsuranceResult {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    updated_time: u64,
    list: Vec<Insurance>
}
impl InsuranceResult {
    pub fn updated_time(&self) -> u64 {
        self.updated_time
    }

    pub fn set_updated_time(&mut self, updated_time: u64) {
        self.updated_time = updated_time;
    }
    
    pub fn list(&self) -> &Vec<Insurance> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<Insurance>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Insurance {
    coin: String,
    #[serde(deserialize_with = "deserialize_f64")]
    balance: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    value: f64,
}
impl Insurance {
    pub fn coin(&self) -> &str {
        &self.coin
    }

    pub fn set_coin(&mut self, coin: String) {
        self.coin = coin;
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }

    pub fn set_balance(&mut self, balance: f64) {
        self.balance = balance;
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }
}