//! Utility functions for the TCGdex SDK

use crate::error::Result;
use crate::models::{Extension, Quality};
use serde::de::{Error as DeError, Visitor};
use serde::Deserializer;
use std::fmt;

/// Deserialize a value that could be either a string or a number into an i32
pub fn deserialize_string_or_number_to_i32<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrInt;

    impl<'de> Visitor<'de> for StringOrInt {
        type Value = Option<i32>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or integer")
        }

        fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
        where
            E: DeError,
        {
            match value.parse::<i32>() {
                Ok(val) => Ok(Some(val)),
                Err(_) => Ok(None),
            }
        }

        fn visit_string<E>(self, value: String) -> std::result::Result<Self::Value, E>
        where
            E: DeError,
        {
            match value.parse::<i32>() {
                Ok(val) => Ok(Some(val)),
                Err(_) => Ok(None),
            }
        }

        fn visit_i64<E>(self, value: i64) -> std::result::Result<Self::Value, E>
        where
            E: DeError,
        {
            if value >= i32::MIN as i64 && value <= i32::MAX as i64 {
                Ok(Some(value as i32))
            } else {
                Err(E::custom(format!("integer out of range: {}", value)))
            }
        }

        fn visit_u64<E>(self, value: u64) -> std::result::Result<Self::Value, E>
        where
            E: DeError,
        {
            if value <= i32::MAX as u64 {
                Ok(Some(value as i32))
            } else {
                Err(E::custom(format!("integer out of range: {}", value)))
            }
        }

        fn visit_none<E>(self) -> std::result::Result<Self::Value, E>
        where
            E: DeError,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> std::result::Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(self)
        }
    }

    deserializer.deserialize_any(StringOrInt)
}

/// Download an image from the given URL
pub async fn download_image(client: &reqwest::Client, url: &str) -> Result<bytes::Bytes> {
    let response = client.get(url).send().await?.error_for_status()?;
    let bytes = response.bytes().await?;
    Ok(bytes)
}

/// Build a full image URL with the given base URL, quality, and extension
pub fn build_image_url(base_url: &str, quality: Quality, extension: Extension) -> String {
    format!("{}/{}.{}", base_url, quality, extension)
}

/// Build a full image URL with the given base URL and extension (for logos and symbols)
pub fn build_logo_url(base_url: &str, extension: Extension) -> String {
    format!("{}.{}", base_url, extension)
}
