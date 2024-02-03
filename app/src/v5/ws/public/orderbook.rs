use crate::{
    v5::ws::BybitWS,
    constants::PUBLIC_ORDERBOOK_TOPIC,
};
use serde::{
    Deserialize,
    Deserializer
};

impl BybitWS {
    pub fn add_orderbook_args(&mut self, depth: &str, symbol: &str) {
        self.args.push(format!("{}.{}.{}", PUBLIC_ORDERBOOK_TOPIC, depth, symbol));
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicOrderbookResponse {
    topic: String,
    #[serde(rename = "type")]
    type_field: String,
    ts: u64,
    data: PublicOrderbookData,
    cts: u64,
}

impl PublicOrderbookResponse {
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

    pub fn data(&self) -> &PublicOrderbookData {
        &self.data
    }

    pub fn set_data(&mut self, data: PublicOrderbookData) {
        self.data = data;
    }

    pub fn cts(&self) -> u64 {
        self.cts
    }

    pub fn set_cts(&mut self, cts: u64) {
        self.cts = cts;
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PublicOrderbookData {
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "b")]
    pub bids: Vec<Order>,
    #[serde(rename = "a")]
    pub asks: Vec<Order>,
    #[serde(rename = "u")]
    update_id: u64,
    seq: u64,
}

impl PublicOrderbookData {
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn bids(&self) -> &Vec<Order> {
        &self.bids
    }

    pub fn set_bids(&mut self, bids: Vec<Order>) {
        self.bids = bids;
    }

    pub fn asks(&self) -> &Vec<Order> {
        &self.asks
    }

    pub fn set_asks(&mut self, asks: Vec<Order>) {
        self.asks = asks;
    }

    pub fn update_id(&self) -> u64 {
        self.update_id
    }

    pub fn set_update_id(&mut self, update_id: u64) {
        self.update_id = update_id;
    }

    pub fn seq(&self) -> u64 {
        self.seq
    }

    pub fn set_seq(&mut self, seq: u64) {
        self.seq = seq;
    }
}

#[derive(Debug, Clone)]
pub struct Order {
    price: f64,
    size: f64,
}

impl Order {
    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn set_price(&mut self, price: f64) {
        self.price = price;
    }

    pub fn size(&self) -> f64 {
        self.size
    }

    pub fn set_size(&mut self, size: f64) {
        self.size = size;
    }
}


// Convert a JSON tuple to an Order using custom deserialization
impl<'de> Deserialize<'de> for Order {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (price_str, size_str): (String, String) = Deserialize::deserialize(deserializer)?;
        let price = price_str.parse::<f64>()
            .map_err(serde::de::Error::custom)?;
        let size = size_str.parse::<f64>()
            .map_err(serde::de::Error::custom)?;
        
        Ok(Order { price, size })
    }
}
