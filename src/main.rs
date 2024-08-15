//! This main library module provides the functionality to provide and handle the Tcp server
//! and register all the routes for the `crabbysearch` meta search engine website.

#![forbid(unsafe_code, clippy::panic)]
#![deny(missing_docs, clippy::perf)]
#![warn(clippy::cognitive_complexity, rust_2018_idioms)]

pub mod cache;
pub mod config;
pub mod engines;
pub mod handler;
pub mod models;
pub mod results;
pub mod server;
pub mod templates;

use std::net::TcpListener;

use crate::cache::Cache;
use crate::server::router;

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{
    http::header,
    middleware::{Compress, Logger},
    web, App, HttpServer,
};
use config::Config;
use handler::{file_path, FileType};

/// Runs the web server
#[actix_web::main]
async fn main() {
    let config = Config::parse();
    let cache = Cache::build(&config);

    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .init();

    log::info!(
        "starting server on port {} and IP {}",
        config.port,
        config.binding_ip
    );
    log::info!(
        "Open http://{}:{}/ in your browser",
        config.binding_ip,
        config.port,
    );

    let listener = TcpListener::bind((config.binding_ip.clone(), config.port))
        .expect("could not create TcpListener");

    let public_folder_path: &str = file_path(FileType::Theme).unwrap();

    let _ = HttpServer::new(move || {
        let cors: Cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET"])
            .allowed_headers(vec![
                header::ORIGIN,
                header::CONTENT_TYPE,
                header::REFERER,
                header::COOKIE,
            ]);

        App::new()
            // Compress the responses provided by the server for the client requests.
            .wrap(Compress::default())
            .wrap(Logger::default()) // added logging middleware for logging.
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(cache.clone()))
            .wrap(cors)
            // Serve images and static files (css and js files).
            .service(
                fs::Files::new("/static", format!("{}/static", public_folder_path))
                    .show_files_listing(),
            )
            .service(
                fs::Files::new("/images", format!("{}/images", public_folder_path))
                    .show_files_listing(),
            )
            .service(router::robots_data) // robots.txt
            .service(router::index) // index page
            .service(server::routes::search::search) // search page
            .service(router::about) // about page
            .service(router::settings) // settings page
            .default_service(web::route().to(router::not_found)) // error page
    })
    // Start server on 127.0.0.1 with the user provided port number. for example 127.0.0.1:8080.
    .listen(listener)
    .expect("could not bind to TCP listener")
    .run()
    .await;
}
