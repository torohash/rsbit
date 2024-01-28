use crate::{
    v5::ws::BybitWS,
    constants::PUBLIC_TRADE_TOPIC,
    utils::deserialize_f64,
};
use serde::Deserialize;

impl BybitWS {
    pub fn add_trade_args(&mut self, symbol: &str) {
        self.args.push(format!("{}.{}", PUBLIC_TRADE_TOPIC, symbol.to_string()));
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicTradeResponse {
    topic: String,
    #[serde(rename = "type")]
    type_field: String,
    ts: u64,
    data: Vec<PublicTradeData>,
}

impl PublicTradeResponse {
    pub fn topic(&self) -> &str {
        &self.topic
    }

    pub fn set_topic(&mut self, topic: String) {
        self.topic = topic;
    }

    pub fn type_field(&self) -> &str {
        &self.type_field
    }

    pub fn set_type_field(&mut self, type_field: String) {
        self.type_field = type_field;
    }

    pub fn ts(&self) -> u64 {
        self.ts
    }

    pub fn set_ts(&mut self, ts: u64) {
        self.ts = ts;
    }

    pub fn data(&self) -> &Vec<PublicTradeData> {
        &self.data
    }

    pub fn set_data(&mut self, data: Vec<PublicTradeData>) {
        self.data = data;
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PublicTradeData {
    #[serde(rename = "T")]
    timestamp: u64,
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "S")]
    side: String,
    #[serde(rename = "v", deserialize_with = "deserialize_f64")]
    volume: f64,
    #[serde(rename = "p", deserialize_with = "deserialize_f64")]
    price: f64,
    #[serde(rename = "L")]
    direction_of_price_change: String,
    #[serde(rename = "i")]
    trade_id: String,
    #[serde(rename = "BT")]
    block_trade: bool,
}

impl PublicTradeData {
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn set_timestamp(&mut self, timestamp: u64) {
        self.timestamp = timestamp;
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn side(&self) -> &str {
        &self.side
    }

    pub fn set_side(&mut self, side: String) {
        self.side = side;
    }

    pub fn volume(&self) -> f64 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: f64) {
        self.volume = volume;
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn set_price(&mut self, price: f64) {
        self.price = price;
    }

    pub fn direction_of_price_change(&self) -> &str {
        &self.direction_of_price_change
    }

    pub fn set_direction_of_price_change(&mut self, direction_of_price_change: String) {
        self.direction_of_price_change = direction_of_price_change;
    }

    pub fn trade_id(&self) -> &str {
        &self.trade_id
    }

    pub fn set_trade_id(&mut self, trade_id: String) {
        self.trade_id = trade_id;
    }

    pub fn block_trade(&self) -> bool {
        self.block_trade
    }

    pub fn set_block_trade(&mut self, block_trade: bool) {
        self.block_trade = block_trade;
    }
}