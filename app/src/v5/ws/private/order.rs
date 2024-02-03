use crate::{
    v5::ws::BybitWS,
    constants::PRIVATE_ORDER_TOPIC,
    utils::{
        deserialize_f64,
        deserialize_string_to_u64,
        deserialize_option_f64,
    },
};
use serde::Deserialize;

pub enum OrderCategory {
    Linear,
    Inverse,
    Option,
    Spot,
}

impl BybitWS {
    pub fn add_order_args(&mut self, category: Option<OrderCategory>) {
        match category {
            Some(category) => {
                self.args.push(match category {
                    OrderCategory::Linear => format!("{}.{}", PRIVATE_ORDER_TOPIC, "linear"),
                    OrderCategory::Inverse => format!("{}.{}", PRIVATE_ORDER_TOPIC, "inverse"),
                    OrderCategory::Option => format!("{}.{}", PRIVATE_ORDER_TOPIC, "option"),
                    OrderCategory::Spot => format!("{}.{}", PRIVATE_ORDER_TOPIC, "spot"),
                });
            },
            None => {
                self.args.push(PRIVATE_ORDER_TOPIC.to_string());
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivateOrderResponse {
    id: String,
    topic: String,
    creation_time: u64,
    data: Vec<PrivateOrderData>,
}

impl PrivateOrderResponse {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn topic(&self) -> &str {
        &self.topic
    }

    pub fn set_topic(&mut self, topic: String) {
        self.topic = topic;
    }

    pub fn creation_time(&self) -> u64 {
        self.creation_time
    }

    pub fn set_creation_time(&mut self, creation_time: u64) {
        self.creation_time = creation_time;
    }

    pub fn data(&self) -> &Vec<PrivateOrderData> {
        &self.data
    }

    pub fn set_data(&mut self, data: Vec<PrivateOrderData>) {
        self.data = data;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivateOrderData {
    symbol: String,
    order_id: String,
    side: String,
    order_type: String,
    cancel_type: String,
    #[serde(deserialize_with = "deserialize_f64")]
    price: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    qty: f64,
    #[serde(deserialize_with = "deserialize_option_f64")]
    order_iv: Option<f64>,
    time_in_force: String,
    order_status: String,
    order_link_id: String,
    #[serde(deserialize_with = "deserialize_option_f64")]
    last_price_on_created: Option<f64>,
    reduce_only: bool,
    #[serde(deserialize_with = "deserialize_option_f64")]
    leaves_qty: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    leaves_value: Option<f64>,
    #[serde(deserialize_with = "deserialize_f64")]
    cum_exec_qty: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    cum_exec_value: f64,
    #[serde(deserialize_with = "deserialize_option_f64")]
    avg_price: Option<f64>,
    block_trade_id: String,
    position_idx: u64,
    #[serde(deserialize_with = "deserialize_f64")]
    cum_exec_fee: f64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    created_time: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    updated_time: u64,
    reject_reason: String,
    stop_order_type: String,
    #[serde(deserialize_with = "deserialize_option_f64")]
    trigger_price: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    take_profit: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    stop_loss: Option<f64>,
    tp_trigger_by: String,
    sl_trigger_by: String,
    trigger_direction: u8,
    trigger_by: String,
    close_on_trigger: bool,
    category: String,
    place_type: String,
    smp_type: String,
    smp_group: u64,
    smp_order_id: String,
}

impl PrivateOrderData {
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

    pub fn side(&self) -> &str {
        &self.side
    }

    pub fn set_side(&mut self, side: String) {
        self.side = side;
    }

    pub fn order_type(&self) -> &str {
        &self.order_type
    }

    pub fn set_order_type(&mut self, order_type: String) {
        self.order_type = order_type;
    }

    pub fn cancel_type(&self) -> &str {
        &self.cancel_type
    }

    pub fn set_cancel_type(&mut self, cancel_type: String) {
        self.cancel_type = cancel_type;
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn set_price(&mut self, price: f64) {
        self.price = price;
    }

    pub fn qty(&self) -> f64 {
        self.qty
    }

    pub fn set_qty(&mut self, qty: f64) {
        self.qty = qty;
    }

    pub fn order_iv(&self) -> Option<f64> {
        self.order_iv
    }

    pub fn set_order_iv(&mut self, order_iv: Option<f64>) {
        self.order_iv = order_iv;
    }

    pub fn time_in_force(&self) -> &str {
        &self.time_in_force
    }

    pub fn set_time_in_force(&mut self, time_in_force: String) {
        self.time_in_force = time_in_force;
    }

    pub fn order_status(&self) -> &str {
        &self.order_status
    }

    pub fn set_order_status(&mut self, order_status: String) {
        self.order_status = order_status;
    }

    pub fn order_link_id(&self) -> &str {
        &self.order_link_id
    }

    pub fn set_order_link_id(&mut self, order_link_id: String) {
        self.order_link_id = order_link_id;
    }

    pub fn last_price_on_created(&self) -> Option<f64> {
        self.last_price_on_created
    }

    pub fn set_last_price_on_created(&mut self, last_price_on_created: Option<f64>) {
        self.last_price_on_created = last_price_on_created;
    }

    pub fn reduce_only(&self) -> bool {
        self.reduce_only
    }

    pub fn set_reduce_only(&mut self, reduce_only: bool) {
        self.reduce_only = reduce_only;
    }

    pub fn leaves_qty(&self) -> Option<f64> {
        self.leaves_qty
    }

    pub fn set_leaves_qty(&mut self, leaves_qty: Option<f64>) {
        self.leaves_qty = leaves_qty;
    }

    pub fn leaves_value(&self) -> Option<f64> {
        self.leaves_value
    }

    pub fn set_leaves_value(&mut self, leaves_value: Option<f64>) {
        self.leaves_value = leaves_value;
    }

    pub fn cum_exec_qty(&self) -> f64 {
        self.cum_exec_qty
    }

    pub fn set_cum_exec_qty(&mut self, cum_exec_qty: f64) {
        self.cum_exec_qty = cum_exec_qty;
    }

    pub fn cum_exec_value(&self) -> f64 {
        self.cum_exec_value
    }

    pub fn set_cum_exec_value(&mut self, cum_exec_value: f64) {
        self.cum_exec_value = cum_exec_value;
    }

    pub fn avg_price(&self) -> Option<f64> {
        self.avg_price
    }

    pub fn set_avg_price(&mut self, avg_price: f64) {
        self.avg_price = Some(avg_price);
    }

    pub fn block_trade_id(&self) -> &str {
        &self.block_trade_id
    }

    pub fn set_block_trade_id(&mut self, block_trade_id: String) {
        self.block_trade_id = block_trade_id;
    }

    pub fn position_idx(&self) -> u64 {
        self.position_idx
    }

    pub fn set_position_idx(&mut self, position_idx: u64) {
        self.position_idx = position_idx;
    }

    pub fn cum_exec_fee(&self) -> f64 {
        self.cum_exec_fee
    }

    pub fn set_cum_exec_fee(&mut self, cum_exec_fee: f64) {
        self.cum_exec_fee = cum_exec_fee;
    }

    pub fn created_time(&self) -> u64 {
        self.created_time
    }

    pub fn set_created_time(&mut self, created_time: u64) {
        self.created_time = created_time;
    }

    pub fn updated_time(&self) -> u64 {
        self.updated_time
    }

    pub fn set_updated_time(&mut self, updated_time: u64) {
        self.updated_time = updated_time;
    }

    pub fn reject_reason(&self) -> &str {
        &self.reject_reason
    }

    pub fn set_reject_reason(&mut self, reject_reason: String) {
        self.reject_reason = reject_reason;
    }

    pub fn stop_order_type(&self) -> &str {
        &self.stop_order_type
    }

    pub fn set_stop_order_type(&mut self, stop_order_type: String) {
        self.stop_order_type = stop_order_type;
    }

    pub fn trigger_price(&self) -> Option<f64> {
        self.trigger_price
    }

    pub fn set_trigger_price(&mut self, trigger_price: Option<f64>) {
        self.trigger_price = trigger_price;
    }

    pub fn take_profit(&self) -> Option<f64> {
        self.take_profit
    }

    pub fn set_take_profit(&mut self, take_profit: Option<f64>) {
        self.take_profit = take_profit;
    }

    pub fn stop_loss(&self) -> Option<f64> {
        self.stop_loss
    }

    pub fn set_stop_loss(&mut self, stop_loss: Option<f64>) {
        self.stop_loss = stop_loss;
    }

    pub fn tp_trigger_by(&self) -> &str {
        &self.tp_trigger_by
    }

    pub fn set_tp_trigger_by(&mut self, tp_trigger_by: String) {
        self.tp_trigger_by = tp_trigger_by;
    }

    pub fn sl_trigger_by(&self) -> &str {
        &self.sl_trigger_by
    }

    pub fn set_sl_trigger_by(&mut self, sl_trigger_by: String) {
        self.sl_trigger_by = sl_trigger_by;
    }

    pub fn trigger_direction(&self) -> u8 {
        self.trigger_direction
    }

    pub fn set_trigger_direction(&mut self, trigger_direction: u8) {
        self.trigger_direction = trigger_direction;
    }

    pub fn trigger_by(&self) -> &str {
        &self.trigger_by
    }

    pub fn set_trigger_by(&mut self, trigger_by: String) {
        self.trigger_by = trigger_by;
    }

    pub fn close_on_trigger(&self) -> bool {
        self.close_on_trigger
    }

    pub fn set_close_on_trigger(&mut self, close_on_trigger: bool) {
        self.close_on_trigger = close_on_trigger;
    }

    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn set_category(&mut self, category: String) {
        self.category = category;
    }

    pub fn place_type(&self) -> &str {
        &self.place_type
    }

    pub fn set_place_type(&mut self, place_type: String) {
        self.place_type = place_type;
    }

    pub fn smp_type(&self) -> &str {
        &self.smp_type
    }

    pub fn set_smp_type(&mut self, smp_type: String) {
        self.smp_type = smp_type;
    }

    pub fn smp_group(&self) -> u64 {
        self.smp_group
    }

    pub fn set_smp_group(&mut self, smp_group: u64) {
        self.smp_group = smp_group;
    }

    pub fn smp_order_id(&self) -> &str {
        &self.smp_order_id
    }

    pub fn set_smp_order_id(&mut self, smp_order_id: String) {
        self.smp_order_id = smp_order_id;
    }

}