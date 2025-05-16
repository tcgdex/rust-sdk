//! SerieResume model implementation

use serde::{Deserialize, Serialize};
use async_trait::async_trait;

use crate::endpoints::{Fetchable, Listable};
use crate::error::Result;
use crate::models::Extension;
use crate::utils;

/// Poku00e9mon TCG Serie Resume, contains basic information about a specific serie
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerieResume {
    /// The serie's unique ID
    pub id: String,
    /// The serie name
    pub name: String,
    /// The serie logo URL (without extension)
    pub logo: Option<String>,
}

impl SerieResume {
    /// Get the full logo URL with the specified extension
    pub fn get_logo_url(&self, extension: Extension) -> Option<String> {
        self.logo.as_deref().map(|base| utils::build_logo_url(base, extension))
    }
    
    /// Download the serie logo with the specified extension
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
    
    /// Get the full serie information
    pub async fn get_full_serie(&self, tcgdex: &crate::TCGdex) -> Result<crate::models::Serie> {
        tcgdex.serie.get(tcgdex.client(), &self.id).await
    }
}

#[async_trait]
impl Fetchable for SerieResume {
    // Using the default implementation
}

#[async_trait]
impl Listable for SerieResume {
    // Using the default implementation
}