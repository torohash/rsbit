pub mod inverse;
pub mod linear;
pub mod option;
pub mod spot;

use self::{
    inverse::InverseTickersResult,
    linear::LinearTickersResult,
    spot::SpotTickersResult,
    option::OptionTickersResult,
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

const PATH: &'static str = "/v5/market/tickers";

impl BybitApi {
    /// Retrieves tickers from the market.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the tickers request.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::api::{
    ///     v5::get::market::get_tickers::{
    ///         GetTickersParameters,
    ///         GetTickersCategory,
    ///         TickersResult,
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = GetTickersParameters::new(GetTickersCategory::Linear);
    ///     let response = api.get_tickers(params).await;
    ///     match response {
    ///         Ok(result) => {
    ///             match result.result() {
    ///                 TickersResult::Linear(tickers) => {} // Handle linear tickers,
    ///                 _ => {} // Handle something else,
    ///             }
    ///         },
    ///         Err(err) => {
    ///             // Handle the error
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn get_tickers(&self, params: GetTickersParameters) -> Result<GetTickersResponse> {
        self.get(PATH, Some(params), false).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GetTickersCategory {
    Linear,
    Option,
    Spot,
    Inverse,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetTickersParameters {
    category: GetTickersCategory,
    symbol: Option<String>,
    base_coin: Option<String>,
    exp_date: Option<String>,
}

impl GetTickersParameters {
    /// Creates a new instance of `GetTickersParameters` with the specified category.
    ///
    /// # Arguments
    ///
    /// * `category` - The category of the tickers.
    ///
    /// # Returns
    ///
    /// A new instance of `GetTickersParameters`.
    pub fn new(category: GetTickersCategory) -> Self {
        Self {
            category,
            symbol: None,
            base_coin: None,
            exp_date: None
        }
    }

    /// Sets the symbol for the `GetTickersParameters`.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The symbol to set.
    ///
    /// # Returns
    ///
    /// The modified `GetTickersParameters` instance.
    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    /// Sets the base coin for the `GetTickersParameters`.
    ///
    /// # Arguments
    ///
    /// * `base_coin` - The base coin to set.
    ///
    /// # Returns
    ///
    /// The modified `GetTickersParameters` instance.
    pub fn with_base_coin(mut self, base_coin: String) -> Self {
        self.base_coin = Some(base_coin);
        self
    }

    /// Sets the expiration date for the `GetTickersParameters`.
    ///
    /// # Arguments
    ///
    /// * `exp_date` - The expiration date to set.
    ///
    /// # Returns
    ///
    /// The modified `GetTickersParameters` instance.
    pub fn with_exp_date(mut self, exp_date: String) -> Self {
        self.exp_date = Some(exp_date);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTickersResponse {
    ret_code: i32,
    ret_msg: String,
    result: TickersResult,
    ret_ext_info: Value,
    time: u64,
}

impl GetTickersResponse {
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

    pub fn result(&self) -> &TickersResult {
        &self.result
    }

    pub fn set_result(&mut self, result: TickersResult) {
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
pub enum TickersResult {
    Linear(LinearTickersResult),
    Option(OptionTickersResult),
    Spot(SpotTickersResult),
    Inverse(InverseTickersResult),
}

impl TickersResult {
    pub fn linear(&self) -> Option<&LinearTickersResult> {
        match self {
            TickersResult::Linear(data) => Some(data),
            _ => None,
        }
    }

    pub fn option(&self) -> Option<&OptionTickersResult> {
        match self {
            TickersResult::Option(data) => Some(data),
            _ => None,
        }
    }

    pub fn inverse(&self) -> Option<&InverseTickersResult> {
        match self {
            TickersResult::Inverse(data) => Some(data),
            _ => None,
        }
    }

    pub fn spot(&self) -> Option<&SpotTickersResult> {
        match self {
            TickersResult::Spot(data) => Some(data),
            _ => None,
        }
    }

    pub fn category(&self) -> &str {
        match self {
            TickersResult::Linear(_) => CATEGORY_LINEAR,
            TickersResult::Option(_) => CATEGORY_OPTION,
            TickersResult::Inverse(_) => CATEGORY_INVERSE,
            TickersResult::Spot(_) => CATEGORY_SPOT,
        }
    }
}

