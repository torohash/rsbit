use crate::{
    v5::api::{
        BybitApi,
        get::Get,
    },
    utils::{
        deserialize_f64,
        deserialize_option_f64,
    },
};

use serde::{
    Serialize,
    Deserialize,
};
use serde_json::Value;
use anyhow::Result;

const PATH: &'static str = "/v5/account/wallet-balance";

impl BybitApi {
    pub async fn get_wallet_balance(&self, params: GetWalletBalanceParameters) -> Result<GetWalletBalanceResponse> {
        self.get(PATH, Some(params), true).await
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum GetWalletBalanceAccountType {
    UNIFIED,
    CONTRACT,
    SPOT,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletBalanceParameters {
    account_type: GetWalletBalanceAccountType,
    coin: Option<String>,
}

impl GetWalletBalanceParameters {
    pub fn new(account_type: GetWalletBalanceAccountType) -> Self {
        Self {
            account_type,
            coin: None
        }
    }

    pub fn with_coin(mut self, coin: String) -> Self {
        self.coin = Some(coin);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletBalanceResponse {
    ret_code: i32,
    ret_msg: String,
    result: WalletBalanceResult,
    ret_ext_info: Value,
    time: u64,
}
impl GetWalletBalanceResponse {
    pub fn ret_code(&self) -> i32 {
        self.ret_code
    }

    pub fn set_ret_code(&mut self, ret_code: i32) {
        self.ret_code = ret_code;
    }

    pub fn ret_msg(&self) -> &str {
        &self.ret_msg
    }

    pub fn set_ret_msg(&mut self, ret_msg: String) {
        self.ret_msg = ret_msg;
    }

    pub fn result(&self) -> &WalletBalanceResult {
        &self.result
    }

    pub fn set_result(&mut self, result: WalletBalanceResult) {
        self.result = result;
    }

    pub fn ret_ext_info(&self) -> &Value {
        &self.ret_ext_info
    }

    pub fn set_ret_ext_info(&mut self, ret_ext_info: Value) {
        self.ret_ext_info = ret_ext_info;
    }

    pub fn time(&self) -> u64 {
        self.time
    }

    pub fn set_time(&mut self, time: u64) {
        self.time = time;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalanceResult {
    list: Vec<WalletBalance>
}
impl WalletBalanceResult {
    pub fn list(&self) -> &Vec<WalletBalance> {
        &self.list
    }

    pub fn set_list(&mut self, list: Vec<WalletBalance>) {
        self.list = list;
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalance {
    #[serde(deserialize_with = "deserialize_option_f64")]
    pub total_equity: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    account_i_m_rate: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    total_margin_balance: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    total_initial_margin: Option<f64>,
    account_type: String,
    #[serde(deserialize_with = "deserialize_option_f64")]
    total_available_balance: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    account_m_m_rate: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    total_perp_u_p_l: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    total_wallet_balance: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    account_l_t_v: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_f64")]
    total_maintenance_margin: Option<f64>,
    coin: Vec<Coin>,
}

impl WalletBalance {
    pub fn total_equity(&self) -> Option<f64> {
        self.total_equity
    }

    pub fn set_total_equity(&mut self, total_equity: f64) {
        self.total_equity = Some(total_equity);
    }
    
    pub fn account_i_m_rate(&self) -> Option<f64> {
        self.account_i_m_rate
    }

    pub fn set_account_i_m_rate(&mut self, account_i_m_rate: f64) {
        self.account_i_m_rate = Some(account_i_m_rate);
    }

    pub fn total_margin_balance(&self) -> Option<f64> {
        self.total_margin_balance
    }

    pub fn set_total_margin_balance(&mut self, total_margin_balance: f64) {
        self.total_margin_balance = Some(total_margin_balance);
    }

    pub fn total_initial_margin(&self) -> Option<f64> {
        self.total_initial_margin
    }

    pub fn set_total_initial_margin(&mut self, total_initial_margin: f64) {
        self.total_initial_margin = Some(total_initial_margin);
    }

    pub fn account_type(&self) -> &str {
        &self.account_type
    }

    pub fn set_account_type(&mut self, account_type: String) {
        self.account_type = account_type;
    }

    pub fn total_available_balance(&self) -> Option<f64> {
        self.total_available_balance
    }

    pub fn set_total_available_balance(&mut self, total_available_balance: f64) {
        self.total_available_balance = Some(total_available_balance);
    }

    pub fn account_m_m_rate(&self) -> Option<f64> {
        self.account_m_m_rate
    }

    pub fn set_account_m_m_rate(&mut self, account_m_m_rate: f64) {
        self.account_m_m_rate = Some(account_m_m_rate);
    }

    pub fn total_perp_u_p_l(&self) -> Option<f64> {
        self.total_perp_u_p_l
    }

    pub fn set_total_perp_u_p_l(&mut self, total_perp_u_p_l: f64) {
        self.total_perp_u_p_l = Some(total_perp_u_p_l);
    }

    pub fn total_wallet_balance(&self) -> Option<f64> {
        self.total_wallet_balance
    }

    pub fn set_total_wallet_balance(&mut self, total_wallet_balance: f64) {
        self.total_wallet_balance = Some(total_wallet_balance);
    }

    pub fn account_l_t_v(&self) -> Option<f64> {
        self.account_l_t_v
    }

    pub fn set_account_l_t_v(&mut self, account_l_t_v: f64) {
        self.account_l_t_v = Some(account_l_t_v);
    }

    pub fn total_maintenance_margin(&self) -> Option<f64> {
        self.total_maintenance_margin
    }

    pub fn set_total_maintenance_margin(&mut self, total_maintenance_margin: f64) {
        self.total_maintenance_margin = Some(total_maintenance_margin);
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
    #[serde(deserialize_with = "deserialize_option_f64")]
    pub available_to_borrow: Option<f64>,
    #[serde(deserialize_with = "deserialize_f64")]
    pub bonus: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub accrued_interest: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub available_to_withdraw: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub total_order_i_m: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub equity: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub total_position_m_m: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub usd_value: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub spot_hedging_qty: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub unrealised_pnl: f64,
    pub collateral_switch: bool,
    #[serde(deserialize_with = "deserialize_f64")]
    pub borrow_amount: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub total_position_i_m: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub wallet_balance: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub cum_realised_pnl: f64,
    #[serde(deserialize_with = "deserialize_f64")]
    pub locked: f64,
    pub margin_collateral: bool,
    pub coin: String,
}

impl Coin {
    pub fn available_to_borrow(&self) -> Option<f64> {
        self.available_to_borrow
    }

    pub fn set_available_to_borrow(&mut self, available_to_borrow: f64) {
        self.available_to_borrow = Some(available_to_borrow);
    }

    pub fn bonus(&self) -> f64 {
        self.bonus
    }

    pub fn set_bonus(&mut self, bonus: f64) {
        self.bonus = bonus;
    }

    pub fn accrued_interest(&self) -> f64 {
        self.accrued_interest
    }

    pub fn set_accrued_interest(&mut self, accrued_interest: f64) {
        self.accrued_interest = accrued_interest;
    }

    pub fn available_to_withdraw(&self) -> f64 {
        self.available_to_withdraw
    }

    pub fn set_available_to_withdraw(&mut self, available_to_withdraw: f64) {
        self.available_to_withdraw = available_to_withdraw;
    }

    pub fn total_order_i_m(&self) -> f64 {
        self.total_order_i_m
    }

    pub fn set_total_order_i_m(&mut self, total_order_i_m: f64) {
        self.total_order_i_m = total_order_i_m;
    }

    pub fn equity(&self) -> f64 {
        self.equity
    }

    pub fn set_equity(&mut self, equity: f64) {
        self.equity = equity;
    }

    pub fn total_position_m_m(&self) -> f64 {
        self.total_position_m_m
    }

    pub fn set_total_position_m_m(&mut self, total_position_m_m: f64) {
        self.total_position_m_m = total_position_m_m;
    }

    pub fn usd_value(&self) -> f64 {
        self.usd_value
    }

    pub fn set_usd_value(&mut self, usd_value: f64) {
        self.usd_value = usd_value;
    }

    pub fn spot_hedging_qty(&self) -> f64 {
        self.spot_hedging_qty
    }

    pub fn set_spot_hedging_qty(&mut self, spot_hedging_qty: f64) {
        self.spot_hedging_qty = spot_hedging_qty;
    }

    pub fn unrealised_pnl(&self) -> f64 {
        self.unrealised_pnl
    }

    pub fn set_unrealised_pnl(&mut self, unrealised_pnl: f64) {
        self.unrealised_pnl = unrealised_pnl;
    }

    pub fn collateral_switch(&self) -> bool {
        self.collateral_switch
    }

    pub fn set_collateral_switch(&mut self, collateral_switch: bool) {
        self.collateral_switch = collateral_switch;
    }

    pub fn borrow_amount(&self) -> f64 {
        self.borrow_amount
    }

    pub fn set_borrow_amount(&mut self, borrow_amount: f64) {
        self.borrow_amount = borrow_amount;
    }

    pub fn total_position_i_m(&self) -> f64 {
        self.total_position_i_m
    }

    pub fn set_total_position_i_m(&mut self, total_position_i_m: f64) {
        self.total_position_i_m = total_position_i_m;
    }

    pub fn wallet_balance(&self) -> f64 {
        self.wallet_balance
    }

    pub fn set_wallet_balance(&mut self, wallet_balance: f64) {
        self.wallet_balance = wallet_balance;
    }

    pub fn cum_realised_pnl(&self) -> f64 {
        self.cum_realised_pnl
    }

    pub fn set_cum_realised_pnl(&mut self, cum_realised_pnl: f64) {
        self.cum_realised_pnl = cum_realised_pnl;
    }

    pub fn locked(&self) -> f64 {
        self.locked
    }

    pub fn set_locked(&mut self, locked: f64) {
        self.locked = locked;
    }

    pub fn margin_collateral(&self) -> bool {
        self.margin_collateral
    }

    pub fn set_margin_collateral(&mut self, margin_collateral: bool) {
        self.margin_collateral = margin_collateral;
    }

    pub fn coin(&self) -> &str {
        &self.coin
    }

    pub fn set_coin(&mut self, coin: String) {
        self.coin = coin;
    }
}