//! SetResume model implementation

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::endpoints::{Fetchable, Listable};
use crate::error::Result;
use crate::models::common::SetCardCountResume;
use crate::models::Extension;
use crate::utils;

/// Set resume, contains basic information about a specific set
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetResume {
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
}

impl SetResume {
    /// Get the full logo URL with the specified extension
    pub fn get_logo_url(&self, extension: Extension) -> Option<String> {
        self.logo
            .as_deref()
            .map(|base| utils::build_logo_url(base, extension))
    }

    /// Download the set logo with the specified extension
    pub async fn get_logo(
        &self,
        tcgdex: &crate::TCGdex,
        extension: Extension,
    ) -> Result<Option<bytes::Bytes>> {
        match self.get_logo_url(extension) {
            Some(url) => {
                let bytes = utils::download_image(tcgdex.client(), &url).await?;
                Ok(Some(bytes))
            }
            None => Ok(None),
        }
    }

    /// Get the full symbol URL with the specified extension
    pub fn get_symbol_url(&self, extension: Extension) -> Option<String> {
        self.symbol
            .as_deref()
            .map(|base| utils::build_logo_url(base, extension))
    }

    /// Download the set symbol with the specified extension
    pub async fn get_symbol(
        &self,
        tcgdex: &crate::TCGdex,
        extension: Extension,
    ) -> Result<Option<bytes::Bytes>> {
        match self.get_symbol_url(extension) {
            Some(url) => {
                let bytes = utils::download_image(tcgdex.client(), &url).await?;
                Ok(Some(bytes))
            }
            None => Ok(None),
        }
    }

    /// Get the full set information
    pub async fn get_full_set(&self, tcgdex: &crate::TCGdex) -> Result<crate::models::Set> {
        tcgdex.set.get(&self.id).await
    }
}

#[async_trait]
impl Fetchable for SetResume {
    // Using the default implementation
}

#[async_trait]
impl Listable for SetResume {
    // Using the default implementation
}
