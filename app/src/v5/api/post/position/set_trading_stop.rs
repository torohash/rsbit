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

const PATH: &'static str = "/v5/position/trading-stop";

impl BybitApi {
    /// set trading stop.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for set trading stop.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::v5::api::{
    ///     post::position::set_trading_stop::{
    ///         SetTradingStopParameters,
    ///         SetTradingStopCategory
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = SetTradingStopParameters::new(
    ///         SetTradingStopCategory::Linear,
    ///         "BTCUSDT".to_string(),
    ///         1,
    ///     ).with_take_profit(50000.0);
    ///     let response = api.set_trading_stop(params).await;
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
    pub async fn set_trading_stop(&self, params: SetTradingStopParameters) -> Result<SetTradingStopResponse> {
        self.post(PATH, Some(params)).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SetTradingStopCategory {
    Linear,
    Inverse,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTradingStopParameters {
    category: SetTradingStopCategory,
    symbol: String,
    #[serde(serialize_with = "serialize_option_as_string")]
    take_profit: Option<f64>,
    #[serde(serialize_with = "serialize_option_as_string")]
    stop_loss: Option<f64>,
    #[serde(serialize_with = "serialize_option_as_string")]
    trailing_stop: Option<f64>,
    tp_trigger_by: Option<String>,
    sl_trigger_by: Option<String>,
    #[serde(serialize_with = "serialize_option_as_string")]
    active_price: Option<f64>,
    tpsl_mode: Option<String>,
    #[serde(serialize_with = "serialize_option_as_string")]
    tp_size: Option<f64>,
    #[serde(serialize_with = "serialize_option_as_string")]
    sl_size: Option<f64>,
    #[serde(serialize_with = "serialize_option_as_string")]
    tp_limit_price: Option<f64>,
    #[serde(serialize_with = "serialize_option_as_string")]
    sl_limit_price: Option<f64>,
    tp_order_type: Option<String>,
    sl_order_type: Option<String>,
    position_idx: u8,
}

impl SetTradingStopParameters {
    /// Creates a new instance of `SetTradingStopParameters`.
    ///
    /// # Arguments
    ///
    /// * `category` - The category.
    /// * `symbol` - The symbol.
    /// * `position_idx` - The position idx.
    ///
    /// # Returns
    ///
    /// A new instance of `SetTradingStopParameters`.
    pub fn new(category: SetTradingStopCategory, symbol: String, position_idx: u8) -> Self {
        Self {
            category,
            symbol,
            take_profit: None,
            stop_loss: None,
            trailing_stop: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            active_price: None,
            tpsl_mode: None,
            tp_size: None,
            sl_size: None,
            tp_limit_price: None,
            sl_limit_price: None,
            tp_order_type: None,
            sl_order_type: None,
            position_idx,
        }
    }

    /// Sets the take profit for the set trading stop.
    ///
    /// # Arguments
    ///
    /// * `take_profit` - The take profit to set.
    ///
    /// # Returns
    ///
    /// The modified `SetTradingStopParameters` instance.
    pub fn with_take_profit(mut self, take_profit: f64) -> Self {
        self.take_profit = Some(take_profit);
        self
    }

    /// Sets the stop loss for the set trading stop.
    ///
    /// # Arguments
    ///
    /// * `stop_loss` - The stop loss to set.
    ///
    /// # Returns
    ///
    /// The modified `SetTradingStopParameters` instance.
    pub fn with_stop_loss(mut self, stop_loss: f64) -> Self {
        self.stop_loss = Some(stop_loss);
        self
    }

    /// Sets the trailing stop for the set trading stop.
    ///
    /// # Arguments
    ///
    /// * `trailing_stop` - The trailing stop to set.
    ///
    /// # Returns
    ///
    /// The modified `SetTradingStopParameters` instance.
    pub fn with_trailing_stop(mut self, trailing_stop: f64) -> Self {
        self.trailing_stop = Some(trailing_stop);
        self
    }

    /// Sets the tp trigger by for the set trading stop.
    ///
    /// # Arguments
    ///
    /// * `tp_trigger_by` - The tp trigger by to set.
    ///
    /// # Returns
    ///
    /// The modified `SetTradingStopParameters` instance.
    pub fn with_tp_trigger_by(mut self, tp_trigger_by: String) -> Self {
        self.tp_trigger_by = Some(tp_trigger_by);
        self
    }

    /// Sets the sl trigger by for the set trading stop.
    ///
    /// # Arguments
    ///
    /// * `sl_trigger_by` - The sl trigger by to set.
    ///
    /// # Returns
    ///
    /// The modified `SetTradingStopParameters` instance.
    pub fn with_sl_trigger_by(mut self, sl_trigger_by: String) -> Self {
        self.sl_trigger_by = Some(sl_trigger_by);
        self
    }

    /// Sets the active price for the set trading stop.
    ///
    /// # Arguments
    ///
    /// * `active_price` - The active price to set.
    ///
    /// # Returns
    ///
    /// The modified `SetTradingStopParameters` instance.
    pub fn with_active_price(mut self, active_price: f64) -> Self {
        self.active_price = Some(active_price);
        self
    }

    /// Sets the tpsl mode for the set trading stop.
    ///
    /// # Arguments
    ///
    /// * `tpsl_mode` - The tpsl mode to set.
    ///
    /// # Returns
    ///
    /// The modified `SetTradingStopParameters` instance.
    pub fn with_tpsl_mode(mut self, tpsl_mode: String) -> Self {
        self.tpsl_mode = Some(tpsl_mode);
        self
    }

    /// Sets the tp size for the set trading stop.
    ///
    /// # Arguments
    ///
    /// * `tp_size` - The tp size to set.
    ///
    /// # Returns
    ///
    /// The modified `SetTradingStopParameters` instance.
    pub fn with_tp_size(mut self, tp_size: f64) -> Self {
        self.tp_size = Some(tp_size);
        self
    }

    /// Sets the sl size for the set trading stop.
    ///
    /// # Arguments
    ///
    /// * `sl_size` - The sl size to set.
    ///
    /// # Returns
    ///
    /// The modified `SetTradingStopParameters` instance.
    pub fn with_sl_size(mut self, sl_size: f64) -> Self {
        self.sl_size = Some(sl_size);
        self
    }

    /// Sets the tp limit price for the set trading stop.
    ///
    /// # Arguments
    ///
    /// * `tp_limit_price` - The tp limit price to set.
    ///
    /// # Returns
    ///
    /// The modified `SetTradingStopParameters` instance.
    pub fn with_tp_limit_price(mut self, tp_limit_price: f64) -> Self {
        self.tp_limit_price = Some(tp_limit_price);
        self
    }

    /// Sets the sl limit price for the set trading stop.
    ///
    /// # Arguments
    ///
    /// * `sl_limit_price` - The sl limit price to set.
    ///
    /// # Returns
    ///
    /// The modified `SetTradingStopParameters` instance.
    pub fn with_sl_limit_price(mut self, sl_limit_price: f64) -> Self {
        self.sl_limit_price = Some(sl_limit_price);
        self
    }

    /// Sets the tp order type for the set trading stop.
    ///
    /// # Arguments
    ///
    /// * `tp_order_type` - The tp order type to set.
    ///
    /// # Returns
    ///
    /// The modified `SetTradingStopParameters` instance.
    pub fn with_tp_order_type(mut self, tp_order_type: String) -> Self {
        self.tp_order_type = Some(tp_order_type);
        self
    }

    /// Sets the sl order type for the set trading stop.
    ///
    /// # Arguments
    ///
    /// * `sl_order_type` - The sl order type to set.
    ///
    /// # Returns
    ///
    /// The modified `SetTradingStopParameters` instance.
    pub fn with_sl_order_type(mut self, sl_order_type: String) -> Self {
        self.sl_order_type = Some(sl_order_type);
        self
    }

    /// Sets the position idx for the set trading stop.
    ///
    /// # Arguments
    ///
    /// * `position_idx` - The position idx to set.
    ///
    /// # Returns
    ///
    /// The modified `SetTradingStopParameters` instance.
    pub fn with_position_idx(mut self, position_idx: u8) -> Self {
        self.position_idx = position_idx;
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTradingStopResponse {
    ret_code: i32,
    ret_msg: String,
    result: Value,
    ret_ext_info: Value,
    time: u64,
}
impl SetTradingStopResponse {
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

    pub fn result(&self) -> &Value {
        &self.result
    }

    pub fn set_result(&mut self, result: Value) {
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