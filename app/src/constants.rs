pub const TESTNET_API_URL: &'static str = "https://api-testnet.bybit.com";
pub const MAINNET_API_URL: &'static str = "https://api.bybit.com";
pub const DEFAULT_RECV_WINDOW: &'static str = "5000";

pub const MAINNET_SPOT_PUBLIC_CHANNEL: &'static str = "wss://stream.bybit.com/v5/public/spot";
pub const MAINNET_LINEAR_PUBLIC_CHANNEL: &'static str = "wss://stream.bybit.com/v5/public/linear";
pub const MAINNET_INVERSE_PUBLIC_CHANNEL: &'static str = "wss://stream.bybit.com/v5/public/inverse";
pub const MAINNET_OPTION_PUBLIC_CHANNEL: &'static str = "wss://stream.bybit.com/v5/public/option";

pub const TESTNET_SPOT_PUBLIC_CHANNEL: &'static str = "wss://stream-testnet.bybit.com/v5/public/spot";
pub const TESTNET_LINEAR_PUBLIC_CHANNEL: &'static str = "wss://stream-testnet.bybit.com/v5/public/linear";
pub const TESTNET_INVERSE_PUBLIC_CHANNEL: &'static str = "wss://stream-testnet.bybit.com/v5/public/inverse";
pub const TESTNET_OPTION_PUBLIC_CHANNEL: &'static str = "wss://stream-testnet.bybit.com/v5/public/option";

pub const MAINNET_PRIVATE_CHANNEL: &'static str = "wss://stream.bybit.com/private";
pub const TESTNET_PRIVATE_CHANNEL: &'static str = "wss://stream-testnet.bybit.com/private";

pub const CATEGORY_LINEAR: &'static str = "linear";
pub const CATEGORY_SPOT: &'static str = "spot";
pub const CATEGORY_INVERSE: &'static str = "inverse";
pub const CATEGORY_OPTION: &'static str = "option";

pub const PUBLIC_TRADE_TOPIC: &'static str = "publicTrade";
pub const PUBLIC_ORDERBOOK_TOPIC: &'static str = "orderbook";
pub const PUBLIC_TICKERS_TOPIC: &'static str = "tickers";
pub const PUBLIC_KLINE_TOPIC: &'static str = "kline";
pub const PUBLIC_LIQUIDATION_TOPIC: &'static str = "liquidation";