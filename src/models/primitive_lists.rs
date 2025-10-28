//! Types for handling lists of primitive values from the API

use crate::endpoints::Listable;
use async_trait::async_trait;
use serde::de::{self, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

/// Wrapper for a list of strings returned from the API
#[derive(Debug, Clone, Serialize)]
pub struct StringList(pub Vec<String>);

// Custom deserialization to handle both array of strings and single string
impl<'de> Deserialize<'de> for StringList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StringListVisitor;

        impl<'de> Visitor<'de> for StringListVisitor {
            type Value = StringList;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string or array of strings")
            }

            // Handle a single string value
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StringList(vec![value.to_string()]))
            }

            // Handle a single string value
            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StringList(vec![value]))
            }

            // Handle an array of strings
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element()? {
                    values.push(value);
                }
                Ok(StringList(values))
            }
        }

        deserializer.deserialize_any(StringListVisitor)
    }
}

impl StringList {
    /// Get a reference to the inner vector
    pub fn inner(&self) -> &Vec<String> {
        &self.0
    }

    /// Check if the list contains a specific string
    pub fn contains(&self, value: &str) -> bool {
        self.0.contains(&value.to_string())
    }

    /// Check if the list is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get the length of the list
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

/// Wrapper for a list of integers returned from the API
#[derive(Debug, Clone, Serialize)]
pub struct IntList(pub Vec<u32>);

// Custom deserialization to handle both array of integers and single integer
impl<'de> Deserialize<'de> for IntList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IntListVisitor;

        impl<'de> Visitor<'de> for IntListVisitor {
            type Value = IntList;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an integer or array of integers")
            }

            // Handle a single integer value
            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value <= u32::MAX as u64 {
                    Ok(IntList(vec![value as u32]))
                } else {
                    Err(E::custom(format!("integer out of range: {}", value)))
                }
            }

            // Handle a single integer value as string
            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value.parse::<u32>() {
                    Ok(val) => Ok(IntList(vec![val])),
                    Err(_) => Err(E::custom(format!("failed to parse '{}' as u32", value))),
                }
            }

            // Handle an array of integers
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut values = Vec::new();
                while let Some(value) = seq.next_element()? {
                    values.push(value);
                }
                Ok(IntList(values))
            }
        }

        deserializer.deserialize_any(IntListVisitor)
    }
}

impl IntList {
    /// Get a reference to the inner vector
    pub fn inner(&self) -> &Vec<u32> {
        &self.0
    }

    /// Check if the list contains a specific integer
    pub fn contains(&self, value: u32) -> bool {
        self.0.contains(&value)
    }

    /// Check if the list is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get the length of the list
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

// Implement Listable for our wrapper types
#[async_trait]
impl Listable for StringList {}

#[async_trait]
impl Listable for IntList {}
