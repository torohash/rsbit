use crate::v5::api::{
    BybitApi,
    post::Post,
};

use serde::{
    Serialize,
    Deserialize,
};
use anyhow::Result;

const PATH: &'static str = "/v5/order/disconnected-cancel-all";

impl BybitApi {
    /// set DCP(Disconnect Cancel All).
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for set DCP.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rsbit::v5::api::{
    ///     post::trade::set_dcp::{
    ///         SetDCPParameters,
    ///     },
    ///     BybitApi,
    /// };
    /// #[tokio::main]
    /// async fn main() {
    ///     let api = BybitApi::new();
    ///     let params = SetDCPParameters::new(
    ///         10,
    ///     );
    ///     let response = api.set_dcp(params).await;
    ///     match response {
    ///         Ok(info) => {
    ///             // Handle the data
    ///         },
    ///         Err(err) => {
    ///             // Handle the error
    ///         },
    ///     }
    /// }
    /// ```
    pub async fn set_dcp(&self, params: SetDCPParameters) -> Result<SetDCPResponse> {
        self.post(PATH, Some(params)).await
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDCPParameters {
    time_window: u32,
}

impl SetDCPParameters {
    /// Creates a new instance of `SetDCPParameters`.
    ///
    /// # Arguments
    ///
    /// * `time_window` - The time window of set DCP.
    ///
    /// # Returns
    ///
    /// A new instance of `SetDCPParameters`.
    pub fn new(time_window: u32) -> Self {
        Self {
            time_window
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDCPResponse {
    ret_code: i32,
    ret_msg: String,
}
impl SetDCPResponse {
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

}
