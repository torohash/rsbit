use crate::{
    v5::ws::BybitWS,
    constants::PUBLIC_LIQUIDATION_TOPIC,
    utils::deserialize_f64,
};
use serde::Deserialize;

impl BybitWS {
    pub fn add_liquidation_args(&mut self, symbol: &str) {
        self.args.push(format!("{}.{}", PUBLIC_LIQUIDATION_TOPIC, symbol));
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicLiquidationResponse {
    topic: String,
    #[serde(rename = "type")]
    type_field: String,
    ts: u64,
    data: PublicLiquidationData,
}

impl PublicLiquidationResponse {
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

    pub fn data(&self) -> &PublicLiquidationData {
        &self.data
    }

    pub fn set_data(&mut self, data: PublicLiquidationData) {
        self.data = data;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicLiquidationData {
    updated_time: u64,
    symbol: String,
    side: String,
    #[serde(deserialize_with = "deserialize_f64")]
    size: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    price: f64,
}

impl PublicLiquidationData {
    pub fn updated_time(&self) -> u64 {
        self.updated_time
    }

    pub fn set_updated_time(&mut self, updated_time: u64) {
        self.updated_time = updated_time;
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

    pub fn size(&self) -> f64 {
        self.size
    }

    pub fn set_size(&mut self, size: f64) {
        self.size = size;
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn set_price(&mut self, price: f64) {
        self.price = price;
    }
}