//! CardResume model implementation

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::endpoints::{Fetchable, Listable};
use crate::error::Result;
use crate::models::{Extension, Quality};
use crate::utils;

/// Card resume class, contains basic information about a specific card
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardResume {
    /// Globally unique card ID
    pub id: String,
    /// ID indexing this card within its set
    #[serde(rename = "localId")]
    pub local_id: String,
    /// Card name
    pub name: String,
    /// Card image URL (without extension and quality)
    pub image: Option<String>,
}

impl CardResume {
    /// Get the full image URL with the specified quality and extension
    pub fn get_image_url(&self, quality: Quality, extension: Extension) -> Option<String> {
        self.image
            .as_deref()
            .map(|base| utils::build_image_url(base, quality, extension))
    }

    /// Download the card image with the specified quality and extension
    pub async fn get_image(
        &self,
        tcgdex: &crate::TCGdex,
        quality: Quality,
        extension: Extension,
    ) -> Result<Option<bytes::Bytes>> {
        match self.get_image_url(quality, extension) {
            Some(url) => {
                let bytes = utils::download_image(tcgdex.client(), &url).await?;
                Ok(Some(bytes))
            }
            None => Ok(None),
        }
    }

    /// Get the full card information
    pub async fn get_full_card(&self, tcgdex: &crate::TCGdex) -> Result<crate::models::Card> {
        tcgdex.card.get(&self.id).await
    }
}

#[async_trait]
impl Fetchable for CardResume {
    // Using the default implementation
}

#[async_trait]
impl Listable for CardResume {
    // Using the default implementation
}
