//! Error types for the TCGdex SDK

use thiserror::Error;

/// A result type for TCGdex operations
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur when using the TCGdex SDK
#[derive(Error, Debug)]
pub enum Error {
    /// An error occurred with the HTTP client
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// An error occurred while parsing a URL
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    /// An error occurred with serialization or deserialization
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// The API returned no data
    #[error("No data returned from API")]
    NoData,

    /// A generic error occurred
    #[error("{0}")]
    Other(String),
}
