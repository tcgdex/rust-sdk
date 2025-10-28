//! Serie model implementation

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::endpoints::{Fetchable, Listable};
use crate::error::Result;
use crate::models::{Extension, SetResume};
use crate::utils;

/// Pokémon TCG Serie, contains all information about a specific serie
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Serie {
    /// The sets the serie contains
    pub sets: Vec<SetResume>,
    /// The serie's unique ID
    pub id: String,
    /// The serie name
    pub name: String,
    /// The serie logo URL (without extension)
    pub logo: Option<String>,
}

impl Serie {
    /// Get the full logo URL with the specified extension
    pub fn get_logo_url(&self, extension: Extension) -> Option<String> {
        self.logo
            .as_deref()
            .map(|base| utils::build_logo_url(base, extension))
    }

    /// Download the serie logo with the specified extension
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
}

#[async_trait]
impl Fetchable for Serie {
    // Using the default implementation
}

#[async_trait]
impl Listable for Serie {
    // Using the default implementation
}
