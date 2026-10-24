use dioxus::prelude::*;

use ui::{Checker, Theme};

const POPUP_CSS: Asset = asset!("/assets/popup.css");

fn main() {
    dioxus::launch(App);
}

/// The extension popup. No router here: the popup is served from `/index.html`,
/// so path-based routes would never match.
#[component]
fn App() -> Element {
    rsx! {
        Theme {}
        document::Link { rel: "stylesheet", href: POPUP_CSS }
        div {
            class: "window popup-window",
            div {
                class: "title-bar",
                div { class: "title-bar-text", "Do Not Redeem" }
            }
            div {
                class: "window-body",
                Checker {}
            }
        }
    }
}
