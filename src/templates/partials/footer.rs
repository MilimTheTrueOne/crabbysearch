//! A module that handles the footer for all the pages in the `crabbysearch` frontend.

use maud::{html, Markup, PreEscaped};

/// A functions that handles the html code for the footer for all the pages in the search engine
/// frontend.
///
/// # Returns
///
/// It returns the compiled html code for the footer as a result.
pub fn footer() -> Markup {
    html!(
        footer{
           div{
              span{"Powered By "b{"Crabbysearch"}}span{"-"}span{"a lightning-fast, privacy respecting, secure meta
                  search engine"}
           }
           div{
              ul{
                  li{a href="https://github.com/MilimTheTrueOne/crabbysearch"{"Source Code"}}
                  li{a href="https://github.com/MilimTheTrueOne/crabbysearch/issues"{"Issues/Bugs"}}
              }
           }
        }
        script src="static/settings.js"{}
        (PreEscaped("</body>"))
        (PreEscaped("</html>"))
    )
}
