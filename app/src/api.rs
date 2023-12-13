pub mod v5;
mod request;

use crate::constants::{
    TESTNET_API_URL,
    MAINNET_API_URL,
    DEFAULT_RECV_WINDOW,
};

#[derive(Debug, Clone)]
pub struct BybitApi {
    base_url: &'static str,
    api_key: Option<String>,
    api_secret: Option<String>,
    recv_window: String,
}

impl BybitApi {
    /// Gets the base URL.
    ///
    /// # Returns
    ///
    /// The base URL.
    pub fn base_url(&self) -> &'static str {
        self.base_url
    }

    /// Gets the API key.
    ///
    /// # Returns
    ///
    /// The API key.
    pub fn api_key(&self) -> Option<&String> {
        self.api_key.as_ref()
    }

    /// Gets the API secret.
    ///
    /// # Returns
    ///
    /// The API secret.
    pub fn api_secret(&self) -> Option<&String> {
        self.api_secret.as_ref()
    }

    /// Gets the receive window.
    ///
    /// # Returns
    ///
    /// The receive window.
    pub fn recv_window(&self) -> &String {
        &self.recv_window
    }

    /// Create a new instance of the API.
    ///
    /// This method initializes a new instance of the API struct with default values.
    /// It returns the newly created instance.
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Sets the base URL for the API.
    ///
    /// # Arguments
    ///
    /// * `is_testnet` - A boolean indicating whether the API should use the testnet or mainnet URL.
    ///
    /// # Returns
    ///
    /// The modified `Self` object.
    pub fn with_base_url(mut self, is_testnet: bool) -> Self {
        self.base_url = if is_testnet {
            TESTNET_API_URL
        } else {
            MAINNET_API_URL
        };
        self
    }

    /// Sets the API key.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key to be set.
    ///
    /// # Returns
    ///
    /// The modified `Self` object.
    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    /// Sets the API secret.
    ///
    /// # Arguments
    ///
    /// * `api_secret` - The API secret to be set.
    ///
    /// # Returns
    ///
    /// The modified `Self` object.
    pub fn with_api_secret(mut self, api_secret: String) -> Self {
        self.api_secret = Some(api_secret);
        self
    }

    /// Sets the receive window.
    ///
    /// # Arguments
    ///
    /// * `recv_window` - The receive window to be set.
    ///
    /// # Returns
    ///
    /// The modified `Self` object.
    pub fn with_recv_window(mut self, recv_window: String) -> Self {
        self.recv_window = recv_window;
        self
    }
}

impl Default for BybitApi {
    fn default() -> Self {
        Self {
            base_url: TESTNET_API_URL,
            api_key: None,
            api_secret: None,
            recv_window: DEFAULT_RECV_WINDOW.to_string(),
        }
    }
}






