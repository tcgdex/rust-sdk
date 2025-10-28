//! Query builder for filtering API results

use std::fmt;

/// A query builder for filtering API results
pub struct Query {
    params: Vec<QueryParam>,
}

/// A query parameter for filtering API results
struct QueryParam {
    key: String,
    value: String,
}

impl Query {
    /// Create a new empty query
    pub fn new() -> Self {
        Self { params: Vec::new() }
    }

    /// Add a query parameter for filtering records that contain the given value
    pub fn contains<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: fmt::Display,
        V: fmt::Display,
    {
        self.params.push(QueryParam {
            key: key.to_string(),
            value: value.to_string(),
        });
        self
    }

    /// Add a query parameter for filtering records where the field equals the given value
    pub fn equal<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: fmt::Display,
        V: fmt::Display,
    {
        self.params.push(QueryParam {
            key: key.to_string(),
            value: format!("eq:{}", value),
        });
        self
    }

    /// Add a query parameter for filtering records where the field does not equal the given value
    pub fn not_equal<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: fmt::Display,
        V: fmt::Display,
    {
        self.params.push(QueryParam {
            key: key.to_string(),
            value: format!("neq:{}", value),
        });
        self
    }

    /// Add a query parameter for filtering records where the field does not contain the given value
    pub fn not_contains<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: fmt::Display,
        V: fmt::Display,
    {
        self.params.push(QueryParam {
            key: key.to_string(),
            value: format!("not:{}", value),
        });
        self
    }

    /// Add a query parameter for filtering records where the field is greater than the given value
    pub fn greater_than<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: fmt::Display,
        V: fmt::Display,
    {
        self.params.push(QueryParam {
            key: key.to_string(),
            value: format!("gt:{}", value),
        });
        self
    }

    /// Add a query parameter for filtering records where the field is greater than or equal to the given value
    pub fn greater_or_equal_than<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: fmt::Display,
        V: fmt::Display,
    {
        self.params.push(QueryParam {
            key: key.to_string(),
            value: format!("gte:{}", value),
        });
        self
    }

    /// Add a query parameter for filtering records where the field is less than the given value
    pub fn less_than<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: fmt::Display,
        V: fmt::Display,
    {
        self.params.push(QueryParam {
            key: key.to_string(),
            value: format!("lt:{}", value),
        });
        self
    }

    /// Add a query parameter for filtering records where the field is less than or equal to the given value
    pub fn less_or_equal_than<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: fmt::Display,
        V: fmt::Display,
    {
        self.params.push(QueryParam {
            key: key.to_string(),
            value: format!("lte:{}", value),
        });
        self
    }

    /// Add a query parameter for filtering records where the field is null
    pub fn is_null<K>(&mut self, key: K) -> &mut Self
    where
        K: fmt::Display,
    {
        self.params.push(QueryParam {
            key: key.to_string(),
            value: "null:".to_string(),
        });
        self
    }

    /// Add a query parameter for filtering records where the field is not null
    pub fn not_null<K>(&mut self, key: K) -> &mut Self
    where
        K: fmt::Display,
    {
        self.params.push(QueryParam {
            key: key.to_string(),
            value: "notnull:".to_string(),
        });
        self
    }

    /// Add query parameters for sorting results by the given field and order
    pub fn sort<K, V>(&mut self, key: K, order: V) -> &mut Self
    where
        K: fmt::Display,
        V: fmt::Display,
    {
        self.params.push(QueryParam {
            key: "sort:field".to_string(),
            value: key.to_string(),
        });
        self.params.push(QueryParam {
            key: "sort:order".to_string(),
            value: order.to_string(),
        });
        self
    }

    /// Add query parameters for paginating results
    pub fn paginate(&mut self, page: u32, items_per_page: u32) -> &mut Self {
        self.params.push(QueryParam {
            key: "pagination:page".to_string(),
            value: page.to_string(),
        });
        self.params.push(QueryParam {
            key: "pagination:itemsPerPage".to_string(),
            value: items_per_page.to_string(),
        });
        self
    }

    /// Add query parameter to limit the number of results returned
    pub fn limit(&mut self, count: u32) -> &mut Self {
        self.params.push(QueryParam {
            key: "pagination:itemsPerPage".to_string(),
            value: count.to_string(),
        });
        self
    }

    /// Build the query string
    pub fn build(&self) -> String {
        if self.params.is_empty() {
            return String::new();
        }

        let params: Vec<String> = self
            .params
            .iter()
            .map(|param| format!("{0}={1}", encode(&param.key), encode(&param.value)))
            .collect();

        format!("?{}", params.join("&"))
    }

    // Convenience aliases

    /// Alias for contains
    pub fn includes<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: fmt::Display,
        V: fmt::Display,
    {
        self.contains(key, value)
    }

    /// Alias for contains
    pub fn like<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: fmt::Display,
        V: fmt::Display,
    {
        self.contains(key, value)
    }

    /// Alias for not_contains
    pub fn not_includes<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: fmt::Display,
        V: fmt::Display,
    {
        self.not_contains(key, value)
    }

    /// Alias for not_contains
    pub fn not_like<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: fmt::Display,
        V: fmt::Display,
    {
        self.not_contains(key, value)
    }

    /// Alias for greater_than
    pub fn gt<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: fmt::Display,
        V: fmt::Display,
    {
        self.greater_than(key, value)
    }

    /// Alias for greater_or_equal_than
    pub fn gte<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: fmt::Display,
        V: fmt::Display,
    {
        self.greater_or_equal_than(key, value)
    }

    /// Alias for less_than
    pub fn lt<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: fmt::Display,
        V: fmt::Display,
    {
        self.less_than(key, value)
    }

    /// Alias for less_or_equal_than
    pub fn lte<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: fmt::Display,
        V: fmt::Display,
    {
        self.less_or_equal_than(key, value)
    }
}

impl Default for Query {
    fn default() -> Self {
        Self::new()
    }
}

/// URL encode a string, replacing spaces with %20 and removing certain characters
fn encode(s: &str) -> String {
    // Replace spaces with %20 and encode other characters
    let s = s.replace(' ', "%20");
    // Remove quotes, backslashes, and certain Unicode characters (simplified for now)
    let s = s.replace(['"', '\'', '\\'], "");
    // Encode the string
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}
