use crate::utils::deserialize_option_f64;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicOptionTickersResponse {
    id: String,
    topic: String,
    #[serde(rename = "type")]
    type_field: String,
    data: OptionTickers,
    ts: u64,
}

impl PublicOptionTickersResponse {
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

    pub fn type_field(&self) -> &str {
        &self.type_field
    }

    pub fn set_type_field(&mut self, type_field: String) {
        self.type_field = type_field;
    }

    pub fn data(&self) -> &OptionTickers {
        &self.data
    }

    pub fn set_data(&mut self, data: OptionTickers) {
        self.data = data;
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
pub struct OptionTickers {
    symbol: String,
    #[serde(deserialize_with = "deserialize_option_f64")]
    bid_price: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    bid_size: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    bid_iv: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    ask_price: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    ask_size: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    ask_iv: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    last_price: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    high_price24h: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    low_price24h: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    mark_price: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    index_price: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    underlying_price: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    open_interest: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    turnover24h: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    volume24h: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    total_volume: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    total_turnover: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    delta: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    gamma: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    vega: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    theta: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    predicted_delivery_price: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    change24h: Option<f64>,
}

impl OptionTickers {
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn bid_price(&self) -> Option<f64> {
        self.bid_price
    }

    pub fn set_bid_price(&mut self, bid_price: Option<f64>) {
        self.bid_price = bid_price;
    }

    pub fn bid_size(&self) -> Option<f64> {
        self.bid_size
    }

    pub fn set_bid_size(&mut self, bid_size: Option<f64>) {
        self.bid_size = bid_size;
    }

    pub fn bid_iv(&self) -> Option<f64> {
        self.bid_iv
    }

    pub fn set_bid_iv(&mut self, bid_iv: Option<f64>) {
        self.bid_iv = bid_iv;
    }

    pub fn ask_price(&self) -> Option<f64> {
        self.ask_price
    }

    pub fn set_ask_price(&mut self, ask_price: Option<f64>) {
        self.ask_price = ask_price;
    }

    pub fn ask_size(&self) -> Option<f64> {
        self.ask_size
    }

    pub fn set_ask_size(&mut self, ask_size: Option<f64>) {
        self.ask_size = ask_size;
    }

    pub fn ask_iv(&self) -> Option<f64> {
        self.ask_iv
    }

    pub fn set_ask_iv(&mut self, ask_iv: Option<f64>) {
        self.ask_iv = ask_iv;
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

    pub fn mark_price(&self) -> Option<f64> {
        self.mark_price
    }

    pub fn set_mark_price(&mut self, mark_price: Option<f64>) {
        self.mark_price = mark_price;
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

    pub fn open_interest(&self) -> Option<f64> {
        self.open_interest
    }

    pub fn set_open_interest(&mut self, open_interest: Option<f64>) {
        self.open_interest = open_interest;
    }

    pub fn turnover24h(&self) -> Option<f64> {
        self.turnover24h
    }

    pub fn set_turnover24h(&mut self, turnover24h: Option<f64>) {
        self.turnover24h = turnover24h;
    }

    pub fn volume24h(&self) -> Option<f64> {
        self.volume24h
    }

    pub fn set_volume24h(&mut self, volume24h: Option<f64>) {
        self.volume24h = volume24h;
    }

    pub fn total_volume(&self) -> Option<f64> {
        self.total_volume
    }

    pub fn set_total_volume(&mut self, total_volume: Option<f64>) {
        self.total_volume = total_volume;
    }

    pub fn total_turnover(&self) -> Option<f64> {
        self.total_turnover
    }

    pub fn set_total_turnover(&mut self, total_turnover: Option<f64>) {
        self.total_turnover = total_turnover;
    }

    pub fn delta(&self) -> Option<f64> {
        self.delta
    }

    pub fn set_delta(&mut self, delta: Option<f64>) {
        self.delta = delta;
    }

    pub fn gamma(&self) -> Option<f64> {
        self.gamma
    }

    pub fn set_gamma(&mut self, gamma: Option<f64>) {
        self.gamma = gamma;
    }

    pub fn vega(&self) -> Option<f64> {
        self.vega
    }

    pub fn set_vega(&mut self, vega: Option<f64>) {
        self.vega = vega;
    }

    pub fn theta(&self) -> Option<f64> {
        self.theta
    }

    pub fn set_theta(&mut self, theta: Option<f64>) {
        self.theta = theta;
    }

    pub fn predicted_delivery_price(&self) -> Option<f64> {
        self.predicted_delivery_price
    }

    pub fn set_predicted_delivery_price(&mut self, predicted_delivery_price: Option<f64>) {
        self.predicted_delivery_price = predicted_delivery_price;
    }

    pub fn change24h(&self) -> Option<f64> {
        self.change24h
    }

    pub fn set_change24h(&mut self, change24h: Option<f64>) {
        self.change24h = change24h;
    }

}