//! This module handles the search route of the search engine website.

use crate::{
    cache::Cache,
    config::Config,
    engines::Engines,
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

/// Handles the route of search page of the `crabbysearch` meta search engine website and it takes
/// two search url parameters `q` and `page` where `page` parameter is optional.
///
/// # Example
///
/// ```bash
/// wget "http://127.0.0.1:8080/search?q=sweden&page=1"
/// ```
///
/// Or
///
/// ```bash
/// wget "http://127.0.0.1:8080/search?q=sweden"
/// ```
#[get("/search")]
pub async fn search(
    req: HttpRequest,
    config: web::Data<Config>,
    cache: web::Data<Cache>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let params = web::Query::<SearchParams>::from_query(req.query_string())?;

    if params.q.as_ref().is_some_and(|q| q.trim().is_empty()) || params.q.is_none() {
        return Ok(HttpResponse::TemporaryRedirect()
            .insert_header(("location", "/"))
            .finish());
    }

    let query = params.q.as_ref().unwrap().trim();

    let cookie = req.cookie("appCookie");

    // Get search settings using the user's cookie or from the server's config
    let search_settings: crate::engines::Engines = cookie
        .and_then(|cookie_value| serde_json::from_str(cookie_value.value()).ok())
        .unwrap();

    // Closure wrapping the results function capturing local references
    let get_results = |page| results(config.clone(), cache.clone(), query, page, &search_settings);

    // .max(1) makes sure that the page >= 0.
    let page = params.page.unwrap_or(1).max(1) - 1;
    let previous_page = page.saturating_sub(1);
    let next_page = page + 1;

    let mut results = (SearchResults::default(), String::default());
    if page != previous_page {
        let (previous_results, current_results, next_results) = join!(
            get_results(previous_page),
            get_results(page),
            get_results(next_page)
        );
        let (parsed_previous_results, parsed_next_results) = (previous_results?, next_results?);

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

        results = current_results?;

        cache.cache_results(&results_list, &cache_keys);
    } else {
        let (current_results, next_results) = join!(get_results(page), get_results(page + 1));

        let parsed_next_results = next_results?;

        results = current_results?;

        let (cache_keys, results_list) = (
            [results.1.clone(), parsed_next_results.1.clone()],
            [results.0.clone(), parsed_next_results.0],
        );

        cache.cache_results(&results_list, &cache_keys);
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
    config: web::Data<Config>,
    cache: web::Data<crate::cache::Cache>,
    query: &str,
    page: u32,
    upstream: &Engines,
) -> Result<(SearchResults, String), Box<dyn std::error::Error>> {
    // eagerly parse cookie value to evaluate safe search level

    let cache_key = format!("search?q={}&page={}&engines={:?}", query, page, upstream);

    // fetch the cached results json.
    let response = cache.cached_results(&cache_key);

    if let Some(results) = response {
        return Ok((results, cache_key));
    }

    // check if the cookie value is empty or not if it is empty then use the
    // default selected upstream search engines from the config file otherwise
    // parse the non-empty cookie and grab the user selected engines from the
    // UI and use that.
    let mut results: SearchResults = match true {
        false => aggregate(query, page, config, &Vec::<EngineHandler>::from(upstream)).await?,
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
