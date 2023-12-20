use serde::{
    Deserialize,
    Serialize,
};
use crate::utils::{
    deserialize_f64,
    deserialize_option_f64,
    deserialize_string_to_u64,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InverseTickersResult {
    list: Vec<InverseTickers>,
}
impl InverseTickersResult {
    pub fn list(&self) -> &Vec<InverseTickers> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<InverseTickers>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InverseTickers {
    symbol: String,
    #[serde(deserialize_with = "deserialize_f64")]
    last_price: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    index_price: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    mark_price: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    prev_price24h: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    price24h_pcnt: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    high_price24h: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    low_price24h: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    prev_price1h: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    open_interest: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    open_interest_value: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    turnover24h: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    volume24h: f64,
    #[serde(deserialize_with = "deserialize_option_f64")]
    funding_rate: Option<f64>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    next_funding_time: u64,
    #[serde(deserialize_with = "deserialize_option_f64")]
    predicted_delivery_price: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    basis_rate: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    delivery_fee_rate: Option<f64>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    delivery_time: u64,
    #[serde(deserialize_with = "deserialize_f64")]
    ask1_size: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    bid1_price: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    ask1_price: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    bid1_size: f64,
    #[serde(deserialize_with = "deserialize_option_f64")]
    basis: Option<f64>,
}

impl InverseTickers {
    pub fn symbol(&self) -> &String {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn last_price(&self) -> f64 {
        self.last_price
    }

    pub fn set_last_price(&mut self, last_price: f64) {
        self.last_price = last_price;
    }

    pub fn index_price(&self) -> f64 {
        self.index_price
    }

    pub fn set_index_price(&mut self, index_price: f64) {
        self.index_price = index_price;
    }

    pub fn mark_price(&self) -> f64 {
        self.mark_price
    }

    pub fn set_mark_price(&mut self, mark_price: f64) {
        self.mark_price = mark_price;
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

    pub fn prev_price1h(&self) -> f64 {
        self.prev_price1h
    }

    pub fn set_prev_price1h(&mut self, prev_price1h: f64) {
        self.prev_price1h = prev_price1h;
    }

    pub fn open_interest(&self) -> f64 {
        self.open_interest
    }

    pub fn set_open_interest(&mut self, open_interest: f64) {
        self.open_interest = open_interest;
    }

    pub fn open_interest_value(&self) -> f64 {
        self.open_interest_value
    }

    pub fn set_open_interest_value(&mut self, open_interest_value: f64) {
        self.open_interest_value = open_interest_value;
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

    pub fn funding_rate(&self) -> Option<f64> {
        self.funding_rate
    }

    pub fn set_funding_rate(&mut self, funding_rate: f64) {
        self.funding_rate = Some(funding_rate);
    }

    pub fn next_funding_time(&self) -> u64 {
        self.next_funding_time
    }

    pub fn set_next_funding_time(&mut self, next_funding_time: u64) {
        self.next_funding_time = next_funding_time;
    }

    pub fn predicted_delivery_price(&self) -> Option<f64> {
        self.predicted_delivery_price
    }

    pub fn set_predicted_delivery_price(&mut self, predicted_delivery_price: f64) {
        self.predicted_delivery_price = Some(predicted_delivery_price);
    }

    pub fn basis_rate(&self) -> Option<f64> {
        self.basis_rate
    }

    pub fn set_basis_rate(&mut self, basis_rate: f64) {
        self.basis_rate = Some(basis_rate);
    }

    pub fn delivery_fee_rate(&self) -> Option<f64> {
        self.delivery_fee_rate
    }

    pub fn set_delivery_fee_rate(&mut self, delivery_fee_rate: f64) {
        self.delivery_fee_rate = Some(delivery_fee_rate);
    }

    pub fn delivery_time(&self) -> u64 {
        self.delivery_time
    }

    pub fn set_delivery_time(&mut self, delivery_time: u64) {
        self.delivery_time = delivery_time;
    }

    pub fn ask1_size(&self) -> f64 {
        self.ask1_size
    }

    pub fn set_ask1_size(&mut self, ask1_size: f64) {
        self.ask1_size = ask1_size;
    }

    pub fn bid1_price(&self) -> f64 {
        self.bid1_price
    }

    pub fn set_bid1_price(&mut self, bid1_price: f64) {
        self.bid1_price = bid1_price;
    }

    pub fn ask1_price(&self) -> f64 {
        self.ask1_price
    }

    pub fn set_ask1_price(&mut self, ask1_price: f64) {
        self.ask1_price = ask1_price;
    }

    pub fn bid1_size(&self) -> f64 {
        self.bid1_size
    }

    pub fn set_bid1_size(&mut self, bid1_size: f64) {
        self.bid1_size = bid1_size;
    }

    pub fn basis(&self) -> Option<f64> {
        self.basis
    }

    pub fn set_basis(&mut self, basis: f64) {
        self.basis = Some(basis);
    }
}