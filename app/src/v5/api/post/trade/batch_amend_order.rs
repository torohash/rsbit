use crate::{
    v5::api::{
        BybitApi,
        post::Post,
    },
    utils::serialize_option_as_string,
};

use serde::{
    Serialize,
    Deserialize,
};
use serde_json::Value;
use anyhow::Result;

const PATH: &'static str = "/v5/order/create-batch";

impl BybitApi {
    /// batch amend orders.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for batch amend orders.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::v5::api::{
    ///     post::trade::{
    ///         batch_place_order::{
    ///             BatchPlaceOrderParameters,
    ///             BatchPlaceOrderRequestParameters,
    ///             BatchPlaceOrderCategory
    ///         },
    ///         batch_amend_order::{
    ///             BatchAmendOrderParameters,
    ///             BatchAmendOrderRequestParameters,
    ///             BatchAmendOrderCategory
    ///         },
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = BatchPlaceOrderParameters::new(
    ///         BatchPlaceOrderCategory::Linear,
    ///         vec![
    ///             BatchPlaceOrderRequestParameters::new("BTCUSDT".to_string(), "Buy".to_string(), "Limit".to_string(), 0.01).with_price(30000.0),
    ///             BatchPlaceOrderRequestParameters::new("BTCUSDT".to_string(), "Buy".to_string(), "Limit".to_string(), 0.01).with_price(30000.0),
    ///         ],
    ///     );
    ///     let response = api.batch_place_order(params).await;
    ///     match response {
    ///         Ok(info) => {
    ///             let params = BatchAmendOrderParameters::new(
    ///                 BatchAmendOrderCategory::Linear,
    ///                 vec![
    ///                     BatchAmendOrderRequestParameters::new("BTCUSDT".to_string()).with_order_id(info.result().list()[0].order_id().to_string()).with_qty(0.02),
    ///                     BatchAmendOrderRequestParameters::new("BTCUSDT".to_string()).with_order_id(info.result().list()[1].order_id().to_string()).with_qty(0.02),
    ///                 ],
    ///             );
    ///             let response = api.batch_amend_order(params).await;
    ///             match response {
    ///                 Ok(info) => {
    ///                     // Handle the data
    ///                 },
    ///                 Err(err) => {
    ///                     // Handle the error
    ///                 }
    ///             }
    ///         },
    ///         Err(err) => {
    ///             // Handle the error
    ///         }
    ///     }
    /// }
    pub async fn batch_amend_order(&self, params: BatchAmendOrderParameters) -> Result<BatchAmendOrderResponse> {
        self.post(PATH, Some(params)).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum BatchAmendOrderCategory {
    Linear,
    Option,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchAmendOrderParameters {
    category: BatchAmendOrderCategory,
    request: Vec<BatchAmendOrderRequestParameters>,
}

impl BatchAmendOrderParameters {
    pub fn new(category: BatchAmendOrderCategory, request: Vec<BatchAmendOrderRequestParameters>) -> Self {
        Self {
            category,
            request,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchAmendOrderRequestParameters {
    symbol: String,
    order_id: Option<String>,
    order_link_id: Option<String>,
    #[serde(serialize_with = "serialize_option_as_string")]
    order_iv: Option<f64>,
    trigger_price: Option<String>,
    #[serde(serialize_with = "serialize_option_as_string")]
    qty: Option<f64>,
    #[serde(serialize_with = "serialize_option_as_string")]
    price: Option<f64>,
    tpsl_mode: Option<String>,
    take_profit: Option<String>,
    stop_loss: Option<String>,
    tp_trigger_by: Option<String>,
    sl_trigger_by: Option<String>,
    trigger_by: Option<String>,
    tp_limit_price: Option<String>,
    sl_limit_price: Option<String>,
}

impl BatchAmendOrderRequestParameters {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            order_id: None,
            order_link_id: None,
            order_iv: None,
            trigger_price: None,
            qty: None,
            price: None,
            tpsl_mode: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            trigger_by: None,
            tp_limit_price: None,
            sl_limit_price: None,
        }
    }

    pub fn with_order_id(mut self, order_id: String) -> Self {
        self.order_id = Some(order_id);
        self
    }

    pub fn with_order_link_id(mut self, order_link_id: String) -> Self {
        self.order_link_id = Some(order_link_id);
        self
    }

    pub fn with_order_iv(mut self, order_iv: f64) -> Self {
        self.order_iv = Some(order_iv);
        self
    }

    pub fn with_trigger_price(mut self, trigger_price: String) -> Self {
        self.trigger_price = Some(trigger_price);
        self
    }

    pub fn with_qty(mut self, qty: f64) -> Self {
        self.qty = Some(qty);
        self
    }

    pub fn with_price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }

    pub fn with_tpsl_mode(mut self, tpsl_mode: String) -> Self {
        self.tpsl_mode = Some(tpsl_mode);
        self
    }

    pub fn with_take_profit(mut self, take_profit: String) -> Self {
        self.take_profit = Some(take_profit);
        self
    }

    pub fn with_stop_loss(mut self, stop_loss: String) -> Self {
        self.stop_loss = Some(stop_loss);
        self
    }

    pub fn with_tp_trigger_by(mut self, tp_trigger_by: String) -> Self {
        self.tp_trigger_by = Some(tp_trigger_by);
        self
    }

    pub fn with_sl_trigger_by(mut self, sl_trigger_by: String) -> Self {
        self.sl_trigger_by = Some(sl_trigger_by);
        self
    }

    pub fn with_trigger_by(mut self, trigger_by: String) -> Self {
        self.trigger_by = Some(trigger_by);
        self
    }

    pub fn with_tp_limit_price(mut self, tp_limit_price: String) -> Self {
        self.tp_limit_price = Some(tp_limit_price);
        self
    }

    pub fn with_sl_limit_price(mut self, sl_limit_price: String) -> Self {
        self.sl_limit_price = Some(sl_limit_price);
        self
    }

}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchAmendOrderResponse {
    ret_code: i32,
    ret_msg: String,
    result: BatchAmendOrderResult,
    ret_ext_info: Value,
    time: u64,
}
impl BatchAmendOrderResponse {
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

    pub fn result(&self) -> &BatchAmendOrderResult {
        &self.result
    }

    pub fn set_result(&mut self, result: BatchAmendOrderResult) {
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
pub struct BatchAmendOrderResult {
    list: Vec<BatchAmendOrder>,
}
impl BatchAmendOrderResult {
    pub fn list(&self) -> &Vec<BatchAmendOrder> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<BatchAmendOrder>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchAmendOrder {
    category: String,
    symbol: String,
    order_id: String,
    order_link_id: String,
}

impl BatchAmendOrder {
    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn set_category(&mut self, category: String) {
        self.category = category;
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn order_id(&self) -> &str {
        &self.order_id
    }

    pub fn set_order_id(&mut self, order_id: String) {
        self.order_id = order_id;
    }

    pub fn order_link_id(&self) -> &str {
        &self.order_link_id
    }

    pub fn set_order_link_id(&mut self, order_link_id: String) {
        self.order_link_id = order_link_id;
    }

}