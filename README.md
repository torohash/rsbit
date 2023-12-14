
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
    ).with_symbol("BTCUSDT".to_string()));

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

## develop

Create a `.env` file and write the following:
```
# .env
USER_NAME=your_user_name
```

Then, start the docker container:
```
docker compose build
docker compose up -d
docker compose exec app bash
```

If you want to run test code, create a `.env` file in the root directory of the Rust project and write as follows:
```
# .env
TESTNET_API_KEY=your_api_key
TESTNET_API_SECRET=your_api_secret
```

After that, execute the following command:
It is recommended that the thread specify 1. because the request frequency will be too high.
```
cargo test --all-targets -- --test-threads=1
cargo test --doc
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Serde by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>