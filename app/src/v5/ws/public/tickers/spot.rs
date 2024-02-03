use crate::utils::deserialize_option_f64;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicSpotTickersResponse {
    topic: String,
    #[serde(rename = "type")]
    type_field: String,
    data: SpotTickers,
    cs: u64,
    ts: u64,
}

impl PublicSpotTickersResponse {
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

    pub fn data(&self) -> &SpotTickers {
        &self.data
    }

    pub fn set_data(&mut self, data: SpotTickers) {
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
pub struct SpotTickers {
    symbol: String,
    #[serde(deserialize_with = "deserialize_option_f64")]
    last_price: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    high_price24h: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    low_price24h: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    prev_price24h: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    volume24h: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    turnover24h: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    price24h_pcnt: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    usd_index_price: Option<f64>,
}

impl SpotTickers {
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn last_price(&self) -> Option<f64> {
        self.last_price
    }

    pub fn set_last_price(&mut self, last_price: Option<f64>) {
        self.last_price = last_price;
    }

    pub fn high_price24h(&self) -> Option<f64> {
        self.high_price24h
    }

    pub fn set_high_price24h(&mut self, high_price24h: Option<f64>) {
        self.high_price24h = high_price24h;
    }

    pub fn low_price24h(&self) -> Option<f64> {
        self.low_price24h
    }

    pub fn set_low_price24h(&mut self, low_price24h: Option<f64>) {
        self.low_price24h = low_price24h;
    }

    pub fn prev_price24h(&self) -> Option<f64> {
        self.prev_price24h
    }

    pub fn set_prev_price24h(&mut self, prev_price24h: Option<f64>) {
        self.prev_price24h = prev_price24h;
    }

    pub fn volume24h(&self) -> Option<f64> {
        self.volume24h
    }

    pub fn set_volume24h(&mut self, volume24h: Option<f64>) {
        self.volume24h = volume24h;
    }

    pub fn turnover24h(&self) -> Option<f64> {
        self.turnover24h
    }

    pub fn set_turnover24h(&mut self, turnover24h: Option<f64>) {
        self.turnover24h = turnover24h;
    }

    pub fn price24h_pcnt(&self) -> Option<f64> {
        self.price24h_pcnt
    }

    pub fn set_price24h_pcnt(&mut self, price24h_pcnt: Option<f64>) {
        self.price24h_pcnt = price24h_pcnt;
    }

    pub fn usd_index_price(&self) -> Option<f64> {
        self.usd_index_price
    }

    pub fn set_usd_index_price(&mut self, usd_index_price: Option<f64>) {
        self.usd_index_price = usd_index_price;
    }
}
