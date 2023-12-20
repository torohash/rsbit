use serde::{
    Deserialize,
    Serialize,
};
use crate::utils::deserialize_f64;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionTickersResult {
    list: Vec<OptionTickers>,
}
impl OptionTickersResult {
    pub fn list(&self) -> &Vec<OptionTickers> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<OptionTickers>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionTickers {
    symbol: String,
    #[serde(deserialize_with = "deserialize_f64")]
    bid1_price: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    bid1_size: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    bid1_iv: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    ask1_price: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    ask1_size: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    ask1_iv: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    last_price: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    high_price24h: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    low_price24h: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    mark_price: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    index_price: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    mark_iv: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    underlying_price: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    open_interest: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    turnover24h: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    volume24h: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    total_volume: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    total_turnover: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    delta: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    gamma: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    vega: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    theta: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    predicted_delivery_price: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    change24h: f64,
}

impl OptionTickers {
    pub fn symbol(&self) -> &String {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }
    
    pub fn bid1_price(&self) -> f64 {
        self.bid1_price
    }

    pub fn set_bid1_price(&mut self, bid1_price: f64) {
        self.bid1_price = bid1_price;
    }

    pub fn bid1_size(&self) -> f64 {
        self.bid1_size
    }

    pub fn set_bid1_size(&mut self, bid1_size: f64) {
        self.bid1_size = bid1_size;
    }

    pub fn bid1_iv(&self) -> f64 {
        self.bid1_iv
    }

    pub fn set_bid1_iv(&mut self, bid1_iv: f64) {
        self.bid1_iv = bid1_iv;
    }

    pub fn ask1_price(&self) -> f64 {
        self.ask1_price
    }

    pub fn set_ask1_price(&mut self, ask1_price: f64) {
        self.ask1_price = ask1_price;
    }

    pub fn ask1_size(&self) -> f64 {
        self.ask1_size
    }

    pub fn set_ask1_size(&mut self, ask1_size: f64) {
        self.ask1_size = ask1_size;
    }

    pub fn ask1_iv(&self) -> f64 {
        self.ask1_iv
    }

    pub fn set_ask1_iv(&mut self, ask1_iv: f64) {
        self.ask1_iv = ask1_iv;
    }

    pub fn last_price(&self) -> f64 {
        self.last_price
    }

    pub fn set_last_price(&mut self, last_price: f64) {
        self.last_price = last_price;
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

    pub fn mark_price(&self) -> f64 {
        self.mark_price
    }

    pub fn set_mark_price(&mut self, mark_price: f64) {
        self.mark_price = mark_price;
    }

    pub fn index_price(&self) -> f64 {
        self.index_price
    }

    pub fn set_index_price(&mut self, index_price: f64) {
        self.index_price = index_price;
    }

    pub fn mark_iv(&self) -> f64 {
        self.mark_iv
    }

    pub fn set_mark_iv(&mut self, mark_iv: f64) {
        self.mark_iv = mark_iv;
    }

    pub fn underlying_price(&self) -> f64 {
        self.underlying_price
    }

    pub fn set_underlying_price(&mut self, underlying_price: f64) {
        self.underlying_price = underlying_price;
    }

    pub fn open_interest(&self) -> f64 {
        self.open_interest
    }

    pub fn set_open_interest(&mut self, open_interest: f64) {
        self.open_interest = open_interest;
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

    pub fn total_volume(&self) -> f64 {
        self.total_volume
    }

    pub fn set_total_volume(&mut self, total_volume: f64) {
        self.total_volume = total_volume;
    }

    pub fn total_turnover(&self) -> f64 {
        self.total_turnover
    }

    pub fn set_total_turnover(&mut self, total_turnover: f64) {
        self.total_turnover = total_turnover;
    }

    pub fn delta(&self) -> f64 {
        self.delta
    }

    pub fn set_delta(&mut self, delta: f64) {
        self.delta = delta;
    }

    pub fn gamma(&self) -> f64 {
        self.gamma
    }

    pub fn set_gamma(&mut self, gamma: f64) {
        self.gamma = gamma;
    }

    pub fn vega(&self) -> f64 {
        self.vega
    }

    pub fn set_vega(&mut self, vega: f64) {
        self.vega = vega;
    }

    pub fn theta(&self) -> f64 {
        self.theta
    }

    pub fn set_theta(&mut self, theta: f64) {
        self.theta = theta;
    }

    pub fn predicted_delivery_price(&self) -> f64 {
        self.predicted_delivery_price
    }

    pub fn set_predicted_delivery_price(&mut self, predicted_delivery_price: f64) {
        self.predicted_delivery_price = predicted_delivery_price;
    }

    pub fn change24h(&self) -> f64 {
        self.change24h
    }

    pub fn set_change24h(&mut self, change24h: f64) {
        self.change24h = change24h;
    }
}