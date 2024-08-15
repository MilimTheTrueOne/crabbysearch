//! This module provides the models to parse cookies and search parameters from the search
//! engine website.
use std::borrow::Cow;

use serde::Deserialize;

use crate::config::Style;

/// A named struct which deserializes all the user provided search parameters and stores them.
#[derive(Deserialize)]
pub struct SearchParams {
    /// It stores the search parameter option `q` (or query in simple words)
    /// of the search url.
    pub query: Option<String>,
    /// It stores the search parameter `page` (or pageno in simple words)
    /// of the search url.
    pub page: Option<u32>,
    /// It stores the search parameter `safesearch` (or safe search level in simple words) of the
    /// search url.
    pub safesearch: Option<u8>,
}

/// A named struct which is used to deserialize the cookies fetched from the client side.
#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Cookie<'a> {
    /// It stores the theme name used in the website.
    pub theme: Cow<'a, str>,
    /// It stores the colorscheme name used for the website theme.
    pub colorscheme: Cow<'a, str>,
    /// It stores the user selected upstream search engines selected from the UI.
    pub engines: Cow<'a, Vec<Cow<'a, str>>>,
}

impl<'a> Cookie<'a> {
    /// server_models::Cookie contructor function
    pub fn build(style: &'a Style, mut engines: Vec<Cow<'a, str>>) -> Self {
        engines.sort();
        Self {
            theme: Cow::Borrowed(&style.theme),
            colorscheme: Cow::Borrowed(&style.colorscheme),
            engines: Cow::Owned(engines),
        }
    }
}
