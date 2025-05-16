//! Data models for the TCGdex API

pub mod enums;
mod card;
mod card_resume;
mod serie;
mod serie_resume;
mod set;
mod set_resume;
mod string_endpoint;
mod int_endpoint;
mod common;
mod primitive_lists;

pub use self::card::Card;
pub use self::card_resume::CardResume;
pub use self::serie::Serie;
pub use self::serie_resume::SerieResume;
pub use self::set::Set;
pub use self::set_resume::SetResume;
pub use self::string_endpoint::StringEndpoint;
pub use self::int_endpoint::IntEndpoint;
pub use self::primitive_lists::{StringList, IntList};
pub use common::*;
pub use enums::*;