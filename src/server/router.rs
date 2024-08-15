//! This module provides the functionality to handle different routes of the `crabbysearch`
//! meta search engine website and provide appropriate response to each route/page
//! when requested.

use crate::{
    config::Config,
    handler::{file_path, FileType},
};
use actix_web::{get, http::header::ContentType, web, HttpRequest, HttpResponse};
use tokio::fs::read_to_string;

/// Handles the route of index page or main page of the `crabbysearch` meta search engine website.
#[get("/")]
pub async fn index() -> Result<HttpResponse, Box<dyn std::error::Error>> {
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(crate::templates::views::index::index().0))
}

/// Handles the route of any other accessed route/page which is not provided by the
/// website essentially the 404 error page.
pub async fn not_found() -> Result<HttpResponse, Box<dyn std::error::Error>> {
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(crate::templates::views::not_found::not_found().0))
}

/// Handles the route of robots.txt page of the `crabbysearch` meta search engine website.
#[get("/robots.txt")]
pub async fn robots_data(_req: HttpRequest) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let page_content: String =
        read_to_string(format!("{}/robots.txt", file_path(FileType::Theme)?)).await?;
    Ok(HttpResponse::Ok()
        .content_type(ContentType::plaintext())
        .body(page_content))
}

/// Handles the route of about page of the `crabbysearch` meta search engine website.
#[get("/about")]
pub async fn about() -> Result<HttpResponse, Box<dyn std::error::Error>> {
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(crate::templates::views::about::about().0))
}

/// Handles the route of settings page of the `crabbysearch` meta search engine website.
#[get("/settings")]
pub async fn settings(
    config: web::Data<Config>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(
        crate::templates::views::settings::settings(
            &config
                .upstream_search_engines
                .list()
                .iter()
                .map(|n| (*n, true))
                .collect(),
        )?
        .0,
    ))
}
