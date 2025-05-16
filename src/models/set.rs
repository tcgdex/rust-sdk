//! Set model implementation

use serde::{Deserialize, Serialize};
use async_trait::async_trait;

use crate::endpoints::{Fetchable, Listable};
use crate::error::Result;
use crate::models::common::*;
use crate::models::{CardResume, Extension, SerieResume};
use crate::utils;

/// Poku00e9mon TCG Set, contains all information about a specific set
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Set {
    /// Globally unique set ID
    pub id: String,
    /// The set name
    pub name: String,
    /// The set logo URL (without extension)
    pub logo: Option<String>,
    /// The set symbol URL (without extension)
    pub symbol: Option<String>,
    /// The number of cards in the set
    #[serde(rename = "cardCount")]
    pub card_count: SetCardCountResume,
    /// The serie this set is a part of
    pub serie: SerieResume,
    /// The Poku00e9mon TCG Online code
    #[serde(rename = "tcgOnline")]
    pub tcg_online: Option<String>,
    /// The set release date as yyyy-mm-dd
    #[serde(rename = "releaseDate")]
    pub release_date: String,
    /// The set legality information
    pub legal: Legal,
    /// The cards contained in this set
    pub cards: Vec<CardResume>,
}

impl Set {
    /// Get the full logo URL with the specified extension
    pub fn get_logo_url(&self, extension: Extension) -> Option<String> {
        self.logo.as_deref().map(|base| utils::build_logo_url(base, extension))
    }
    
    /// Download the set logo with the specified extension
    pub async fn get_logo(
        &self,
        client: &reqwest::Client,
        extension: Extension,
    ) -> Result<Option<bytes::Bytes>> {
        match self.get_logo_url(extension) {
            Some(url) => {
                let bytes = utils::download_image(client, &url).await?;
                Ok(Some(bytes))
            }
            None => Ok(None),
        }
    }
    
    /// Get the full symbol URL with the specified extension
    pub fn get_symbol_url(&self, extension: Extension) -> Option<String> {
        self.symbol.as_deref().map(|base| utils::build_logo_url(base, extension))
    }
    
    /// Download the set symbol with the specified extension
    pub async fn get_symbol(
        &self,
        client: &reqwest::Client,
        extension: Extension,
    ) -> Result<Option<bytes::Bytes>> {
        match self.get_symbol_url(extension) {
            Some(url) => {
                let bytes = utils::download_image(client, &url).await?;
                Ok(Some(bytes))
            }
            None => Ok(None),
        }
    }
}

#[async_trait]
impl Fetchable for Set {
    // Using the default implementation
}

#[async_trait]
impl Listable for Set {
    // Using the default implementation
}