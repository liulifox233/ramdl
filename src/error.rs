//! the error handling functions.

use thiserror::Error;
use tokio::task::JoinError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    /// An error occurred while fetching.
    #[error("An error occurred while fetching: {0}")]
    Reqwest(#[from] reqwest::Error),
    /// An error occurred while parsing JSON.
    #[error("An error occurred while parsing JSON: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("An error occurred while processing regex: {0}")]
    FancyRegex(#[from] fancy_regex::Error),

    #[error("An error occurred while decrypting: {0}")]
    Decrypt(String),

    #[error("An error occurred while decode base64: {0}")]
    Base64(#[from] base64::DecodeError),

    #[error("An error occurred while initializing ramdl: {0}")]
    Init(String),

    #[error("An error occurred while joining threads: {0}")]
    JoinError(#[from] JoinError),

    #[error("An unknown error: {0}")]
    Other(String),
}
