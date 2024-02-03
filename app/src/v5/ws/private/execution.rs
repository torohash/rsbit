use crate::{
    v5::ws::BybitWS,
    constants::PRIVATE_EXECUTION_TOPIC,
    utils::{
        deserialize_f64,
        deserialize_string_to_u64,
        deserialize_option_f64,
    },
};
use serde::Deserialize;

pub enum ExecutionCategory {
    Linear,
    Inverse,
    Option,
    Spot,
}

impl BybitWS {
    pub fn add_execution_args(&mut self, category: Option<ExecutionCategory>) {
        match category {
            Some(category) => {
                self.args.push(match category {
                    ExecutionCategory::Linear => format!("{}.{}", PRIVATE_EXECUTION_TOPIC, "linear"),
                    ExecutionCategory::Inverse => format!("{}.{}", PRIVATE_EXECUTION_TOPIC, "inverse"),
                    ExecutionCategory::Option => format!("{}.{}", PRIVATE_EXECUTION_TOPIC, "option"),
                    ExecutionCategory::Spot => format!("{}.{}", PRIVATE_EXECUTION_TOPIC, "spot"),
                });
            },
            None => {
                self.args.push(PRIVATE_EXECUTION_TOPIC.to_string());
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivateExecutionResponse {
    id: String,
    topic: String,
    creation_time: u64,
    data: Vec<PrivateExecutionData>,
}

impl PrivateExecutionResponse {
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

    pub fn data(&self) -> &Vec<PrivateExecutionData> {
        &self.data
    }

    pub fn set_data(&mut self, data: Vec<PrivateExecutionData>) {
        self.data = data;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivateExecutionData {
    category: String,
    symbol: String,
    #[serde(deserialize_with = "deserialize_f64")]
    exec_fee: f64,
    exec_id: String,
    #[serde(deserialize_with = "deserialize_f64")]
    exec_price: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    exec_qty: f64,
    exec_type: String,
    #[serde(deserialize_with = "deserialize_f64")]
    exec_value: f64,
    is_maker: bool,
    #[serde(deserialize_with = "deserialize_f64")]
    fee_rate: f64,
    #[serde(deserialize_with = "deserialize_option_f64")]
    trade_iv: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    mark_iv: Option<f64>,
    block_trade_id: String,
    #[serde(deserialize_with = "deserialize_option_f64")]
    mark_price: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    index_price: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    underlying_price: Option<f64>,
    #[serde(deserialize_with = "deserialize_f64")]
    leaves_qty: f64,
    order_id: String,
    order_link_id: String,
    #[serde(deserialize_with = "deserialize_f64")]
    order_price: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    order_qty: f64,
    order_type: String,
    stop_order_type: String,
    side: String,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    exec_time: u64,
    is_leverage: String,
    closed_size: String,
    seq: u64,
}

impl PrivateExecutionData {
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

    pub fn exec_fee(&self) -> f64 {
        self.exec_fee
    }

    pub fn set_exec_fee(&mut self, exec_fee: f64) {
        self.exec_fee = exec_fee;
    }

    pub fn exec_id(&self) -> &str {
        &self.exec_id
    }

    pub fn set_exec_id(&mut self, exec_id: String) {
        self.exec_id = exec_id;
    }

    pub fn exec_price(&self) -> f64 {
        self.exec_price
    }

    pub fn set_exec_price(&mut self, exec_price: f64) {
        self.exec_price = exec_price;
    }

    pub fn exec_qty(&self) -> f64 {
        self.exec_qty
    }

    pub fn set_exec_qty(&mut self, exec_qty: f64) {
        self.exec_qty = exec_qty;
    }

    pub fn exec_type(&self) -> &str {
        &self.exec_type
    }

    pub fn set_exec_type(&mut self, exec_type: String) {
        self.exec_type = exec_type;
    }

    pub fn exec_value(&self) -> f64 {
        self.exec_value
    }

    pub fn set_exec_value(&mut self, exec_value: f64) {
        self.exec_value = exec_value;
    }

    pub fn is_maker(&self) -> bool {
        self.is_maker
    }

    pub fn set_is_maker(&mut self, is_maker: bool) {
        self.is_maker = is_maker;
    }

    pub fn fee_rate(&self) -> f64 {
        self.fee_rate
    }

    pub fn set_fee_rate(&mut self, fee_rate: f64) {
        self.fee_rate = fee_rate;
    }

    pub fn trade_iv(&self) -> Option<f64> {
        self.trade_iv
    }

    pub fn set_trade_iv(&mut self, trade_iv: Option<f64>) {
        self.trade_iv = trade_iv;
    }

    pub fn mark_iv(&self) -> Option<f64> {
        self.mark_iv
    }

    pub fn set_mark_iv(&mut self, mark_iv: Option<f64>) {
        self.mark_iv = mark_iv;
    }

    pub fn block_trade_id(&self) -> &str {
        &self.block_trade_id
    }

    pub fn set_block_trade_id(&mut self, block_trade_id: String) {
        self.block_trade_id = block_trade_id;
    }

    pub fn mark_price(&self) -> Option<f64> {
        self.mark_price
    }

    pub fn set_mark_price(&mut self, mark_price: f64) {
        self.mark_price = Some(mark_price);
    }

    pub fn index_price(&self) -> Option<f64> {
        self.index_price
    }

    pub fn set_index_price(&mut self, index_price: Option<f64>) {
        self.index_price = index_price;
    }

    pub fn underlying_price(&self) -> Option<f64> {
        self.underlying_price
    }

    pub fn set_underlying_price(&mut self, underlying_price: Option<f64>) {
        self.underlying_price = underlying_price;
    }

    pub fn leaves_qty(&self) -> f64 {
        self.leaves_qty
    }

    pub fn set_leaves_qty(&mut self, leaves_qty: f64) {
        self.leaves_qty = leaves_qty;
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

    pub fn order_price(&self) -> f64 {
        self.order_price
    }

    pub fn set_order_price(&mut self, order_price: f64) {
        self.order_price = order_price;
    }

    pub fn order_qty(&self) -> f64 {
        self.order_qty
    }

    pub fn set_order_qty(&mut self, order_qty: f64) {
        self.order_qty = order_qty;
    }

    pub fn order_type(&self) -> &str {
        &self.order_type
    }

    pub fn set_order_type(&mut self, order_type: String) {
        self.order_type = order_type;
    }

    pub fn stop_order_type(&self) -> &str {
        &self.stop_order_type
    }

    pub fn set_stop_order_type(&mut self, stop_order_type: String) {
        self.stop_order_type = stop_order_type;
    }

    pub fn side(&self) -> &str {
        &self.side
    }

    pub fn set_side(&mut self, side: String) {
        self.side = side;
    }

    pub fn exec_time(&self) -> u64 {
        self.exec_time
    }

    pub fn set_exec_time(&mut self, exec_time: u64) {
        self.exec_time = exec_time;
    }

    pub fn is_leverage(&self) -> &str {
        &self.is_leverage
    }

    pub fn set_is_leverage(&mut self, is_leverage: String) {
        self.is_leverage = is_leverage;
    }

    pub fn closed_size(&self) -> &str {
        &self.closed_size
    }

    pub fn set_closed_size(&mut self, closed_size: String) {
        self.closed_size = closed_size;
    }

    pub fn seq(&self) -> u64 {
        self.seq
    }

    pub fn set_seq(&mut self, seq: u64) {
        self.seq = seq;
    }
}