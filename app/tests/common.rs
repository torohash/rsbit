use rsbit::v5::{
    api::BybitApi,
    ws::BybitWS,
    ws::Channel
};
use dotenv::dotenv;
use std::env;

pub fn setup_api_private() -> BybitApi {
    dotenv().ok();
    let api_key = env::var("TESTNET_API_KEY").expect("TESTNET_API_KEY must be set");
    let api_secret = env::var("TESTNET_API_SECRET").expect("TESTNET_API_SECRET must be set");

    BybitApi::new()
        .with_api_key(api_key)
        .with_api_secret(api_secret)
}

pub fn setup_api_public() -> BybitApi {
    BybitApi::new()
}

pub fn setup_ws(channel: Channel) -> BybitWS {
    dotenv().ok();
    if channel.is_private(){
        let api_key = env::var("TESTNET_API_KEY").expect("TESTNET_API_KEY must be set");
        let api_secret = env::var("TESTNET_API_SECRET").expect("TESTNET_API_SECRET must be set");
    
        BybitWS::new(channel)
            .with_api_key(api_key)
            .with_api_secret(api_secret)
    } else {
        BybitWS::new(channel)
    }
}