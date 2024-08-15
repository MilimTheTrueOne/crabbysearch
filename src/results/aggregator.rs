//! This module provides the functionality to scrape and gathers all the results from the upstream
//! search engines and then removes duplicate results.

use super::user_agent::random_user_agent;
use crate::config::Config;
use crate::models::{
    aggregation_models::{EngineErrorInfo, SearchResult, SearchResults},
    engine_models::{EngineError, EngineHandler},
};

use error_stack::Report;
use futures::stream::FuturesUnordered;
use reqwest::{Client, ClientBuilder};
use std::sync::Arc;
use tokio::{task::JoinHandle, time::Duration};

/// A constant for holding the prebuilt Client globally in the app.
static CLIENT: std::sync::OnceLock<Client> = std::sync::OnceLock::new();

/// Aliases for long type annotations

type FutureVec =
    FuturesUnordered<JoinHandle<Result<Vec<(String, SearchResult)>, Report<EngineError>>>>;

/// The function aggregates the scraped results from the user-selected upstream search engines.
/// These engines can be chosen either from the user interface (UI) or from the configuration file.
/// The code handles this process by matching the selected search engines and adding them to a vector.
/// This vector is then used to create an asynchronous task vector using `tokio::spawn`, which returns
/// a future. This future is awaited in another loop. Once the results are collected, they are filtered
/// to remove any errors and ensure only proper results are included. If an error is encountered, it is
/// sent to the UI along with the name of the engine and the type of error. This information is finally
/// placed in the returned `SearchResults` struct.
///
/// Additionally, the function eliminates duplicate results. If two results are identified as coming from
/// multiple engines, their names are combined to indicate that the results were fetched from these upstream
/// engines. After this, all the data in the `Vec` is removed and placed into a struct that contains all
/// the aggregated results in a vector. Furthermore, the query used is also added to the struct. This step is
/// necessary to ensure that the search bar in the search remains populated even when searched from the query URL.
///
/// Overall, this function serves to aggregate scraped results from user-selected search engines, handling errors,
/// removing duplicates, and organizing the data for display in the UI.
///
/// # Example:
///
/// If you search from the url like `https://127.0.0.1/search?q=huston` then the search bar should
/// contain the word huston and not remain empty.
///
/// # Arguments
///
/// * `query` - Accepts a string to query with the above upstream search engines.
/// * `page` - Accepts an u32 page number.
/// * `random_delay` - Accepts a boolean value to add a random delay before making the request.
/// * `debug` - Accepts a boolean value to enable or disable debug mode option.
/// * `upstream_search_engines` - Accepts a vector of search engine names which was selected by the
/// * `request_timeout` - Accepts a time (secs) as a value which controls the server request timeout.
///     user through the UI or the config file.
///
/// # Error
///
/// Returns an error a reqwest and scraping selector errors if any error occurs in the results
/// function in either `searx` or `duckduckgo` or both otherwise returns a `SearchResults struct`
/// containing appropriate values.
pub async fn aggregate(
    query: &str,
    page: u32,
    config: actix_web::web::Data<Config>,
    upstream_search_engines: &[EngineHandler],
) -> Result<SearchResults, Box<dyn std::error::Error>> {
    let client = CLIENT.get_or_init(|| {
        ClientBuilder::new()
            .timeout(Duration::from_secs(config.request_timeout as u64)) // Add timeout to request to avoid DDOSing the server
            .pool_idle_timeout(Duration::from_secs(
                config.pool_idle_connection_timeout as u64,
            ))
            .tcp_keepalive(Duration::from_secs(config.tcp_connection_keep_alive as u64))
            .connect_timeout(Duration::from_secs(config.request_timeout as u64)) // Add timeout to request to avoid DDOSing the server
            .https_only(true)
            .gzip(true)
            .brotli(true)
            .http2_adaptive_window(config.adaptive_window)
            .build()
            .unwrap()
    });

    let user_agent: &str = random_user_agent();

    let mut names: Vec<&str> = Vec::with_capacity(0);

    // create tasks for upstream result fetching
    let tasks: FutureVec = FutureVec::new();

    let query: Arc<String> = Arc::new(query.to_string());
    for engine_handler in upstream_search_engines {
        let (name, search_engine) = engine_handler.clone().into_name_engine();
        names.push(name);
        let query_partially_cloned = query.clone();
        tasks.push(tokio::spawn(async move {
            search_engine
                .results(&query_partially_cloned, page, user_agent, client)
                .await
        }));
    }

    // get upstream responses
    let mut responses = Vec::with_capacity(tasks.len());

    for task in tasks {
        if let Ok(result) = task.await {
            responses.push(result)
        }
    }

    // aggregate search results, removing duplicates and handling errors the upstream engines returned
    let mut result_map: Vec<(String, SearchResult)> = Vec::new();
    let mut engine_errors_info: Vec<EngineErrorInfo> = Vec::new();

    let mut handle_error = |error: &Report<EngineError>, engine_name: &'static str| {
        log::error!("Engine Error: {:?}", error);
        engine_errors_info.push(EngineErrorInfo::new(
            error.downcast_ref::<EngineError>().unwrap(),
            engine_name,
        ));
    };

    for _ in 0..responses.len() {
        let response = responses.pop().unwrap();
        let engine = names.pop().unwrap();

        if result_map.is_empty() {
            match response {
                Ok(results) => result_map = results,
                Err(error) => handle_error(&error, engine),
            };
            continue;
        }

        match response {
            Ok(result) => {
                result.into_iter().for_each(|(key, value)| {
                    match result_map.iter().find(|(key_s, _)| key_s == &key) {
                        Some(value) => value.1.to_owned().add_engines(engine),
                        None => result_map.push((key, value)),
                    };
                });
            }
            Err(error) => handle_error(&error, engine),
        };
    }

    let mut results: Vec<SearchResult> =
        result_map.iter().map(|(_, value)| value.clone()).collect();

    results.sort_by(|a, b| a.description.len().cmp(&b.description.len()));
    Ok(SearchResults::new(results, &engine_errors_info))
}
