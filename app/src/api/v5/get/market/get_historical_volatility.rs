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
use anyhow::Result;

const PATH: &'static str = "/v5/market/historical-volatility";

impl BybitApi {
    /// Retrieves historical volatility data from the API.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the historical volatility request.
    ///
    /// # Examples
    ///
    /// Retrieves the open interest data for the market.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the request.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::api::{
    ///     v5::get::market::get_historical_volatility::{
    ///         GetHistoricalVolatilityParameters,
    ///         GetHistoricalVolatilityCategory
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = GetHistoricalVolatilityParameters::new(GetHistoricalVolatilityCategory::Option);
    ///     let response = api.get_historical_volatility(params).await;
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
    pub async fn get_historical_volatility(&self, params: GetHistoricalVolatilityParameters) -> Result<GetHistoricalVolatilityResponse> {
        self.get(PATH, Some(params), false).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GetHistoricalVolatilityCategory {
    Option,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetHistoricalVolatilityParameters {
    category: GetHistoricalVolatilityCategory,
    base_coin: Option<String>,
    period: Option<u64>,
    start_time: Option<u64>,
    end_time: Option<u64>,
}

impl GetHistoricalVolatilityParameters {
    /// Creates a new instance of `GetHistoricalVolatilityParameters` with the specified category.
    ///
    /// # Arguments
    ///
    /// * `category` - The category of historical volatility.
    ///
    /// # Returns
    ///
    /// A new instance of `GetHistoricalVolatilityParameters`.
    pub fn new(category: GetHistoricalVolatilityCategory) -> Self {
        Self {
            category,
            base_coin: None,
            period: None,
            start_time: None,
            end_time: None,
        }
    }

    /// Sets the base coin for the historical volatility parameters.
    ///
    /// # Arguments
    ///
    /// * `base_coin` - The base coin symbol.
    ///
    /// # Returns
    ///
    /// The updated `GetHistoricalVolatilityParameters` instance.
    pub fn with_base_coin(mut self, base_coin: String) -> Self {
        self.base_coin = Some(base_coin);
        self
    }

    /// Sets the period for the historical volatility parameters.
    ///
    /// # Arguments
    ///
    /// * `period` - The period in seconds.
    ///
    /// # Returns
    ///
    /// The updated `GetHistoricalVolatilityParameters` instance.
    pub fn with_period(mut self, period: u64) -> Self {
        self.period = Some(period);
        self
    }

    /// Sets the start time for the historical volatility parameters.
    ///
    /// # Arguments
    ///
    /// * `start_time` - The start time in seconds.
    ///
    /// # Returns
    ///
    /// The updated `GetHistoricalVolatilityParameters` instance.
    pub fn with_start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// Sets the end time for the historical volatility parameters.
    ///
    /// # Arguments
    ///
    /// * `end_time` - The end time in seconds.
    ///
    /// # Returns
    ///
    /// The updated `GetHistoricalVolatilityParameters` instance.
    pub fn with_end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetHistoricalVolatilityResponse {
    ret_code: i32,
    ret_msg: String,
    category: String,
    result: Vec<HistoricalVolatility>,
}
impl GetHistoricalVolatilityResponse {
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

    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn set_category(&mut self, category: String) {
        self.category = category;
    }

    pub fn result(&self) -> &Vec<HistoricalVolatility> {
        &self.result
    }

    pub fn set_result(&mut self, result: Vec<HistoricalVolatility>) {
        self.result = result;
    }

}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalVolatility {
    period: u64,
    #[serde(deserialize_with = "deserialize_f64")]
    value: f64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    time: u64,
}
impl HistoricalVolatility {
    pub fn period(&self) -> u64 {
        self.period
    }

    pub fn set_period(&mut self, period: u64) {
        self.period = period;
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    pub fn time(&self) -> u64 {
        self.time
    }

    pub fn set_time(&mut self, time: u64) {
        self.time = time;
    }
}