pub mod inverse;
pub mod linear;
pub mod option;
pub mod spot;

use self::{
    inverse::InverseInstrumentsInfoResult,
    linear::LinearInstrumentsInfoResult,
    spot::SpotInstrumentsInfoResult,
    option::OptionInstrumentsInfoResult,
};
use crate::{
    api::{
        BybitApi,
        v5::get::Get,
    },
    constants::{
        CATEGORY_INVERSE,
        CATEGORY_OPTION,
        CATEGORY_LINEAR,
        CATEGORY_SPOT,
    }
};
use anyhow::Result;
use serde::{
    Deserialize,
    Serialize,
};
use serde_json::Value;

const PATH: &'static str = "/v5/market/instruments-info";

impl BybitApi {
    /// Retrieves information about instruments.
    ///
    /// This method sends a request to the server to retrieve information about instruments based on the provided parameters.
    /// It returns a `Result` containing a `GetInstrumentsInfoResponse` if the request is successful, or an error if the request fails.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for retrieving instrument information.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::api::{
    ///     v5::get::market::get_instruments_info::{
    ///         GetInstrumentsInfoParameters,
    ///         GetInstrumentsInfoCategory,
    ///         InstrumentsInfoResult,
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = GetInstrumentsInfoParameters::new(GetInstrumentsInfoCategory::Linear);
    ///     let response = api.get_instruments_info(params).await;
    ///     match response {
    ///         Ok(result) => {
    ///             match result.result() {
    ///                 InstrumentsInfoResult::Linear(info) => {} // Handle linear instrument info,
    ///                 _ => {} // Handle something else,
    ///             }
    ///         },
    ///         Err(err) => {
    ///             // Handle the error
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn get_instruments_info(&self, params: GetInstrumentsInfoParameters) -> Result<GetInstrumentsInfoResponse> {
        self.get(PATH, Some(params), false).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GetInstrumentsInfoCategory {
    Linear,
    Option,
    Spot,
    Inverse,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInstrumentsInfoParameters {
    category: GetInstrumentsInfoCategory,
    symbol: Option<String>,
    status: Option<String>,
    base_coin: Option<String>,
    limit: Option<u32>,
    cursor: Option<String>,
}

impl GetInstrumentsInfoParameters {
    /// Creates a new instance of `GetInstrumentsInfoParameters` with the specified category.
    ///
    /// # Arguments
    ///
    /// * `category`
    ///
    /// # Returns
    ///
    /// A new instance of `GetInstrumentsInfoParameters`.
    pub fn new(category: GetInstrumentsInfoCategory) -> Self {
        Self {
            category,
            symbol: None,
            status: None,
            base_coin: None,
            limit: None,
            cursor: None,
        }
    }

    /// Sets the symbol for the request.
    ///
    /// # Arguments
    ///
    /// * `symbol`
    ///
    /// # Returns
    ///
    /// The modified `GetInstrumentsInfoParameters` instance.
    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    /// Sets the status for the request.
    ///
    /// # Arguments
    ///
    /// * `status`
    ///
    /// # Returns
    ///
    /// The modified `GetInstrumentsInfoParameters` instance.
    pub fn with_status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }

    /// Sets the base coin for the request.
    ///
    /// # Arguments
    ///
    /// * `base_coin`
    ///
    /// # Returns
    ///
    /// The modified `GetInstrumentsInfoParameters` instance.
    pub fn with_base_coin(mut self, base_coin: String) -> Self {
        self.base_coin = Some(base_coin);
        self
    }

    /// Sets the limit for the request.
    ///
    /// # Arguments
    ///
    /// * `limit`
    ///
    /// # Returns
    ///
    /// The modified `GetInstrumentsInfoParameters` instance.
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the cursor for the request.
    ///
    /// # Arguments
    ///
    /// * `cursor`
    ///
    /// # Returns
    ///
    /// The modified `GetInstrumentsInfoParameters` instance.
    pub fn with_cursor(mut self, cursor: String) -> Self {
        self.cursor = Some(cursor);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInstrumentsInfoResponse {
    ret_code: i32,
    ret_msg: String,
    result: InstrumentsInfoResult,
    ret_ext_info: Value,
    time: u64,
}

impl GetInstrumentsInfoResponse {
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

    pub fn result(&self) -> &InstrumentsInfoResult {
        &self.result
    }

    pub fn set_result(&mut self, result: InstrumentsInfoResult) {
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
#[serde(tag = "category", rename_all = "camelCase")]
pub enum InstrumentsInfoResult {
    Linear(LinearInstrumentsInfoResult),
    Option(OptionInstrumentsInfoResult),
    Spot(SpotInstrumentsInfoResult),
    Inverse(InverseInstrumentsInfoResult),
}

impl InstrumentsInfoResult {
    pub fn linear(&self) -> Option<&LinearInstrumentsInfoResult> {
        match self {
            InstrumentsInfoResult::Linear(data) => Some(data),
            _ => None,
        }
    }

    pub fn option(&self) -> Option<&OptionInstrumentsInfoResult> {
        match self {
            InstrumentsInfoResult::Option(data) => Some(data),
            _ => None,
        }
    }

    pub fn inverse(&self) -> Option<&InverseInstrumentsInfoResult> {
        match self {
            InstrumentsInfoResult::Inverse(data) => Some(data),
            _ => None,
        }
    }

    pub fn spot(&self) -> Option<&SpotInstrumentsInfoResult> {
        match self {
            InstrumentsInfoResult::Spot(data) => Some(data),
            _ => None,
        }
    }

    pub fn category(&self) -> &str {
        match self {
            InstrumentsInfoResult::Linear(_) => CATEGORY_LINEAR,
            InstrumentsInfoResult::Option(_) => CATEGORY_OPTION,
            InstrumentsInfoResult::Inverse(_) => CATEGORY_INVERSE,
            InstrumentsInfoResult::Spot(_) => CATEGORY_SPOT,
        }
    }
}

