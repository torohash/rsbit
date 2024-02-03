use crate::{
    v5::ws::BybitWS,
    constants::PRIVATE_WALLET_TOPIC,
    utils::{
        deserialize_f64,
        deserialize_option_f64,
    },
};
use serde::Deserialize;


impl BybitWS {
    pub fn add_wallet_args(&mut self) {
        self.args.push(PRIVATE_WALLET_TOPIC.to_string());
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivateWalletResponse {
    id: String,
    topic: String,
    creation_time: u64,
    data: Vec<PrivateWalletData>,
}

impl PrivateWalletResponse {
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

    pub fn data(&self) -> &Vec<PrivateWalletData> {
        &self.data
    }

    pub fn set_data(&mut self, data: Vec<PrivateWalletData>) {
        self.data = data;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivateWalletData {
    #[serde(deserialize_with = "deserialize_f64")]
    account_i_m_rate: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    account_m_m_rate: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    total_equity: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    total_wallet_balance: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    total_margin_balance: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    total_available_balance: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    total_perp_u_p_l: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    total_initial_margin: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    total_maintenance_margin: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    account_l_t_v: f64,
    account_type: String,
    coin: Vec<Coin>,
}

impl PrivateWalletData {
    pub fn account_i_m_rate(&self) -> f64 {
        self.account_i_m_rate
    }

    pub fn set_account_i_m_rate(&mut self, account_i_m_rate: f64) {
        self.account_i_m_rate = account_i_m_rate;
    }

    pub fn account_m_m_rate(&self) -> f64 {
        self.account_m_m_rate
    }

    pub fn set_account_m_m_rate(&mut self, account_m_m_rate: f64) {
        self.account_m_m_rate = account_m_m_rate;
    }

    pub fn total_equity(&self) -> f64 {
        self.total_equity
    }

    pub fn set_total_equity(&mut self, total_equity: f64) {
        self.total_equity = total_equity;
    }

    pub fn total_wallet_balance(&self) -> f64 {
        self.total_wallet_balance
    }

    pub fn set_total_wallet_balance(&mut self, total_wallet_balance: f64) {
        self.total_wallet_balance = total_wallet_balance;
    }

    pub fn total_margin_balance(&self) -> f64 {
        self.total_margin_balance
    }

    pub fn set_total_margin_balance(&mut self, total_margin_balance: f64) {
        self.total_margin_balance = total_margin_balance;
    }

    pub fn total_available_balance(&self) -> f64 {
        self.total_available_balance
    }

    pub fn set_total_available_balance(&mut self, total_available_balance: f64) {
        self.total_available_balance = total_available_balance;
    }

    pub fn total_perp_u_p_l(&self) -> f64 {
        self.total_perp_u_p_l
    }

    pub fn set_total_perp_u_p_l(&mut self, total_perp_u_p_l: f64) {
        self.total_perp_u_p_l = total_perp_u_p_l;
    }

    pub fn total_initial_margin(&self) -> f64 {
        self.total_initial_margin
    }

    pub fn set_total_initial_margin(&mut self, total_initial_margin: f64) {
        self.total_initial_margin = total_initial_margin;
    }

    pub fn total_maintenance_margin(&self) -> f64 {
        self.total_maintenance_margin
    }

    pub fn set_total_maintenance_margin(&mut self, total_maintenance_margin: f64) {
        self.total_maintenance_margin = total_maintenance_margin;
    }

    pub fn account_l_t_v(&self) -> f64 {
        self.account_l_t_v
    }

    pub fn set_account_l_t_v(&mut self, account_l_t_v: f64) {
        self.account_l_t_v = account_l_t_v;
    }

    pub fn account_type(&self) -> &str {
        &self.account_type
    }

    pub fn set_account_type(&mut self, account_type: String) {
        self.account_type = account_type;
    }

    pub fn coin(&self) -> &Vec<Coin> {
        &self.coin
    }

    pub fn set_coin(&mut self, coin: Vec<Coin>) {
        self.coin = coin;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coin {
    coin: String,
    #[serde(deserialize_with = "deserialize_f64")]
    equity: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    usd_value: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    wallet_balance: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    available_to_withdraw: f64,
    #[serde(deserialize_with = "deserialize_option_f64")]
    available_to_borrow: Option<f64>,
    #[serde(deserialize_with = "deserialize_f64")]
    borrow_amount: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    accrued_interest: f64,
    total_order_i_m: String,
    total_position_i_m: String,
    total_position_m_m: String,
    #[serde(deserialize_with = "deserialize_f64")]
    unrealised_pnl: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    cum_realised_pnl: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    bonus: f64,
    collateral_switch: bool,
    margin_collateral: bool,
    #[serde(deserialize_with = "deserialize_f64")]
    locked: f64,
}

impl Coin {
    pub fn coin(&self) -> &str {
        &self.coin
    }

    pub fn set_coin(&mut self, coin: String) {
        self.coin = coin;
    }

    pub fn equity(&self) -> f64 {
        self.equity
    }

    pub fn set_equity(&mut self, equity: f64) {
        self.equity = equity;
    }

    pub fn usd_value(&self) -> f64 {
        self.usd_value
    }

    pub fn set_usd_value(&mut self, usd_value: f64) {
        self.usd_value = usd_value;
    }

    pub fn wallet_balance(&self) -> f64 {
        self.wallet_balance
    }

    pub fn set_wallet_balance(&mut self, wallet_balance: f64) {
        self.wallet_balance = wallet_balance;
    }

    pub fn available_to_withdraw(&self) -> f64 {
        self.available_to_withdraw
    }

    pub fn set_available_to_withdraw(&mut self, available_to_withdraw: f64) {
        self.available_to_withdraw = available_to_withdraw;
    }

    pub fn available_to_borrow(&self) -> Option<f64> {
        self.available_to_borrow
    }

    pub fn set_available_to_borrow(&mut self, available_to_borrow: f64) {
        self.available_to_borrow = Some(available_to_borrow);
    }

    pub fn borrow_amount(&self) -> f64 {
        self.borrow_amount
    }

    pub fn set_borrow_amount(&mut self, borrow_amount: f64) {
        self.borrow_amount = borrow_amount;
    }

    pub fn accrued_interest(&self) -> f64 {
        self.accrued_interest
    }

    pub fn set_accrued_interest(&mut self, accrued_interest: f64) {
        self.accrued_interest = accrued_interest;
    }

    pub fn total_order_i_m(&self) -> &str {
        &self.total_order_i_m
    }

    pub fn set_total_order_i_m(&mut self, total_order_i_m: String) {
        self.total_order_i_m = total_order_i_m;
    }

    pub fn total_position_i_m(&self) -> &str {
        &self.total_position_i_m
    }

    pub fn set_total_position_i_m(&mut self, total_position_i_m: String) {
        self.total_position_i_m = total_position_i_m;
    }

    pub fn total_position_m_m(&self) -> &str {
        &self.total_position_m_m
    }

    pub fn set_total_position_m_m(&mut self, total_position_m_m: String) {
        self.total_position_m_m = total_position_m_m;
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

    pub fn bonus(&self) -> f64 {
        self.bonus
    }

    pub fn set_bonus(&mut self, bonus: f64) {
        self.bonus = bonus;
    }

    pub fn collateral_switch(&self) -> bool {
        self.collateral_switch
    }

    pub fn set_collateral_switch(&mut self, collateral_switch: bool) {
        self.collateral_switch = collateral_switch;
    }

    pub fn margin_collateral(&self) -> bool {
        self.margin_collateral
    }

    pub fn set_margin_collateral(&mut self, margin_collateral: bool) {
        self.margin_collateral = margin_collateral;
    }

    pub fn locked(&self) -> f64 {
        self.locked
    }

    pub fn set_locked(&mut self, locked: f64) {
        self.locked = locked;
    }

}