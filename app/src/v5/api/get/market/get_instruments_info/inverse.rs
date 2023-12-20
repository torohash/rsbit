use serde::{
    Deserialize,
    Serialize,
};
use crate::utils::deserialize_f64;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InverseInstrumentsInfoResult {
    list: Vec<InverseInstrumentInfo>,
    next_page_cursor: String,
}
impl InverseInstrumentsInfoResult {
    pub fn list(&self) -> &Vec<InverseInstrumentInfo> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<InverseInstrumentInfo>) {
        self.list = list;
    }

    pub fn next_page_cursor(&self) -> &String {
        &self.next_page_cursor
    }

    pub fn set_next_page_cursor(&mut self, next_page_cursor: String) {
        self.next_page_cursor = next_page_cursor;
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InverseInstrumentInfo {
    symbol: String,
    contract_type: String,
    status: String,
    base_coin: String,
    quote_coin: String,
    launch_time: String,
    delivery_time: String,
    delivery_fee_rate: String,
    price_scale: String,
    leverage_filter: InverseLeverageFilter,
    price_filter: InversePriceFilter,
    lot_size_filter: InverseLotSizeFilter,
    unified_margin_trade: bool,
    funding_interval: i32,
    settle_coin: String,
}
impl InverseInstrumentInfo {
    pub fn symbol(&self) -> &String {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn contract_type(&self) -> &String {
        &self.contract_type
    }

    pub fn set_contract_type(&mut self, contract_type: String) {
        self.contract_type = contract_type;
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

    pub fn launch_time(&self) -> &String {
        &self.launch_time
    }

    pub fn set_launch_time(&mut self, launch_time: String) {
        self.launch_time = launch_time;
    }

    pub fn delivery_time(&self) -> &String {
        &self.delivery_time
    }

    pub fn set_delivery_time(&mut self, delivery_time: String) {
        self.delivery_time = delivery_time;
    }

    pub fn delivery_fee_rate(&self) -> &String {
        &self.delivery_fee_rate
    }

    pub fn set_delivery_fee_rate(&mut self, delivery_fee_rate: String) {
        self.delivery_fee_rate = delivery_fee_rate;
    }

    pub fn price_scale(&self) -> &String {
        &self.price_scale
    }

    pub fn set_price_scale(&mut self, price_scale: String) {
        self.price_scale = price_scale;
    }

    pub fn leverage_filter(&self) -> &InverseLeverageFilter {
        &self.leverage_filter
    }

    pub fn set_leverage_filter(&mut self, leverage_filter: InverseLeverageFilter) {
        self.leverage_filter = leverage_filter;
    }

    pub fn price_filter(&self) -> &InversePriceFilter {
        &self.price_filter
    }

    pub fn set_price_filter(&mut self, price_filter: InversePriceFilter) {
        self.price_filter = price_filter;
    }

    pub fn lot_size_filter(&self) -> &InverseLotSizeFilter {
        &self.lot_size_filter
    }

    pub fn set_lot_size_filter(&mut self, lot_size_filter: InverseLotSizeFilter) {
        self.lot_size_filter = lot_size_filter;
    }

    pub fn unified_margin_trade(&self) -> bool {
        self.unified_margin_trade
    }

    pub fn set_unified_margin_trade(&mut self, unified_margin_trade: bool) {
        self.unified_margin_trade = unified_margin_trade;
    }

    pub fn funding_interval(&self) -> i32 {
        self.funding_interval
    }

    pub fn set_funding_interval(&mut self, funding_interval: i32) {
        self.funding_interval = funding_interval;
    }

    pub fn settle_coin(&self) -> &String {
        &self.settle_coin
    }

    pub fn set_settle_coin(&mut self, settle_coin: String) {
        self.settle_coin = settle_coin;
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InverseLeverageFilter {
    #[serde(deserialize_with = "deserialize_f64")]
    min_leverage: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    max_leverage: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    leverage_step: f64,
}

impl InverseLeverageFilter {
    pub fn min_leverage(&self) -> f64 {
        self.min_leverage
    }

    pub fn set_min_leverage(&mut self, min_leverage: f64) {
        self.min_leverage = min_leverage;
    }

    pub fn max_leverage(&self) -> f64 {
        self.max_leverage
    }

    pub fn set_max_leverage(&mut self, max_leverage: f64) {
        self.max_leverage = max_leverage;
    }

    pub fn leverage_step(&self) -> f64 {
        self.leverage_step
    }

    pub fn set_leverage_step(&mut self, leverage_step: f64) {
        self.leverage_step = leverage_step;
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InversePriceFilter {
    #[serde(deserialize_with = "deserialize_f64")]
    min_price: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    max_price: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    tick_size: f64,
}

impl InversePriceFilter {
    pub fn min_price(&self) -> f64 {
        self.min_price
    }

    pub fn set_min_price(&mut self, min_price: f64) {
        self.min_price = min_price;
    }

    pub fn max_price(&self) -> f64 {
        self.max_price
    }

    pub fn set_max_price(&mut self, max_price: f64) {
        self.max_price = max_price;
    }

    pub fn tick_size(&self) -> f64 {
        self.tick_size
    }

    pub fn set_tick_size(&mut self, tick_size: f64) {
        self.tick_size = tick_size;
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InverseLotSizeFilter {
    #[serde(deserialize_with = "deserialize_f64")]
    max_order_qty: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    min_order_qty: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    qty_step: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    post_only_max_order_qty: f64,
}

impl InverseLotSizeFilter {
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

    pub fn qty_step(&self) -> f64 {
        self.qty_step
    }

    pub fn set_qty_step(&mut self, qty_step: f64) {
        self.qty_step = qty_step;
    }

    pub fn post_only_max_order_qty(&self) -> f64 {
        self.post_only_max_order_qty
    }

    pub fn set_post_only_max_order_qty(&mut self, post_only_max_order_qty: f64) {
        self.post_only_max_order_qty = post_only_max_order_qty;
    }
}