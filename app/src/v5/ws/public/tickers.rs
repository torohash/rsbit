pub mod linear;
pub mod inverse;
pub mod option;
pub mod spot;

use crate::{
    v5::ws::BybitWS,
    constants::PUBLIC_TICKERS_TOPIC,
};

impl BybitWS {
    pub fn add_tickers_args(&mut self, symbol: &str) {
        self.args.push(format!("{}.{}", PUBLIC_TICKERS_TOPIC, symbol));
    }
}