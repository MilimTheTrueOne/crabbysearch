//! A module that handles the view for the about page in the `crabbysearch` frontend.

use maud::{html, Markup, PreEscaped};

use crate::templates::partials::{footer::footer, header::header};

/// A function that handles the html code for the about page view in the search engine frontend.
///
/// # Arguments
///
/// * `colorscheme` - It takes the colorscheme name as an argument.
/// * `theme` - It takes the theme name as an argument.
///
/// # Returns
///
/// It returns the compiled html markup code as a result.
pub fn about() -> Markup {
    let feature_lightning = r#"
        <svg xmlns="http://www.w3.org/2000/svg" width="60" viewBox="0 0 256 256"><path fill="currentColor" d="m213.85 125.46l-112 120a8 8 0 0 1-13.69-7l14.66-73.33l-57.63-21.64a8 8 0 0 1-3-13l112-120a8 8 0 0 1 13.69 7l-14.7 73.41l57.63 21.61a8 8 0 0 1 3 12.95Z"/></svg>
    "#;
    let feature_secure = r#"
        <svg xmlns="http://www.w3.org/2000/svg" width="60" viewBox="0 0 24 24"><path fill="currentColor" fill-rule="evenodd" d="M1.25 12a5.75 5.75 0 0 1 10.8-2.75H21c.966 0 1.75.784 1.75 1.75v2.5a.75.75 0 0 1-.75.75h-2.25V16a.75.75 0 0 1-.75.75h-2.5a.75.75 0 0 1-.75-.75v-1.75h-3.457A5.751 5.751 0 0 1 1.25 12M7 10a2 2 0 1 0 0 4a2 2 0 0 0 0-4" clip-rule="evenodd"/></svg>
    "#;
    let feature_clean = r#"
        <svg xmlns="http://www.w3.org/2000/svg" width="60" viewBox="0 0 24 24"><path fill="currentColor" d="M8.665 15.735c.245.173.537.265.836.264v-.004a1.441 1.441 0 0 0 1.327-.872l.613-1.864a2.87 2.87 0 0 1 1.817-1.812l1.778-.578a1.442 1.442 0 0 0-.052-2.74l-1.755-.57a2.877 2.877 0 0 1-1.822-1.823l-.578-1.777a1.446 1.446 0 0 0-2.732.022l-.583 1.792a2.877 2.877 0 0 1-1.77 1.786l-1.777.57a1.444 1.444 0 0 0 .017 2.735l1.754.569a2.887 2.887 0 0 1 1.822 1.826l.578 1.775c.099.283.283.527.527.7m7.667 5.047a1.123 1.123 0 0 1-.41-.55l-.328-1.006a1.292 1.292 0 0 0-.821-.823l-.991-.323a1.148 1.148 0 0 1-.781-1.083a1.142 1.142 0 0 1 .771-1.08l1.006-.326a1.3 1.3 0 0 0 .8-.82l.324-.991a1.143 1.143 0 0 1 2.157-.021l.329 1.014a1.299 1.299 0 0 0 .82.816l.992.323a1.141 1.141 0 0 1 .039 2.165l-1.014.329a1.3 1.3 0 0 0-.818.822l-.322.989c-.078.23-.226.43-.425.57a1.14 1.14 0 0 1-1.328-.005"/></svg>
    "#;
    let feature_privacy = r#"
        <svg xmlns="http://www.w3.org/2000/svg" width="60" viewBox="0 0 24 24"><path fill="currentColor" d="M4.19 4.47C3.47 4.79 3 5.51 3 6.3V11c0 5.55 3.84 10.74 9 12c5.16-1.26 9-6.45 9-12V6.3c0-.79-.47-1.51-1.19-1.83l-7-3.11c-.52-.23-1.11-.23-1.62 0zM12 7c.55 0 1 .45 1 1s-.45 1-1 1s-1-.45-1-1s.45-1 1-1m0 4c.55 0 1 .45 1 1v4c0 .55-.45 1-1 1s-1-.45-1-1v-4c0-.55.45-1 1-1"/></svg>
    "#;
    let feature_foss = r#"
        <svg xmlns="http://www.w3.org/2000/svg" width="60" viewBox="0 0 24 24"><path fill="currentColor" d="M12.001 2c5.523 0 10 4.477 10 10c0 4.13-2.504 7.676-6.077 9.201l-2.518-6.55A3 3 0 0 0 12 9a3 3 0 0 0-1.404 5.652l-2.518 6.55A10.003 10.003 0 0 1 2 12C2 6.477 6.477 2 12 2"/></svg>
    "#;
    let feature_customizable = r#"
        <svg xmlns="http://www.w3.org/2000/svg" width="60" viewBox="0 0 20 20"><path fill="currentColor" d="M18.33 3.57s.27-.8-.31-1.36c-.53-.52-1.22-.24-1.22-.24c-.61.3-5.76 3.47-7.67 5.57c-.86.96-2.06 3.79-1.09 4.82c.92.98 3.96-.17 4.79-1c2.06-2.06 5.21-7.17 5.5-7.79M1.4 17.65c2.37-1.56 1.46-3.41 3.23-4.64c.93-.65 2.22-.62 3.08.29c.63.67.8 2.57-.16 3.46c-1.57 1.45-4 1.55-6.15.89"/></svg>
    "#;
    html!(
        (header())
        main class="about-container"{
         article {

             div class="text-block" {
                h3 class="text-block-title" {"Why crabbysearch?"}
                div class="hero-text-container" {
                    p class="hero-text" {"crabbysearch aggregates results from multiple search engines and presents them in an unbiased manner, filtering out trackers and ads."}
                }
            }

            div class="feature-list" {
                h3 class="feature-list-title" {"Features"}
                div class="features" {

                    div class="feature-card" {
                        div class="feature-card-header" {
                            div class="feature-card-icon" { (PreEscaped(feature_lightning)) }
                            h4 {
                                "Lightning-fast"
                            }
                        }
                        div class="feature-card-body" {
                            p {
                                "Results load within milliseconds for an instant search experience."
                            }
                        }
                    }

                    div class="feature-card" {
                        div class="feature-card-header" {
                            div class="feature-card-icon" { (PreEscaped(feature_secure)) }
                            h4 {
                                "Secure Search"
                            }
                        }
                        div class="feature-card-body" {
                            p {
                                "All searches are performed over an encrypted connection to prevent snooping."
                            }
                        }
                    }

                    div class="feature-card" {
                        div class="feature-card-header" {
                            div class="feature-card-icon" { (PreEscaped(feature_clean)) }
                            h4 {
                                "Ad-free Results"
                            }
                        }
                        div class="feature-card-body" {
                            p {
                                "All search results are ad free and clutter free for a clean search experience."
                            }
                        }
                    }

                    div class="feature-card" {
                        div class="feature-card-header" {
                            div class="feature-card-icon" { (PreEscaped(feature_privacy)) }
                            h4 {
                                "Privacy-focused"
                            }
                        }
                        div class="feature-card-body" {
                            p {
                                "crabbysearch does not track, store or sell your search data. Your privacy is our priority."
                            }
                        }
                    }

                    div class="feature-card" {
                        div class="feature-card-header" {
                            div class="feature-card-icon" { (PreEscaped(feature_foss)) }
                            h4 {
                                "Free and Open-source"
                            }
                        }
                        div class="feature-card-body" {
                            p {
                                "The entire project's code is open source and available for free on "{a href="https://github.com/MilimTheTrueOne/crabbysearch"{"GitHub"}}"."
                            }
                        }
                    }

                    div class="feature-card" {
                        div class="feature-card-header" {
                            div class="feature-card-icon" { (PreEscaped(feature_customizable)) }
                            h4 {
                                "Highly Customizable"
                            }
                        }
                        div class="feature-card-body" {
                            p {
                                "crabbysearch comes with 9 built-in color themes and supports creating custom themes effortlessly."
                            }
                        }
                    }
                }
             }

         }

         h3 class="about-footnote" {"Developed by the "{a href="https://github.com/MilimTheTrueOne/crabbysearch"{"crabbysearch team"}}}
        }
        (footer())
    )
}
