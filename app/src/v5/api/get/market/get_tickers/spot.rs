use serde::{
    Deserialize,
    Serialize,
};
use crate::utils::{
    deserialize_f64,
    deserialize_option_f64,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotTickersResult {
    list: Vec<SpotTickers>,
}
impl SpotTickersResult {
    pub fn list(&self) -> &Vec<SpotTickers> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<SpotTickers>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotTickers {
    symbol: String,
    #[serde(deserialize_with = "deserialize_option_f64")]
    bid1_price: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    bid1_size: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    ask1_price: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    ask1_size: Option<f64>,
    #[serde(deserialize_with = "deserialize_f64")]
    last_price: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    prev_price24h: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    price24h_pcnt: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    high_price24h: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    low_price24h: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    turnover24h: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    volume24h: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    usd_index_price: f64,
}

impl SpotTickers {
    pub fn symbol(&self) -> &String {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
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

    pub fn last_price(&self) -> f64 {
        self.last_price
    }

    pub fn set_last_price(&mut self, last_price: f64) {
        self.last_price = last_price;
    }

    pub fn prev_price24h(&self) -> f64 {
        self.prev_price24h
    }

    pub fn set_prev_price24h(&mut self, prev_price24h: f64) {
        self.prev_price24h = prev_price24h;
    }

    pub fn price24h_pcnt(&self) -> f64 {
        self.price24h_pcnt
    }

    pub fn set_price24h_pcnt(&mut self, price24h_pcnt: f64) {
        self.price24h_pcnt = price24h_pcnt;
    }

    pub fn high_price24h(&self) -> f64 {
        self.high_price24h
    }

    pub fn set_high_price24h(&mut self, high_price24h: f64) {
        self.high_price24h = high_price24h;
    }

    pub fn low_price24h(&self) -> f64 {
        self.low_price24h
    }

    pub fn set_low_price24h(&mut self, low_price24h: f64) {
        self.low_price24h = low_price24h;
    }

    pub fn turnover24h(&self) -> f64 {
        self.turnover24h
    }

    pub fn set_turnover24h(&mut self, turnover24h: f64) {
        self.turnover24h = turnover24h;
    }

    pub fn volume24h(&self) -> f64 {
        self.volume24h
    }

    pub fn set_volume24h(&mut self, volume24h: f64) {
        self.volume24h = volume24h;
    }

    pub fn usd_index_price(&self) -> f64 {
        self.usd_index_price
    }

    pub fn set_usd_index_price(&mut self, usd_index_price: f64) {
        self.usd_index_price = usd_index_price;
    }
}
