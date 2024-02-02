use crate::utils::{
    deserialize_option_f64,
    deserialize_string_to_option_u64,
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicLinearTickersResponse {
    topic: String,
    #[serde(rename = "type")]
    type_field: String,
    data: LinearTickers,
    cs: u64,
    ts: u64,
}

impl PublicLinearTickersResponse {
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

    pub fn data(&self) -> &LinearTickers {
        &self.data
    }

    pub fn set_data(&mut self, data: LinearTickers) {
        self.data = data;
    }

    pub fn cs(&self) -> u64 {
        self.cs
    }

    pub fn set_cs(&mut self, cs: u64) {
        self.cs = cs;
    }

    pub fn ts(&self) -> u64 {
        self.ts
    }

    pub fn set_ts(&mut self, ts: u64) {
        self.ts = ts;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinearTickers {
    symbol: String,
    tick_direction: String,
    #[serde(deserialize_with = "deserialize_option_f64")]
    price24h_pcnt: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    last_price: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    prev_price24h: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    high_price24h: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    low_price24h: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    prev_price1h: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    mark_price: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    index_price: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    open_interest: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    open_interest_value: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    turnover24h: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    volume24h: Option<f64>,
    #[serde(deserialize_with = "deserialize_string_to_option_u64")]
    next_funding_time: Option<u64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    funding_rate: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    bid1_price: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    bid1_size: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    ask1_price: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    ask1_size: Option<f64>,
}

impl LinearTickers {
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn tick_direction(&self) -> &str {
        &self.tick_direction
    }

    pub fn set_tick_direction(&mut self, tick_direction: String) {
        self.tick_direction = tick_direction;
    }

    pub fn price24h_pcnt(&self) -> Option<f64> {
        self.price24h_pcnt
    }

    pub fn set_price24h_pcnt(&mut self, price24h_pcnt: f64) {
        self.price24h_pcnt = Some(price24h_pcnt);
    }

    pub fn last_price(&self) -> Option<f64> {
        self.last_price
    }

    pub fn set_last_price(&mut self, last_price: f64) {
        self.last_price = Some(last_price);
    }

    pub fn prev_price24h(&self) -> Option<f64> {
        self.prev_price24h
    }

    pub fn set_prev_price24h(&mut self, prev_price24h: f64) {
        self.prev_price24h = Some(prev_price24h);
    }

    pub fn high_price24h(&self) -> Option<f64> {
        self.high_price24h
    }

    pub fn set_high_price24h(&mut self, high_price24h: f64) {
        self.high_price24h = Some(high_price24h);
    }

    pub fn low_price24h(&self) -> Option<f64> {
        self.low_price24h
    }

    pub fn set_low_price24h(&mut self, low_price24h: f64) {
        self.low_price24h = Some(low_price24h);
    }

    pub fn prev_price1h(&self) -> Option<f64> {
        self.prev_price1h
    }

    pub fn set_prev_price1h(&mut self, prev_price1h: f64) {
        self.prev_price1h = Some(prev_price1h);
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

    pub fn set_index_price(&mut self, index_price: f64) {
        self.index_price = Some(index_price);
    }

    pub fn open_interest(&self) -> Option<f64> {
        self.open_interest
    }

    pub fn set_open_interest(&mut self, open_interest: f64) {
        self.open_interest = Some(open_interest);
    }

    pub fn open_interest_value(&self) -> Option<f64> {
        self.open_interest_value
    }

    pub fn set_open_interest_value(&mut self, open_interest_value: f64) {
        self.open_interest_value = Some(open_interest_value);
    }

    pub fn turnover24h(&self) -> Option<f64> {
        self.turnover24h
    }

    pub fn set_turnover24h(&mut self, turnover24h: f64) {
        self.turnover24h = Some(turnover24h);
    }

    pub fn volume24h(&self) -> Option<f64> {
        self.volume24h
    }

    pub fn set_volume24h(&mut self, volume24h: f64) {
        self.volume24h = Some(volume24h);
    }

    pub fn next_funding_time(&self) -> Option<u64> {
        self.next_funding_time
    }

    pub fn set_next_funding_time(&mut self, next_funding_time: u64) {
        self.next_funding_time = Some(next_funding_time);
    }

    pub fn funding_rate(&self) -> Option<f64> {
        self.funding_rate
    }

    pub fn set_funding_rate(&mut self, funding_rate: f64) {
        self.funding_rate = Some(funding_rate);
    }

    pub fn bid1_price(&self) -> Option<f64> {
        self.bid1_price
    }

    pub fn set_bid1_price(&mut self, bid1_price: f64) {
        self.bid1_price = Some(bid1_price);
    }

    pub fn bid1_size(&self) -> Option<f64> {
        self.bid1_size
    }

    pub fn set_bid1_size(&mut self, bid1_size: f64) {
        self.bid1_size = Some(bid1_size);
    }

    pub fn ask1_price(&self) -> Option<f64> {
        self.ask1_price
    }

    pub fn set_ask1_price(&mut self, ask1_price: f64) {
        self.ask1_price = Some(ask1_price);
    }

    pub fn ask1_size(&self) -> Option<f64> {
        self.ask1_size
    }

    pub fn set_ask1_size(&mut self, ask1_size: f64) {
        self.ask1_size = Some(ask1_size);
    }
}
