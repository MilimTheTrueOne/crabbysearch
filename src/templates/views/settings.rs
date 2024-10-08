//! A module that handles the view for the settings page in the `crabbysearch` frontend.

use maud::{html, Markup};

use crate::templates::partials::{
    footer::footer,
    header::header,
    settings_tabs::{cookies::cookies, engines::engines},
};

/// A function that handles the html code for the settings page view in the search engine frontend.
///
/// # Arguments
///
/// * `safe_search_level` - It takes the safe search level as an argument.
/// * `colorscheme` - It takes the colorscheme name as an argument.
/// * `theme` - It takes the theme name as an argument.
/// * `animation` - It takes the animation name as an argument.
/// * `engine_names` - It takes a list of engine names as an argument.
///
/// # Error
///
/// This function returns a compiled html markup code on success otherwise returns a standard error
/// message.
pub fn settings(
    engine_names: &Vec<(&'static str, bool)>,
) -> Result<Markup, Box<dyn std::error::Error>> {
    Ok(html!(
        (header())
        main class="settings"{
           h1{"Settings"}
           hr;
           .settings_container{
              .sidebar{
                  div class="btn active" onclick="setActiveTab(this)"{"general"}
                  .btn onclick="setActiveTab(this)"{"user interface"}
                  .btn onclick="setActiveTab(this)"{"engines"}
                  .btn onclick="setActiveTab(this)"{"cookies"}
              }
              .main_container{
                  (engines(&engine_names))
                  (cookies())
                  p class="message"{}
                  button type="submit" onclick="setClientSettings()"{"Save"}
              }
           }
        }
        script src="static/settings.js"{}
        script src="static/cookies.js"{}
        (footer())
    ))
}
