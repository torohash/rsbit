use std::fmt::{
    Display,
    Formatter,
    Result
};
use std::error::Error;

#[derive(Debug)]
pub enum ApiKeyError {
    NotFoundApiKey,
    NotFoundApiSecret,
}

impl Display for ApiKeyError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            ApiKeyError::NotFoundApiKey => write!(f, "API key not found. Required for authentication."),
            ApiKeyError::NotFoundApiSecret => write!(f, "API secret not found. Required for authentication."),
        }
    }
}

impl Error for ApiKeyError {}