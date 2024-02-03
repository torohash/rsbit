use crate::{
    v5::ws::BybitWS,
    constants::PRIVATE_POSITION_TOPIC,
    utils::{
        deserialize_f64,
        deserialize_string_to_u64,
        deserialize_option_f64,
    },
};
use serde::Deserialize;

pub enum PositionCategory {
    Linear,
    Inverse,
    Option,
}

impl BybitWS {
    pub fn add_position_args(&mut self, category: Option<PositionCategory>) {
        match category {
            Some(category) => {
                self.args.push(match category {
                    PositionCategory::Linear => format!("{}.{}", PRIVATE_POSITION_TOPIC, "linear"),
                    PositionCategory::Inverse => format!("{}.{}", PRIVATE_POSITION_TOPIC, "inverse"),
                    PositionCategory::Option => format!("{}.{}", PRIVATE_POSITION_TOPIC, "option"),
                });
            },
            None => {
                self.args.push(PRIVATE_POSITION_TOPIC.to_string());
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivatePositionResponse {
    id: String,
    topic: String,
    creation_time: u64,
    data: Vec<PrivatePositionData>,
}

impl PrivatePositionResponse {
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

    pub fn creation_time(&self) -> u64 {
        self.creation_time
    }

    pub fn set_creation_time(&mut self, creation_time: u64) {
        self.creation_time = creation_time;
    }

    pub fn data(&self) -> &Vec<PrivatePositionData> {
        &self.data
    }

    pub fn set_data(&mut self, data: Vec<PrivatePositionData>) {
        self.data = data;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivatePositionData {
    position_idx: u64,
    trade_mode: u8,
    risk_id: i64,
    #[serde(deserialize_with = "deserialize_option_f64")]
    risk_limit_value: Option<f64>,
    symbol: String,
    side: String,
    #[serde(deserialize_with = "deserialize_f64")]
    size: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    entry_price: f64,
    #[serde(deserialize_with = "deserialize_option_f64")]
    leverage: Option<f64>,
    #[serde(deserialize_with = "deserialize_f64")]
    position_value: f64,
    #[serde(deserialize_with = "deserialize_option_f64")]
    position_balance: Option<f64>,
    #[serde(deserialize_with = "deserialize_f64")]
    mark_price: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    position_i_m: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    position_m_m: f64,
    #[serde(deserialize_with = "deserialize_option_f64")]
    take_profit: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    stop_loss: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    trailing_stop: Option<f64>,
    #[serde(deserialize_with = "deserialize_f64")]
    unrealised_pnl: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    cum_realised_pnl: f64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    created_time: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    updated_time: u64,
    tpsl_mode: Option<String>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    liq_price: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    bust_price: Option<f64>,
    category: String,
    position_status: String,
    adl_rank_indicator: u8,
    auto_add_margin: u8,
    leverage_sys_updated_time: Option<String>,
    mmr_sys_updated_time: Option<String>,
    seq: u64,
    is_reduce_only: bool,
}

impl PrivatePositionData {
    pub fn position_idx(&self) -> u64 {
        self.position_idx
    }

    pub fn set_position_idx(&mut self, position_idx: u64) {
        self.position_idx = position_idx;
    }

    pub fn trade_mode(&self) -> u8 {
        self.trade_mode
    }

    pub fn set_trade_mode(&mut self, trade_mode: u8) {
        self.trade_mode = trade_mode;
    }

    pub fn risk_id(&self) -> i64 {
        self.risk_id
    }

    pub fn set_risk_id(&mut self, risk_id: i64) {
        self.risk_id = risk_id;
    }

    pub fn risk_limit_value(&self) -> Option<f64> {
        self.risk_limit_value
    }

    pub fn set_risk_limit_value(&mut self, risk_limit_value: f64) {
        self.risk_limit_value = Some(risk_limit_value);
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }

    pub fn side(&self) -> &str {
        &self.side
    }

    pub fn set_side(&mut self, side: String) {
        self.side = side;
    }

    pub fn size(&self) -> f64 {
        self.size
    }

    pub fn set_size(&mut self, size: f64) {
        self.size = size;
    }

    pub fn entry_price(&self) -> f64 {
        self.entry_price
    }

    pub fn set_entry_price(&mut self, entry_price: f64) {
        self.entry_price = entry_price;
    }

    pub fn leverage(&self) -> Option<f64> {
        self.leverage
    }

    pub fn set_leverage(&mut self, leverage: f64) {
        self.leverage = Some(leverage);
    }

    pub fn position_value(&self) -> f64 {
        self.position_value
    }

    pub fn set_position_value(&mut self, position_value: f64) {
        self.position_value = position_value;
    }

    pub fn position_balance(&self) -> Option<f64> {
        self.position_balance
    }

    pub fn set_position_balance(&mut self, position_balance: f64) {
        self.position_balance = Some(position_balance);
    }

    pub fn mark_price(&self) -> f64 {
        self.mark_price
    }

    pub fn set_mark_price(&mut self, mark_price: f64) {
        self.mark_price = mark_price;
    }

    pub fn position_i_m(&self) -> f64 {
        self.position_i_m
    }

    pub fn set_position_i_m(&mut self, position_i_m: f64) {
        self.position_i_m = position_i_m;
    }

    pub fn position_m_m(&self) -> f64 {
        self.position_m_m
    }

    pub fn set_position_m_m(&mut self, position_m_m: f64) {
        self.position_m_m = position_m_m;
    }

    pub fn take_profit(&self) -> Option<f64> {
        self.take_profit
    }

    pub fn set_take_profit(&mut self, take_profit: f64) {
        self.take_profit = Some(take_profit);
    }

    pub fn stop_loss(&self) -> Option<f64> {
        self.stop_loss
    }

    pub fn set_stop_loss(&mut self, stop_loss: f64) {
        self.stop_loss = Some(stop_loss);
    }

    pub fn trailing_stop(&self) -> Option<f64> {
        self.trailing_stop
    }

    pub fn set_trailing_stop(&mut self, trailing_stop: f64) {
        self.trailing_stop = Some(trailing_stop);
    }

    pub fn unrealised_pnl(&self) -> f64 {
        self.unrealised_pnl
    }

    pub fn set_unrealised_pnl(&mut self, unrealised_pnl: f64) {
        self.unrealised_pnl = unrealised_pnl;
    }

    pub fn cum_realised_pnl(&self) -> f64 {
        self.cum_realised_pnl
    }

    pub fn set_cum_realised_pnl(&mut self, cum_realised_pnl: f64) {
        self.cum_realised_pnl = cum_realised_pnl;
    }

    pub fn created_time(&self) -> u64 {
        self.created_time
    }

    pub fn set_created_time(&mut self, created_time: u64) {
        self.created_time = created_time;
    }

    pub fn updated_time(&self) -> u64 {
        self.updated_time
    }

    pub fn set_updated_time(&mut self, updated_time: u64) {
        self.updated_time = updated_time;
    }

    pub fn tpsl_mode(&self) -> &Option<String> {
        &self.tpsl_mode
    }

    pub fn set_tpsl_mode(&mut self, tpsl_mode: String) {
        self.tpsl_mode = Some(tpsl_mode);
    }

    pub fn liq_price(&self) -> Option<f64> {
        self.liq_price
    }

    pub fn set_liq_price(&mut self, liq_price: f64) {
        self.liq_price = Some(liq_price);
    }

    pub fn bust_price(&self) -> Option<f64> {
        self.bust_price
    }

    pub fn set_bust_price(&mut self, bust_price: f64) {
        self.bust_price = Some(bust_price);
    }

    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn set_category(&mut self, category: String) {
        self.category = category;
    }

    pub fn position_status(&self) -> &str {
        &self.position_status
    }

    pub fn set_position_status(&mut self, position_status: String) {
        self.position_status = position_status;
    }

    pub fn adl_rank_indicator(&self) -> u8 {
        self.adl_rank_indicator
    }

    pub fn set_adl_rank_indicator(&mut self, adl_rank_indicator: u8) {
        self.adl_rank_indicator = adl_rank_indicator;
    }

    pub fn auto_add_margin(&self) -> u8 {
        self.auto_add_margin
    }

    pub fn set_auto_add_margin(&mut self, auto_add_margin: u8) {
        self.auto_add_margin = auto_add_margin;
    }

    pub fn leverage_sys_updated_time(&self) -> &Option<String> {
        &self.leverage_sys_updated_time
    }

    pub fn set_leverage_sys_updated_time(&mut self, leverage_sys_updated_time: String) {
        self.leverage_sys_updated_time = Some(leverage_sys_updated_time);
    }

    pub fn mmr_sys_updated_time(&self) -> &Option<String> {
        &self.mmr_sys_updated_time
    }

    pub fn set_mmr_sys_updated_time(&mut self, mmr_sys_updated_time: String) {
        self.mmr_sys_updated_time = Some(mmr_sys_updated_time);
    }

    pub fn seq(&self) -> u64 {
        self.seq
    }

    pub fn set_seq(&mut self, seq: u64) {
        self.seq = seq;
    }

    pub fn is_reduce_only(&self) -> bool {
        self.is_reduce_only
    }

    pub fn set_is_reduce_only(&mut self, is_reduce_only: bool) {
        self.is_reduce_only = is_reduce_only;
    }
}