//! Endpoint handling for the TCGdex API

use std::fmt::Debug;
use std::marker::PhantomData;

use async_trait::async_trait;
use serde::de::DeserializeOwned;

use crate::error::Result;
use crate::query::Query;
use crate::TCGdex;

/// A trait for models that can be fetched from the API
#[async_trait]
pub trait Fetchable: Sized + DeserializeOwned + Debug {
    /// Fetch a model by ID
    async fn fetch(client: &reqwest::Client, url: &str) -> Result<Self> {
        let response = client.get(url).send().await?.error_for_status()?;
        let data = response.json().await?;
        Ok(data)
    }
}

/// A trait for models that can be listed from the API
#[async_trait]
pub trait Listable: Sized + DeserializeOwned + Debug {
    /// Fetch a list of models
    async fn fetch_list(client: &reqwest::Client, url: &str) -> Result<Vec<Self>> {
        let response = client.get(url).send().await?.error_for_status()?;
        let data = response.json().await?;
        Ok(data)
    }
}

/// Generic endpoint for TCGdex API resources
pub struct Endpoint<Item, List> {
    base_url: String,
    path: String,
    client: reqwest::Client,
    _item: PhantomData<Item>,
    _list: PhantomData<List>,
}

impl<Item, List> Endpoint<Item, List>
where
    Item: Fetchable + Send + Sync,
    List: Listable + Send + Sync,
{
    /// Create a new endpoint with the given base URL and path
    pub fn new(sdk: &TCGdex, path: impl Into<String>) -> Self {
        Self {
            base_url: format!("{}/{}", sdk.get_endpoint(), sdk.language),
            path: path.into(),
            client: sdk.client().clone(),
            _item: PhantomData,
            _list: PhantomData,
        }
    }

    /// Set the base URL for this endpoint
    pub fn set_base_url(&mut self, base_url: &str) -> &mut Self {
        self.base_url = base_url.to_string();
        self
    }

    /// Get a single item by ID
    pub async fn get(&self, id: &str) -> Result<Item> {
        let url = format!("{}/{}/{}", self.base_url, self.path, id.replace(' ', "%20"));

        // Print debug info in debug builds
        #[cfg(debug_assertions)]
        println!("[DEBUG] Fetching get URL: {}", url);

        let response = self.client.get(&url).send().await?;

        #[cfg(debug_assertions)]
        {
            let status = response.status();
            println!("[DEBUG] Response status: {}", status);

            if !status.is_success() {
                let text = response.text().await?;
                println!("[DEBUG] Error response body: {}", text);
                return Err(crate::error::Error::NoData);
            }
        }

        let response = response.error_for_status()?;
        let data = response.json().await?;
        Ok(data)
    }

    /// List all items, optionally filtered by a query
    pub async fn list(&self, query: Option<&Query>) -> Result<Vec<List>> {
        let mut url = format!("{}/{}", self.base_url, self.path);

        if let Some(q) = query {
            url.push_str(&q.build());
        }

        // Print debug info in debug builds
        #[cfg(debug_assertions)]
        println!("[DEBUG] Fetching list URL: {}", url);

        let response = self.client.get(&url).send().await?;

        #[cfg(debug_assertions)]
        {
            let status = response.status();
            println!("[DEBUG] Response status: {}", status);

            if !status.is_success() {
                let text = response.text().await?;
                println!("[DEBUG] Error response body: {}", text);
                return Err(crate::error::Error::NoData);
            }
        }

        let response = response.error_for_status()?;

        // For debugging, check the response content
        #[cfg(debug_assertions)]
        {
            let body = response.text().await?;
            println!("[DEBUG] Response body: {}", body);

            // Re-parse the saved body
            use serde_json::from_str;
            match from_str(&body) {
                Ok(data) => return Ok(data),
                Err(e) => {
                    println!("[DEBUG] JSON parsing error: {}", e);
                    return Err(crate::error::Error::Serialization(e));
                }
            }
        }

        // In release mode, just parse directly
        #[cfg(not(debug_assertions))]
        {
            let data = response.json().await?;
            Ok(data)
        }
    }
}

impl<Item, List> Default for Endpoint<Item, List>
where
    Item: Fetchable + Send + Sync,
    List: Listable + Send + Sync,
{
    fn default() -> Self {
        Self {
            base_url: String::new(),
            path: String::new(),
            client: reqwest::Client::default(),
            _item: PhantomData,
            _list: PhantomData,
        }
    }
}
