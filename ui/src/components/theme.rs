use dioxus::prelude::*;

const WIN98_CSS: Asset = asset!("/assets/98.css");
const UI_CSS: Asset = asset!("/assets/ui.css");

/// Loads the stylesheets the shared components rely on.
/// Render this once near the root of every app.
#[component]
pub fn Theme() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: WIN98_CSS }
        document::Link { rel: "stylesheet", href: UI_CSS }
    }
}
