//! IntEndpoint model implementation

use serde::{Deserialize, Serialize};
use async_trait::async_trait;

use crate::endpoints::Fetchable;
use crate::models::CardResume;

/// Generic class for integer-based endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntEndpoint {
    /// The endpoint value
    pub name: u32,
    /// The cards that contain this value
    pub cards: Vec<CardResume>,
}

#[async_trait]
impl Fetchable for IntEndpoint {
    // Using the default implementation
}

// IntList is used instead of u32 for list operations