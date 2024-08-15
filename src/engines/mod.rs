//! This module provides different modules which handles the functionlity to fetch results from the
//! upstream search engines based on user requested queries. Also provides different models to
//! provide a standard functions to be implemented for all the upstream search engine handling
//! code. Moreover, it also provides a custom error for the upstream search engine handling code.

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::models::engine_models::EngineHandler;

pub mod bing;
pub mod brave;
pub mod duckduckgo;
pub mod librex;
pub mod mojeek;
pub mod search_result_parser;
pub mod searx;
pub mod startpage;

/// Struct that keeps track of search engines
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Engines {
    bing: bool,
    brave: bool,
    duckduckgo: bool,
    librex: bool,
    mojeek: bool,
    search_result_parser: bool,
    searx: bool,
    startpage: bool,
}

impl Default for Engines {
    fn default() -> Self {
        Self {
            bing: true,
            brave: true,
            duckduckgo: true,
            librex: true,
            mojeek: true,
            search_result_parser: true,
            searx: true,
            startpage: true,
        }
    }
}

impl From<&Engines> for Vec<EngineHandler> {
    fn from(value: &Engines) -> Self {
        let mut v = vec![];
        if value.duckduckgo {
            let engine = crate::engines::duckduckgo::DuckDuckGo::default();
            v.push(EngineHandler::new("duckduckgo", Arc::new(engine)));
        }
        if value.searx {
            let engine = crate::engines::searx::Searx::default();
            v.push(EngineHandler::new("searx", Arc::new(engine)));
        }
        if value.brave {
            let engine = crate::engines::brave::Brave::default();
            v.push(EngineHandler::new("brave", Arc::new(engine)));
        }
        if value.startpage {
            let engine = crate::engines::startpage::Startpage::default();
            v.push(EngineHandler::new("startpage", Arc::new(engine)));
        }
        if value.librex {
            let engine = crate::engines::librex::LibreX::default();
            v.push(EngineHandler::new("librex", Arc::new(engine)));
        }
        if value.mojeek {
            let engine = crate::engines::mojeek::Mojeek::default();
            v.push(EngineHandler::new("mojeek", Arc::new(engine)));
        }
        if value.bing {
            let engine = crate::engines::bing::Bing::default();
            v.push(EngineHandler::new("bing", Arc::new(engine)));
        }
        v
    }
}
