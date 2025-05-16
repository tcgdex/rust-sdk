//! Enums used throughout the TCGdex SDK

use strum_macros::{Display, EnumString};

/// Language codes supported by the TCGdex API
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
#[strum(serialize_all = "lowercase")]
#[allow(non_camel_case_types)]
pub enum Language {
    /// English
    EN,
    /// French
    FR,
    /// Spanish
    ES,
    /// Latin Spanish
    #[strum(serialize = "es-mx")]
    ES_MX,
    /// Italian
    IT,
    /// Brazilian Portuguese
    #[strum(serialize = "pt-br")]
    PT_BR,
    /// Portugal Portuguese
    #[strum(serialize = "pt-pt")]
    PT_PT,
    /// German
    DE,
    /// Dutch
    NL,
    /// Polish
    PL,
    /// Russian
    RU,
    /// Japanese
    JA,
    /// Korean
    KO,
    /// Traditional Chinese
    #[strum(serialize = "zh-tw")]
    ZH_TW,
    /// Indonesian
    ID,
    /// Thai
    TH,
    /// Simplified Chinese
    #[strum(serialize = "zh-cn")]
    ZH_CN,
}

impl Default for Language {
    fn default() -> Self {
        Self::EN
    }
}

/// The different image formats available
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Extension {
    /// PNG image with transparent background
    PNG,
    /// JPG image with white background
    JPG,
    /// WEBP image with transparent background
    WEBP,
}

impl Default for Extension {
    fn default() -> Self {
        Self::PNG
    }
}

/// Image quality options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Quality {
    /// High quality image
    HIGH,
    /// Low quality image
    LOW,
}

impl Default for Quality {
    fn default() -> Self {
        Self::HIGH
    }
}