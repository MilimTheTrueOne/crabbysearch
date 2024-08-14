//! This module handles the search route of the search engine website.

use crate::{
    cache::Cache,
    config::parser::Config,
    models::{
        aggregation_models::SearchResults,
        engine_models::EngineHandler,
        server_models::{self, SearchParams},
    },
    results::aggregator::aggregate,
};
use actix_web::{get, http::header::ContentType, web, HttpRequest, HttpResponse};
use std::borrow::Cow;
use tokio::join;

/// Handles the route of search page of the `websurfx` meta search engine website and it takes
/// two search url parameters `q` and `page` where `page` parameter is optional.
///
/// # Example
///
/// ```bash
/// curl "http://127.0.0.1:8080/search?q=sweden&page=1"
/// ```
///
/// Or
///
/// ```bash
/// curl "http://127.0.0.1:8080/search?q=sweden"
/// ```
#[get("/search")]
pub async fn search(
    req: HttpRequest,
    config: web::Data<Config>,
    cache: web::Data<Cache>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    use std::sync::Arc;
    let params = web::Query::<SearchParams>::from_query(req.query_string())?;
    match &params.q {
        Some(query) => {
            if query.trim().is_empty() {
                return Ok(HttpResponse::TemporaryRedirect()
                    .insert_header(("location", "/"))
                    .finish());
            }

            let cookie = req.cookie("appCookie");

            // Get search settings using the user's cookie or from the server's config
            let search_settings: server_models::Cookie<'_> = cookie
                .and_then(|cookie_value| serde_json::from_str(cookie_value.value()).ok())
                .unwrap_or_else(|| {
                    server_models::Cookie::build(
                        &config.style,
                        config
                            .upstream_search_engines
                            .iter()
                            .map(|e| Cow::Borrowed(e.as_str()))
                            .collect(),
                    )
                });

            // Closure wrapping the results function capturing local references
            let get_results =
                |page| results(config.clone(), cache.clone(), query, page, &search_settings);

            // .max(1) makes sure that the page >= 0.
            let page = params.page.unwrap_or(1).max(1) - 1;
            let previous_page = page.saturating_sub(1);
            let next_page = page + 1;

            let mut results = Arc::new((SearchResults::default(), String::default()));
            if page != previous_page {
                let (previous_results, current_results, next_results) = join!(
                    get_results(previous_page),
                    get_results(page),
                    get_results(next_page)
                );
                let (parsed_previous_results, parsed_next_results) =
                    (previous_results?, next_results?);

                let (cache_keys, results_list) = (
                    [
                        parsed_previous_results.1,
                        results.1.clone(),
                        parsed_next_results.1,
                    ],
                    [
                        parsed_previous_results.0,
                        results.0.clone(),
                        parsed_next_results.0,
                    ],
                );

                results = Arc::new(current_results?);

                tokio::spawn(async move { cache.cache_results(&results_list, &cache_keys) });
            } else {
                let (current_results, next_results) =
                    join!(get_results(page), get_results(page + 1));

                let parsed_next_results = next_results?;

                results = Arc::new(current_results?);

                let (cache_keys, results_list) = (
                    [results.1.clone(), parsed_next_results.1.clone()],
                    [results.0.clone(), parsed_next_results.0],
                );

                tokio::spawn(async move { cache.cache_results(&results_list, &cache_keys) });
            }

            Ok(HttpResponse::Ok().content_type(ContentType::html()).body(
                crate::templates::views::search::search(
                    &config.style.colorscheme,
                    &config.style.theme,
                    &config.style.animation,
                    query,
                    &results.0,
                )
                .0,
            ))
        }
        None => Ok(HttpResponse::TemporaryRedirect()
            .insert_header(("location", "/"))
            .finish()),
    }
}

/// Fetches the results for a query and page. It First checks the redis cache, if that
/// fails it gets proper results by requesting from the upstream search engines.
///
/// # Arguments
///
/// * `url` - It takes the url of the current page that requested the search results for a
///     particular search query.
/// * `config` - It takes a parsed config struct.
/// * `query` - It takes the page number as u32 value.
/// * `req` - It takes the `HttpRequest` struct as a value.
///
/// # Error
///
/// It returns the `SearchResults` struct if the search results could be successfully fetched from
/// the cache or from the upstream search engines otherwise it returns an appropriate error.
async fn results(
    config: web::Data<crate::config::parser::Config>,
    cache: web::Data<crate::cache::Cache>,
    query: &str,
    page: u32,
    search_settings: &server_models::Cookie<'_>,
) -> Result<(SearchResults, String), Box<dyn std::error::Error>> {
    // eagerly parse cookie value to evaluate safe search level

    let cache_key = format!(
        "http://{}:{}/search?q={}&page={}&engines={}",
        config.binding_ip,
        config.port,
        query,
        page,
        search_settings.engines.join(",")
    );

    // fetch the cached results json.
    let cached_results = cache.cached_results(&cache_key);
    // check if fetched cache results was indeed fetched or it was an error and if so
    // handle the data accordingly.
    match cached_results {
        Some(results) => Ok((results, cache_key)),
        None => {
            // check if the cookie value is empty or not if it is empty then use the
            // default selected upstream search engines from the config file otherwise
            // parse the non-empty cookie and grab the user selected engines from the
            // UI and use that.
            let mut results: SearchResults = match search_settings.engines.is_empty() {
                false => {
                    aggregate(
                        query,
                        page,
                        config,
                        &search_settings
                            .engines
                            .iter()
                            .filter_map(|engine| EngineHandler::new(engine).ok())
                            .collect::<Vec<EngineHandler>>(),
                    )
                    .await?
                }
                true => {
                    let mut search_results = SearchResults::default();
                    search_results.set_no_engines_selected();
                    search_results
                }
            };
            let (engine_errors_info, results_empty_check, no_engines_selected) = (
                results.engine_errors_info().is_empty(),
                results.results().is_empty(),
                results.no_engines_selected(),
            );
            results.set_filtered(engine_errors_info & results_empty_check & !no_engines_selected);
            cache.cache_results(&[results.clone()], &[cache_key.clone()]);
            Ok((results, cache_key))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    /// A helper function which creates a random mock safe search level value.
    ///
    /// # Returns
    ///
    /// Returns an optional u8 value.
    fn mock_safe_search_level_value() -> Option<u8> {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos() as f32;
        let delay = ((nanos / 1_0000_0000 as f32).floor() as i8) - 1;

        match delay {
            -1 => None,
            some_num => Some(if some_num > 4 { some_num - 4 } else { some_num } as u8),
        }
    }

    #[test]
    /// A test function to test whether the output of the branchless and branched code
    /// for the code to choose the appropriate safe search level is same or not.
    fn get_safesearch_level_branched_branchless_code_test() {
        // Get mock values for the safe search level values for URL parameters, cookie
        // and config.
        let safe_search_level_from_url = mock_safe_search_level_value();
        let cookie_safe_search_level = mock_safe_search_level_value().unwrap_or(0);
        let config_safe_search_level = mock_safe_search_level_value().unwrap_or(0);

        // Branched code
        let safe_search_level_value_from_branched_code = match safe_search_level_from_url {
            Some(safe_search_level_from_url_parsed) => {
                if config_safe_search_level >= 3 {
                    config_safe_search_level
                } else {
                    safe_search_level_from_url_parsed
                }
            }
            None => {
                if config_safe_search_level >= 3 {
                    config_safe_search_level
                } else {
                    cookie_safe_search_level
                }
            }
        };

        // branchless code
        let safe_search_level_value_from_branchless_code =
            (u8::from(safe_search_level_from_url.is_some())
                * ((u8::from(config_safe_search_level >= 3) * config_safe_search_level)
                    + (u8::from(config_safe_search_level < 3)
                        * safe_search_level_from_url.unwrap_or(0))))
                + (u8::from(safe_search_level_from_url.is_none())
                    * ((u8::from(config_safe_search_level >= 3) * config_safe_search_level)
                        + (u8::from(config_safe_search_level < 3) * cookie_safe_search_level)));

        assert_eq!(
            safe_search_level_value_from_branched_code,
            safe_search_level_value_from_branchless_code
        );
    }
}
