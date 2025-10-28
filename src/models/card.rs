//! Card model implementation

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::endpoints::{Fetchable, Listable};
use crate::error::Result;
use crate::models::common::*;
use crate::models::{Extension, Quality, SetResume};
use crate::utils;

/// Pokémon TCG Card, contains all information about a specific card
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    /// Card illustrator
    pub illustrator: Option<String>,
    /// Card rarity
    pub rarity: String,
    /// Card category
    pub category: String,
    /// The card's possible variants
    pub variants: CardVariants,
    /// Resume of the set the card belongs to
    pub set: SetResume,
    /// The Pokémon Pokédex IDs
    #[serde(rename = "dexIDs")]
    pub dex_ids: Option<Vec<i32>>,
    /// HP of the Pokémon - could be integer or string in the API
    #[serde(deserialize_with = "crate::utils::deserialize_string_or_number_to_i32")]
    pub hp: Option<i32>,
    /// Types of the Pokémon
    pub types: Option<Vec<String>>,
    /// Name of the Pokémon this one evolves from
    #[serde(rename = "evolvesFrom")]
    pub evolves_from: Option<String>,
    /// The Pokémon Pokédex description
    pub description: Option<String>,
    /// The Pokémon level
    pub level: Option<String>,
    /// The Pokémon stage
    pub stage: Option<String>,
    /// The Pokémon suffix
    pub suffix: Option<String>,
    /// The item the Pokémon has
    pub item: Option<CardItem>,
    /// The card abilities
    pub abilities: Option<Vec<CardAbility>>,
    /// The card attacks
    pub attacks: Option<Vec<CardAttack>>,
    /// The Pokémon weaknesses
    pub weaknesses: Option<Vec<CardWeakRes>>,
    /// The Pokémon resistances
    pub resistances: Option<Vec<CardWeakRes>>,
    /// The Pokémon retreat cost
    pub retreat: Option<i32>,
    /// The card effect (Trainer/Energy only)
    pub effect: Option<String>,
    /// The trainer sub-type
    #[serde(rename = "trainerType")]
    pub trainer_type: Option<String>,
    /// The energy sub-type
    #[serde(rename = "energyType")]
    pub energy_type: Option<String>,
    /// The card regulation mark
    #[serde(rename = "regulationMark")]
    pub regulation_mark: Option<String>,
    /// The card legality for tournament play
    pub legal: Legal,
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

impl Card {
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
}

#[async_trait]
impl Fetchable for Card {
    // Using the default implementation
}

#[async_trait]
impl Listable for Card {
    // Using the default implementation
}
