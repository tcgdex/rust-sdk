//! The main TCGdex client

use crate::endpoints::Endpoint;
use crate::models::{Card, CardResume, IntList, Language, Serie, SerieResume, Set, SetResume, StringEndpoint, StringList};

/// TCGdex API client
pub struct TCGdex {
    /// The API endpoint URL
    endpoint: String,
    /// The language to use for API responses
    pub language: Language,
    /// HTTP client for requests
    client: reqwest::Client,
    /// Cards endpoint
    pub card: Endpoint<Card, CardResume>,
    /// Sets endpoint
    pub set: Endpoint<Set, SetResume>,
    /// Series endpoint
    pub serie: Endpoint<Serie, SerieResume>,
    /// Variants endpoint
    pub variant: Endpoint<StringEndpoint, StringList>,
    /// Trainer types endpoint
    pub trainer_type: Endpoint<StringEndpoint, StringList>,
    /// Suffixes endpoint
    pub suffix: Endpoint<StringEndpoint, StringList>,
    /// Stages endpoint
    pub stage: Endpoint<StringEndpoint, StringList>,
    /// Regulation marks endpoint
    pub regulation_mark: Endpoint<StringEndpoint, StringList>,
    /// Energy types endpoint
    pub energy_type: Endpoint<StringEndpoint, StringList>,
    /// Pokedex IDs endpoint
    pub dex_id: Endpoint<StringEndpoint, IntList>,
    /// Types endpoint
    pub type_: Endpoint<StringEndpoint, StringList>,
    /// Retreats endpoint
    pub retreat: Endpoint<StringEndpoint, IntList>,
    /// Rarities endpoint
    pub rarity: Endpoint<StringEndpoint, StringList>,
    /// Illustrators endpoint
    pub illustrator: Endpoint<StringEndpoint, StringList>,
    /// HP values endpoint
    pub hp: Endpoint<StringEndpoint, IntList>,
    /// Categories endpoint
    pub category: Endpoint<StringEndpoint, StringList>,
}

impl TCGdex {
    /// Default API endpoint URL
    pub const DEFAULT_ENDPOINT: &'static str = "https://api.tcgdex.net/v2";

    /// Create a new TCGdex client with the default endpoint and the specified language
    pub fn new(language: Language) -> Self {
        Self::with_endpoint(Self::DEFAULT_ENDPOINT, language)
    }

    /// Create a new TCGdex client with the specified endpoint and language
    pub fn with_endpoint(endpoint: &str, language: Language) -> Self {
        let client = reqwest::Client::builder()
            .user_agent(format!("tcgdex-rust-sdk/{}", crate::VERSION))
            .build()
            .unwrap_or_default();

        let mut sdk = Self {
            endpoint: endpoint.to_string(),
            language,
            client,
            card: Endpoint::new(String::new(), "cards"),
            set: Endpoint::new(String::new(), "sets"),
            serie: Endpoint::new(String::new(), "series"),
            variant: Endpoint::new(String::new(), "variants"),
            trainer_type: Endpoint::new(String::new(), "trainer-types"),
            suffix: Endpoint::new(String::new(), "suffixes"),
            stage: Endpoint::new(String::new(), "stages"),
            regulation_mark: Endpoint::new(String::new(), "regulation-marks"),
            energy_type: Endpoint::new(String::new(), "energy-types"),
            dex_id: Endpoint::new(String::new(), "dex-ids"),
            type_: Endpoint::new(String::new(), "types"),
            retreat: Endpoint::new(String::new(), "retreats"),
            rarity: Endpoint::new(String::new(), "rarities"),
            illustrator: Endpoint::new(String::new(), "illustrators"),
            hp: Endpoint::new(String::new(), "hp"),
            category: Endpoint::new(String::new(), "categories"),
        };

        // Update base URL for all endpoints
        sdk.update_endpoints();
        sdk
    }

    /// Get the current API endpoint URL
    pub fn get_endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Set a new API endpoint URL
    pub fn set_endpoint(&mut self, endpoint: &str) -> &mut Self {
        self.endpoint = endpoint.to_string();
        self.update_endpoints();
        self
    }

    /// Set the language for API responses
    pub fn set_language(&mut self, language: Language) -> &mut Self {
        self.language = language;
        self.update_endpoints();
        self
    }

    /// Get the HTTP client used for requests
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    // Update the base URL for all endpoints
    fn update_endpoints(&mut self) {
        let base_url = format!("{}/{}", self.endpoint, self.language);
        
        self.card.set_base_url(&base_url);
        self.set.set_base_url(&base_url);
        self.serie.set_base_url(&base_url);
        self.variant.set_base_url(&base_url);
        self.trainer_type.set_base_url(&base_url);
        self.suffix.set_base_url(&base_url);
        self.stage.set_base_url(&base_url);
        self.regulation_mark.set_base_url(&base_url);
        self.energy_type.set_base_url(&base_url);
        self.dex_id.set_base_url(&base_url);
        self.type_.set_base_url(&base_url);
        self.retreat.set_base_url(&base_url);
        self.rarity.set_base_url(&base_url);
        self.illustrator.set_base_url(&base_url);
        self.hp.set_base_url(&base_url);
        self.category.set_base_url(&base_url);
    }
}

impl Default for TCGdex {
    fn default() -> Self {
        Self::new(Language::default())
    }
}