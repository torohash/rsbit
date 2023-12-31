use crate::{
    v5::api::{
        BybitApi,
        post::Post,
    },
    utils::{
        serialize_as_string,
        serialize_option_as_string,
        deserialize_string_to_u64
    },
};

use serde::{
    Serialize,
    Deserialize,
};
use serde_json::Value;
use anyhow::Result;

const PATH: &'static str = "/v5/order/create-batch";

impl BybitApi {
    /// batch place orders.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for batch place orders.
    ///
    /// # Examples
    /// 
    /// ```rust
    /// use rsbit::v5::api::{
    ///     post::trade::batch_place_order::{
    ///         BatchPlaceOrderParameters,
    ///         BatchPlaceOrderRequestParameters,
    ///         BatchPlaceOrderCategory
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let request = vec![
    ///         BatchPlaceOrderRequestParameters::new(
    ///             "BTCUSDT".to_string(),
    ///             "Buy".to_string(),
    ///             "Market".to_string(),
    ///             0.01,
    ///         ),
    ///     ];
    ///     let params = BatchPlaceOrderParameters::new(
    ///         BatchPlaceOrderCategory::Linear,
    ///         request,
    ///     );
    ///     let response = api.batch_place_order(params).await;
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
    pub async fn batch_place_order(&self, params: BatchPlaceOrderParameters) -> Result<BatchPlaceOrderResponse> {
        self.post(PATH, Some(params)).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum BatchPlaceOrderCategory {
    Linear,
    Option,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchPlaceOrderParameters {
    category: BatchPlaceOrderCategory,
    request: Vec<BatchPlaceOrderRequestParameters>,
}

impl BatchPlaceOrderParameters {
    /// Creates a new instance of `BatchPlaceOrderParameters`.
    ///
    /// # Arguments
    ///
    /// * `category` - The category of the order.
    /// * `request` - The request of the order.
    ///
    /// # Returns
    ///
    /// A new instance of `BatchPlaceOrderParameters`.
    pub fn new(category: BatchPlaceOrderCategory, request: Vec<BatchPlaceOrderRequestParameters>) -> Self {
        Self {
            category,
            request,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchPlaceOrderRequestParameters {
    symbol: String,
    side: String,
    order_type: String,
    #[serde(serialize_with = "serialize_as_string")]
    qty: f64,
    #[serde(serialize_with = "serialize_option_as_string")]
    price: Option<f64>,
    #[serde(serialize_with = "serialize_option_as_string")]
    trigger_direction: Option<u8>,
    trigger_price: Option<String>,
    trigger_by: Option<String>,
    order_iv: Option<String>,
    time_in_force: Option<String>,
    #[serde(serialize_with = "serialize_option_as_string")]
    position_idx: Option<u8>,
    order_link_id: Option<String>,
    take_profit: Option<String>,
    stop_loss: Option<String>,
    tp_trigger_by: Option<String>,
    sl_trigger_by: Option<String>,
    #[serde(serialize_with = "serialize_option_as_string")]
    reduce_only: Option<bool>,
    #[serde(serialize_with = "serialize_option_as_string")]
    close_on_trigger: Option<bool>,
    smp_type: Option<String>,
    #[serde(serialize_with = "serialize_option_as_string")]
    mmp: Option<bool>,
    tpsl_mode: Option<String>,
    tp_limit_price: Option<String>,
    sl_limit_price: Option<String>,
    tp_order_type: Option<String>,
    sl_order_type: Option<String>,
}

impl BatchPlaceOrderRequestParameters {
    /// Creates a new instance of `BatchPlaceOrderRequestParameters`.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The symbol of the order.
    /// * `side` - The side of the order.
    /// * `order_type` - The type of the order.
    /// * `qty` - The quantity of the order.
    ///
    /// # Returns
    ///
    /// A new instance of `BatchPlaceOrderRequestParameters`.
    pub fn new(symbol: String, side: String, order_type: String, qty: f64) -> Self {
        Self {
            symbol,
            side,
            order_type,
            qty,
            price: None,
            trigger_direction: None,
            trigger_price: None,
            trigger_by: None,
            order_iv: None,
            time_in_force: None,
            position_idx: None,
            order_link_id: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            reduce_only: None,
            close_on_trigger: None,
            smp_type: None,
            mmp: None,
            tpsl_mode: None,
            tp_limit_price: None,
            sl_limit_price: None,
            tp_order_type: None,
            sl_order_type: None,
        }
    }

    /// Sets the price for the batch place order.
    ///
    /// # Arguments
    ///
    /// * `price` - The price to set.
    ///
    /// # Returns
    ///
    /// The modified `BatchPlaceOrderParameters` instance.
    pub fn with_price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }

    /// Sets the trigger direction for the batch place order.
    ///
    /// # Arguments
    ///
    /// * `trigger_direction` - The trigger direction to set.
    ///
    /// # Returns
    ///
    /// The modified `BatchPlaceOrderParameters` instance.
    pub fn with_trigger_direction(mut self, trigger_direction: u8) -> Self {
        self.trigger_direction = Some(trigger_direction);
        self
    }

    /// Sets the trigger price for the batch place order.
    ///
    /// # Arguments
    ///
    /// * `trigger_price` - The trigger price to set.
    ///
    /// # Returns
    ///
    /// The modified `BatchPlaceOrderParameters` instance.
    pub fn with_trigger_price(mut self, trigger_price: String) -> Self {
        self.trigger_price = Some(trigger_price);
        self
    }

    /// Sets the trigger by for the batch place order.
    ///
    /// # Arguments
    ///
    /// * `trigger_by` - The trigger by to set.
    ///
    /// # Returns
    ///
    /// The modified `BatchPlaceOrderParameters` instance.
    pub fn with_trigger_by(mut self, trigger_by: String) -> Self {
        self.trigger_by = Some(trigger_by);
        self
    }

    /// Sets the order iv for the batch place order.
    ///
    /// # Arguments
    ///
    /// * `order_iv` - The order iv to set.
    ///
    /// # Returns
    ///
    /// The modified `BatchPlaceOrderParameters` instance.
    pub fn with_order_iv(mut self, order_iv: String) -> Self {
        self.order_iv = Some(order_iv);
        self
    }

    /// Sets the time in force for the batch place order.
    ///
    /// # Arguments
    ///
    /// * `time_in_force` - The time in force to set.
    ///
    /// # Returns
    ///
    /// The modified `BatchPlaceOrderParameters` instance.
    pub fn with_time_in_force(mut self, time_in_force: String) -> Self {
        self.time_in_force = Some(time_in_force);
        self
    }

    /// Sets the with position idx for the batch place order.
    ///
    /// # Arguments
    ///
    /// * `with_position_idx` - The with position idx to set.
    ///
    /// # Returns
    ///
    /// The modified `BatchPlaceOrderParameters` instance.
    pub fn with_position_idx(mut self, position_idx: u8) -> Self {
        self.position_idx = Some(position_idx);
        self
    }

    /// Sets the order link id for the batch place order.
    ///
    /// # Arguments
    ///
    /// * `order_link_id` - The order link id to set.
    ///
    /// # Returns
    ///
    /// The modified `BatchPlaceOrderParameters` instance.
    pub fn with_order_link_id(mut self, order_link_id: String) -> Self {
        self.order_link_id = Some(order_link_id);
        self
    }

    /// Sets the take profit for the batch place order.
    ///
    /// # Arguments
    ///
    /// * `take_profit` - The take profit to set.
    ///
    /// # Returns
    ///
    /// The modified `BatchPlaceOrderParameters` instance.
    pub fn with_take_profit(mut self, take_profit: String) -> Self {
        self.take_profit = Some(take_profit);
        self
    }

    /// Sets the stop loss for the batch place order.
    ///
    /// # Arguments
    ///
    /// * `stop_loss` - The stop loss to set.
    ///
    /// # Returns
    ///
    /// The modified `BatchPlaceOrderParameters` instance.
    pub fn with_stop_loss(mut self, stop_loss: String) -> Self {
        self.stop_loss = Some(stop_loss);
        self
    }

    /// Sets the tp trigger by for the batch place order.
    ///
    /// # Arguments
    ///
    /// * `tp_trigger_by` - The tp trigger by to set.
    ///
    /// # Returns
    ///
    /// The modified `BatchPlaceOrderParameters` instance.
    pub fn with_tp_trigger_by(mut self, tp_trigger_by: String) -> Self {
        self.tp_trigger_by = Some(tp_trigger_by);
        self
    }

    /// Sets the sl trigger by for the batch place order.
    ///
    /// # Arguments
    ///
    /// * `sl_trigger_by` - The sl trigger by to set.
    ///
    /// # Returns
    ///
    /// The modified `BatchPlaceOrderParameters` instance.
    pub fn with_sl_trigger_by(mut self, sl_trigger_by: String) -> Self {
        self.sl_trigger_by = Some(sl_trigger_by);
        self
    }

    /// Sets the reduce only for the batch place order.
    ///
    /// # Arguments
    ///
    /// * `reduce_only` - The reduce only to set.
    ///
    /// # Returns
    ///
    /// The modified `BatchPlaceOrderParameters` instance.
    pub fn with_reduce_only(mut self, reduce_only: bool) -> Self {
        self.reduce_only = Some(reduce_only);
        self
    }

    /// Sets the close on trigger for the batch place order.
    ///
    /// # Arguments
    ///
    /// * `close_on_trigger` - The close on trigger to set.
    ///
    /// # Returns
    ///
    /// The modified `BatchPlaceOrderParameters` instance.
    pub fn with_close_on_trigger(mut self, close_on_trigger: bool) -> Self {
        self.close_on_trigger = Some(close_on_trigger);
        self
    }

    /// Sets the smp type for the batch place order.
    ///
    /// # Arguments
    ///
    /// * `smp_type` - The smp type to set.
    ///
    /// # Returns
    ///
    /// The modified `BatchPlaceOrderParameters` instance.
    pub fn with_smp_type(mut self, smp_type: String) -> Self {
        self.smp_type = Some(smp_type);
        self
    }

    /// Sets the mmp for the batch place order.
    ///
    /// # Arguments
    ///
    /// * `mmp` - The mmp to set.
    ///
    /// # Returns
    ///
    /// The modified `BatchPlaceOrderParameters` instance.
    pub fn with_mmp(mut self, mmp: bool) -> Self {
        self.mmp = Some(mmp);
        self
    }

    /// Sets the tpsl mode for the batch place order.
    ///
    /// # Arguments
    ///
    /// * `tpsl_mode` - The tpsl mode to set.
    ///
    /// # Returns
    ///
    /// The modified `BatchPlaceOrderParameters` instance.
    pub fn with_tpsl_mode(mut self, tpsl_mode: String) -> Self {
        self.tpsl_mode = Some(tpsl_mode);
        self
    }

    /// Sets the tp limit price for the batch place order.
    ///
    /// # Arguments
    ///
    /// * `tp_limit_price` - The tp limit price to set.
    ///
    /// # Returns
    ///
    /// The modified `BatchPlaceOrderParameters` instance.
    pub fn with_tp_limit_price(mut self, tp_limit_price: String) -> Self {
        self.tp_limit_price = Some(tp_limit_price);
        self
    }

    /// Sets the sl limit price for the batch place order.
    ///
    /// # Arguments
    ///
    /// * `sl_limit_price` - The sl limit price to set.
    ///
    /// # Returns
    ///
    /// The modified `BatchPlaceOrderParameters` instance.
    pub fn with_sl_limit_price(mut self, sl_limit_price: String) -> Self {
        self.sl_limit_price = Some(sl_limit_price);
        self
    }

    /// Sets the tp order type for the batch place order.
    ///
    /// # Arguments
    ///
    /// * `tp_orde_ type` - The tp order type to set.
    ///
    /// # Returns
    ///
    /// The modified `BatchPlaceOrderParameters` instance.
    pub fn with_tp_order_type(mut self, tp_order_type: String) -> Self {
        self.tp_order_type = Some(tp_order_type);
        self
    }

    /// Sets the sl order type for the batch place order.
    ///
    /// # Arguments
    ///
    /// * `sl_order_type` - The sl order type to set.
    ///
    /// # Returns
    ///
    /// The modified `BatchPlaceOrderParameters` instance.
    pub fn with_sl_order_type(mut self, sl_order_type: String) -> Self {
        self.sl_order_type = Some(sl_order_type);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchPlaceOrderResponse {
    ret_code: i32,
    ret_msg: String,
    result: BatchPlaceOrderResult,
    ret_ext_info: Value,
    time: u64,
}
impl BatchPlaceOrderResponse {
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

    pub fn result(&self) -> &BatchPlaceOrderResult {
        &self.result
    }

    pub fn set_result(&mut self, result: BatchPlaceOrderResult) {
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
pub struct BatchPlaceOrderResult {
    list: Vec<BatchPlaceOrder>,
}
impl BatchPlaceOrderResult {
    pub fn list(&self) -> &Vec<BatchPlaceOrder> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<BatchPlaceOrder>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchPlaceOrder {
    category: String,
    symbol: String,
    order_id: String,
    order_link_id: String,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    create_at: u64,
}

impl BatchPlaceOrder {
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

    pub fn create_at(&self) -> u64 {
        self.create_at
    }

    pub fn set_create_at(&mut self, create_at: u64) {
        self.create_at = create_at;
    }
}