## Details
This is a library for the Bybit API.

## Usage

The response is returned in the form of a deserialized struct.

```rust
use rsbit::v5::api::{
    BybitApi,
    get::market::get_public_recent_trading_history::{
        GetPublicRecentTradingHistoryParameters,
        GetPublicRecentTradingHistoryCategory,
    }
};

#[tokio::main]
async fn main() {
    // If you're only using public APIs, there is no need to set it up with an API key or API secret.
    let bybit_api = BybitApi::new()
        .with_mainnet() // The default is for the testnet, so please add it for use on the mainnet.
        .with_testnet() // You can also revert to the testnet if needed.
        .with_api_key("your_api_key")
        .with_api_secret("your_api_secret");
    
    let params = GetPublicRecentTradingHistoryParameters::new(
        GetPublicRecentTradingHistoryCategory::Linear,
    ).with_symbol("BTCUSDT".to_string());

    let result = bybit_api.get_public_recent_trading_history.unwrap();

    match result {
        Ok(result) => {
            // Do something with the result.
        },
        Err(err) => {
            // Do something with the error.
        }
    }
}
```

### Reference
- [Introduction | Bybit API Documentation](https://bybit-exchange.github.io/docs/v5/intro)