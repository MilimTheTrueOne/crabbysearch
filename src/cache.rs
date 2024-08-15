//! This module provides the functionality to cache the aggregated results fetched and aggregated
//! from the upstream search engines in a json format.

use mini_moka::sync::Cache as MokaCache;
use mini_moka::sync::ConcurrentCacheExt;

use std::time::Duration;

use crate::{config::Config, models::aggregation_models::SearchResults};

impl From<Vec<u8>> for SearchResults {
    fn from(v: Vec<u8>) -> SearchResults {
        serde_json::from_slice(&v)
            .expect("well, this can only be caused by memory corruption so good luck")
    }
}

impl From<&SearchResults> for Vec<u8> {
    fn from(v: &SearchResults) -> Vec<u8> {
        serde_json::to_vec(v).expect("somehow failed to serialize search results")
    }
}

/// Memory based cache backend.
#[derive(Clone)]
pub struct Cache {
    /// The backend cache which stores data.
    cache: MokaCache<String, Vec<u8>>,
}

impl Cache {
    /// Build new cache
    pub fn build(config: &Config) -> Self {
        log::info!("Initializing in-memory cache");

        Self {
            cache: MokaCache::builder()
                .time_to_live(Duration::from_secs(config.cache_expiry_time))
                .build(),
        }
    }

    /// Retrieve Cached results
    pub fn cached_results(&self, url: &str) -> Option<SearchResults> {
        self.cache.get(&url.to_string()).map(|b| b.into())
    }

    /// Cache results
    pub fn cache_results(&self, search_results: &[SearchResults], urls: &[String]) {
        for (url, search_result) in urls.iter().zip(search_results.iter()) {
            self.cache.insert(url.clone(), search_result.into());
        }

        self.cache.sync();
    }
}
