//! A Rust SDK for the TCGdex API
//!
//! This SDK provides a convenient way to interact with the TCGdex API,
//! which is a database for Poku00e9mon Trading Card Game cards, sets, and series.

mod client;
mod endpoints;
mod error;
mod models;
mod query;
mod utils;

pub use client::TCGdex;
pub use endpoints::Endpoint;
pub use error::{Error, Result};
pub use models::*;
pub use query::Query;

/// The current version of the SDK
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use crate::{Language, Query, TCGdex};

    // Non-async test that doesn't need async runtime
    #[test]
    fn test_default_client() {
        // Test creating a default client
        let client = TCGdex::default();
        assert_eq!(client.language, Language::EN);
        assert_eq!(client.get_endpoint(), TCGdex::DEFAULT_ENDPOINT);
    }

    // Non-async test that doesn't need async runtime
    #[test]
    fn test_custom_client() {
        // Test creating a client with custom language
        let client = TCGdex::new(Language::FR);
        assert_eq!(client.language, Language::FR);
        assert_eq!(client.get_endpoint(), TCGdex::DEFAULT_ENDPOINT);

        // Test creating a client with custom endpoint
        let client = TCGdex::with_endpoint("https://example.com/api", Language::DE);
        assert_eq!(client.language, Language::DE);
        assert_eq!(client.get_endpoint(), "https://example.com/api");
    }

    // This test doesn't actually need to be async either
    #[test]
    fn test_query_builder() {
        // Test query builder methods
        let mut query = Query::new();
        assert_eq!(query.build(), "");

        query.equal("name", "Pikachu");
        assert_eq!(query.build(), "?name=eq%3APikachu");

        query.not_equal("type", "Fire");
        assert_eq!(query.build(), "?name=eq%3APikachu&type=neq%3AFire");

        query.greater_than("hp", 100);
        assert_eq!(
            query.build(),
            "?name=eq%3APikachu&type=neq%3AFire&hp=gt%3A100"
        );

        query.sort("name", "asc");
        assert_eq!(
            query.build(),
            "?name=eq%3APikachu&type=neq%3AFire&hp=gt%3A100&sort%3Afield=name&sort%3Aorder=asc"
        );
    }

    // We would add actual API call tests here if we had mockable test data
    // For now we'll just test the construction of the SDK and Query builder
}
