//! Common structures used across TCGdex models

use serde::{Deserialize, Serialize};

/// Card ability information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardAbility {
    /// The ability type (e.g., "Poku00e9mon Power", "Ability")
    #[serde(rename = "type")]
    pub type_: String,
    /// Name of the ability
    pub name: Option<String>,
    /// Description/Effect of the ability
    pub effect: Option<String>,
}

/// Card attack information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardAttack {
    /// Name of the attack
    pub name: Option<String>,
    /// Cost of the attack (energy types)
    pub cost: Option<Vec<String>>,
    /// Effect/Description of the attack
    pub effect: Option<String>,
    /// Damage the attack deals (numeric or formula)
    #[serde(default)]
    pub damage: Option<String>,
}

/// Card item information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardItem {
    /// The item name
    pub name: Option<String>,
    /// The item effect
    pub effect: Option<String>,
}

/// Card variants information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardVariants {
    /// Basic variant (no special effects)
    pub normal: bool,
    /// Reverse holo variant
    pub reverse: bool,
    /// Holo variant
    pub holo: bool,
    /// First Edition variant
    #[serde(rename = "firstEdition")]
    pub first_edition: bool,
    /// W Promo variant
    #[serde(rename = "wPromo")]
    pub w_promo: bool,
}

/// Card weakness/resistance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardWeakRes {
    /// The affecting energy type
    #[serde(rename = "type")]
    pub type_: String,
    /// The multiplier/value (e.g., "x2", "-30")
    pub value: Option<String>,
}

/// Card legality information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Legal {
    /// Card is usable in standard format
    pub standard: bool,
    /// Card is usable in expanded format
    pub expanded: bool,
}

/// Set card count information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCardCount {
    /// Total number of cards
    pub total: i32,
    /// Number of cards officially numbered
    pub official: i32,
    /// Number of cards with normal variant
    pub normal: Option<i32>,
    /// Number of cards with reverse variant
    pub reverse: Option<i32>,
    /// Number of cards with holo variant
    pub holo: Option<i32>,
    /// Number of cards with first edition variant
    #[serde(rename = "firstEd")]
    pub first_ed: Option<i32>,
}

/// Set card count summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCardCountResume {
    /// Total number of cards
    pub total: i32,
    /// Number of cards officially numbered
    pub official: i32,
}