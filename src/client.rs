//! The main TCGdex client

use crate::endpoints::Endpoint;
use crate::models::{
    Card, CardResume, IntList, Language, Serie, SerieResume, Set, SetResume, StringEndpoint,
    StringList,
};

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
            card: Endpoint::default(),
            set: Endpoint::default(),
            serie: Endpoint::default(),
            variant: Endpoint::default(),
            trainer_type: Endpoint::default(),
            suffix: Endpoint::default(),
            stage: Endpoint::default(),
            regulation_mark: Endpoint::default(),
            energy_type: Endpoint::default(),
            dex_id: Endpoint::default(),
            type_: Endpoint::default(),
            retreat: Endpoint::default(),
            rarity: Endpoint::default(),
            illustrator: Endpoint::default(),
            hp: Endpoint::default(),
            category: Endpoint::default(),
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

    // Initialize or update all endpoints
    fn update_endpoints(&mut self) {
        self.card = Endpoint::new(self, "cards");
        self.set = Endpoint::new(self, "sets");
        self.serie = Endpoint::new(self, "series");
        self.variant = Endpoint::new(self, "variants");
        self.trainer_type = Endpoint::new(self, "trainer-types");
        self.suffix = Endpoint::new(self, "suffixes");
        self.stage = Endpoint::new(self, "stages");
        self.regulation_mark = Endpoint::new(self, "regulation-marks");
        self.energy_type = Endpoint::new(self, "energy-types");
        self.dex_id = Endpoint::new(self, "dex-ids");
        self.type_ = Endpoint::new(self, "types");
        self.retreat = Endpoint::new(self, "retreats");
        self.rarity = Endpoint::new(self, "rarities");
        self.illustrator = Endpoint::new(self, "illustrators");
        self.hp = Endpoint::new(self, "hp");
        self.category = Endpoint::new(self, "categories");
    }
}

impl Default for TCGdex {
    fn default() -> Self {
        Self::new(Language::default())
    }
}
