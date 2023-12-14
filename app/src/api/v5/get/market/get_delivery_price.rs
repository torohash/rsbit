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

const PATH: &'static str = "/v5/market/delivery-price";

impl BybitApi {
    /// Retrieves the delivery price based on the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the delivery price request.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::api::{
    ///     v5::get::market::get_delivery_price::{
    ///         GetDeliveryPriceParameters,
    ///         GetDeliveryPriceCategory
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = GetDeliveryPriceParameters::new(GetDeliveryPriceCategory::Option);
    ///     let response = api.get_delivery_price(params).await;
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
    pub async fn get_delivery_price(&self, params: GetDeliveryPriceParameters) -> Result<GetDeliveryPriceResponse> {
        self.get(PATH, Some(params), false).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum GetDeliveryPriceCategory {
    Option,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDeliveryPriceParameters {
    category: GetDeliveryPriceCategory,
    symbol: Option<String>,
    base_coin: Option<String>,
    limit: Option<u32>,
    cursor: Option<String>,
}

impl GetDeliveryPriceParameters {
    /// Creates a new instance of `GetDeliveryPriceParameters` with the specified category.
    ///
    /// # Arguments
    ///
    /// * `category` - The category of the delivery price.
    ///
    /// # Returns
    ///
    /// A new instance of `GetDeliveryPriceParameters`.
    pub fn new(category: GetDeliveryPriceCategory) -> Self {
        Self {
            category,
            symbol: None,
            base_coin: None,
            limit: None,
            cursor: None,
        }
    }

    /// Sets the symbol for the delivery price.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The symbol to set.
    ///
    /// # Returns
    ///
    /// The modified `GetDeliveryPriceParameters` instance.
    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    /// Sets the base coin for the delivery price.
    ///
    /// # Arguments
    ///
    /// * `base_coin` - The base coin to set.
    ///
    /// # Returns
    ///
    /// The modified `GetDeliveryPriceParameters` instance.
    pub fn with_base_coin(mut self, base_coin: String) -> Self {
        self.base_coin = Some(base_coin);
        self
    }

    /// Sets the limit for the delivery price.
    ///
    /// # Arguments
    ///
    /// * `limit` - The limit to set.
    ///
    /// # Returns
    ///
    /// The modified `GetDeliveryPriceParameters` instance.
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the cursor for the delivery price.
    ///
    /// # Arguments
    ///
    /// * `cursor` - The cursor to set.
    ///
    /// # Returns
    ///
    /// The modified `GetDeliveryPriceParameters` instance.
    pub fn with_cursor(mut self, cursor: String) -> Self {
        self.cursor = Some(cursor);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDeliveryPriceResponse {
    ret_code: i32,
    ret_msg: String,
    result: DeliveryPriceResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetDeliveryPriceResponse {
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

    pub fn result(&self) -> &DeliveryPriceResult {
        &self.result
    }

    pub fn set_result(&mut self, result: DeliveryPriceResult) {
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
pub struct DeliveryPriceResult {
    category: String,
    next_page_cursor: String,
    list: Vec<DeliveryPrice>
}
impl DeliveryPriceResult {
    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn set_category(&mut self, category: String) {
        self.category = category;
    }

    pub fn next_page_cursor(&self) -> &str {
        &self.next_page_cursor
    }

    pub fn set_next_page_cursor(&mut self, next_page_cursor: String) {
        self.next_page_cursor = next_page_cursor;
    }

    pub fn list(&self) -> &Vec<DeliveryPrice> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<DeliveryPrice>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPrice {
    symbol: String,
    #[serde(deserialize_with = "deserialize_f64")]
    delivery_price: f64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    delivery_time: u64,
}
impl DeliveryPrice {
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn delivery_price(&self) -> f64 {
        self.delivery_price
    }

    pub fn set_delivery_price(&mut self, delivery_price: f64) {
        self.delivery_price = delivery_price;
    }

    pub fn delivery_time(&self) -> u64 {
        self.delivery_time
    }

    pub fn set_delivery_time(&mut self, delivery_time: u64) {
        self.delivery_time = delivery_time;
    }
}