//! A module that handles `bar` partial for the `search_bar` partial and the home/index/main page in the `crabbysearch` frontend.

use maud::{html, Markup, PreEscaped};

/// A functions that handles the html code for the bar for the `search_bar` partial and the
/// home/index/main page in the search engine frontend.
///
/// # Arguments
///
/// * `query` - It takes the current search query provided by user as an argument.
///
/// # Returns
///
/// It returns the compiled html code for the search bar as a result.
pub fn bar(query: &str) -> Markup {
    html!(
        (PreEscaped("<div class=\"search_bar\">"))
            input type="search" name="search-box" value=(query) placeholder="Type to search";
            button type="submit" onclick="searchWeb()" {
                img src="./images/magnifying_glass.svg" alt="Info icon for error box";
            }
    )
}
