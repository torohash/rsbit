use crate::{
    v5::ws::BybitWS,
    constants::PUBLIC_KLINE_TOPIC,
    utils::deserialize_f64,
};
use serde::Deserialize;

impl BybitWS {
    pub fn add_kline_args(&mut self, interval: &str, symbol: &str) {
        self.args.push(format!("{}.{}.{}", PUBLIC_KLINE_TOPIC, interval, symbol));
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKlineResponse {
    topic: String,
    #[serde(rename = "type")]
    type_field: String,
    ts: u64,
    data: Vec<PublicKlineData>,
}

impl PublicKlineResponse {
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

    pub fn data(&self) -> &Vec<PublicKlineData> {
        &self.data
    }

    pub fn set_data(&mut self, data: Vec<PublicKlineData>) {
        self.data = data;
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PublicKlineData {
    start: u64,
    end: u64,
    interval: String,
    #[serde(deserialize_with = "deserialize_f64")]
    open: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    close: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    high: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    low: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    volume: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    turnover: f64,
    confirm: bool,
    timestamp: u64,
}

impl PublicKlineData {
    pub fn start(&self) -> u64 {
        self.start
    }

    pub fn set_start(&mut self, start: u64) {
        self.start = start;
    }

    pub fn end(&self) -> u64 {
        self.end
    }

    pub fn set_end(&mut self, end: u64) {
        self.end = end;
    }

    pub fn interval(&self) -> &str {
        &self.interval
    }

    pub fn set_interval(&mut self, interval: String) {
        self.interval = interval;
    }

    pub fn open(&self) -> f64 {
        self.open
    }

    pub fn set_open(&mut self, open: f64) {
        self.open = open;
    }

    pub fn close(&self) -> f64 {
        self.close
    }

    pub fn set_close(&mut self, close: f64) {
        self.close = close;
    }

    pub fn high(&self) -> f64 {
        self.high
    }

    pub fn set_high(&mut self, high: f64) {
        self.high = high;
    }

    pub fn low(&self) -> f64 {
        self.low
    }

    pub fn set_low(&mut self, low: f64) {
        self.low = low;
    }

    pub fn volume(&self) -> f64 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: f64) {
        self.volume = volume;
    }

    pub fn turnover(&self) -> f64 {
        self.turnover
    }

    pub fn set_turnover(&mut self, turnover: f64) {
        self.turnover = turnover;
    }

    pub fn confirm(&self) -> bool {
        self.confirm
    }

    pub fn set_confirm(&mut self, confirm: bool) {
        self.confirm = confirm;
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn set_timestamp(&mut self, timestamp: u64) {
        self.timestamp = timestamp;
    }
}