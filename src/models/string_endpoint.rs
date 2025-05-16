//! StringEndpoint model implementation

use serde::{Deserialize, Serialize};
use async_trait::async_trait;

use crate::endpoints::Fetchable;
use crate::models::CardResume;

/// Generic class for string-based endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringEndpoint {
    /// The endpoint value
    pub name: String,
    /// The cards that contain this value
    pub cards: Vec<CardResume>,
}

#[async_trait]
impl Fetchable for StringEndpoint {
    // Using the default implementation
}

// StringList is used instead of String for list operations