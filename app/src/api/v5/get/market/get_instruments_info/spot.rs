use serde::{
    Deserialize,
    Serialize,
};
use crate::utils::deserialize_f64;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotInstrumentsInfoResult {
    list: Vec<SpotInstrumentInfo>,
    next_page_cursor: Option<String>,
}
impl SpotInstrumentsInfoResult {
    pub fn list(&self) -> &Vec<SpotInstrumentInfo> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<SpotInstrumentInfo>) {
        self.list = list;
    }

    pub fn next_page_cursor(&self) -> &Option<String> {
        &self.next_page_cursor
    }

    pub fn set_next_page_cursor(&mut self, next_page_cursor: Option<String>) {
        self.next_page_cursor = next_page_cursor;
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotInstrumentInfo {
    symbol: String,
    status: String,
    base_coin: String,
    quote_coin: String,
    innovation: String,
    margin_trading: String,
    price_filter: SpotPriceFilter,
    lot_size_filter: SpotLotSizeFilter,
}
impl SpotInstrumentInfo {
    pub fn symbol(&self) -> &String {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn status(&self) -> &String {
        &self.status
    }

    pub fn set_status(&mut self, status: String) {
        self.status = status;
    }

    pub fn base_coin(&self) -> &String {
        &self.base_coin
    }

    pub fn set_base_coin(&mut self, base_coin: String) {
        self.base_coin = base_coin;
    }

    pub fn quote_coin(&self) -> &String {
        &self.quote_coin
    }

    pub fn set_quote_coin(&mut self, quote_coin: String) {
        self.quote_coin = quote_coin;
    }

    pub fn innovation(&self) -> &String {
        &self.innovation
    }

    pub fn set_innovation(&mut self, innovation: String) {
        self.innovation = innovation;
    }

    pub fn margin_trading(&self) -> &String {
        &self.margin_trading
    }

    pub fn set_margin_trading(&mut self, margin_trading: String) {
        self.margin_trading = margin_trading;
    }

    pub fn price_filter(&self) -> &SpotPriceFilter {
        &self.price_filter
    }

    pub fn set_price_filter(&mut self, price_filter: SpotPriceFilter) {
        self.price_filter = price_filter;
    }

    pub fn lot_size_filter(&self) -> &SpotLotSizeFilter {
        &self.lot_size_filter
    }

    pub fn set_lot_size_filter(&mut self, lot_size_filter: SpotLotSizeFilter) {
        self.lot_size_filter = lot_size_filter;
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotPriceFilter {
    #[serde(deserialize_with = "deserialize_f64")]
    tick_size: f64,
}

impl SpotPriceFilter {
    pub fn tick_size(&self) -> f64 {
        self.tick_size
    }

    pub fn set_tick_size(&mut self, tick_size: f64) {
        self.tick_size = tick_size;
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotLotSizeFilter {
    #[serde(deserialize_with = "deserialize_f64")]
    base_precision: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    quote_precision: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    min_order_amt: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    max_order_amt: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    max_order_qty: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    min_order_qty: f64,
}
impl SpotLotSizeFilter {
    pub fn base_precision(&self) -> f64 {
        self.base_precision
    }

    pub fn set_base_precision(&mut self, base_precision: f64) {
        self.base_precision = base_precision;
    }

    pub fn quote_precision(&self) -> f64 {
        self.quote_precision
    }

    pub fn set_quote_precision(&mut self, quote_precision: f64) {
        self.quote_precision = quote_precision;
    }

    pub fn min_order_amt(&self) -> f64 {
        self.min_order_amt
    }

    pub fn set_min_order_amt(&mut self, min_order_amt: f64) {
        self.min_order_amt = min_order_amt;
    }

    pub fn max_order_amt(&self) -> f64 {
        self.max_order_amt
    }

    pub fn set_max_order_amt(&mut self, max_order_amt: f64) {
        self.max_order_amt = max_order_amt;
    }

    pub fn max_order_qty(&self) -> f64 {
        self.max_order_qty
    }

    pub fn set_max_order_qty(&mut self, max_order_qty: f64) {
        self.max_order_qty = max_order_qty;
    }

    pub fn min_order_qty(&self) -> f64 {
        self.min_order_qty
    }

    pub fn set_min_order_qty(&mut self, min_order_qty: f64) {
        self.min_order_qty = min_order_qty;
    }
}
