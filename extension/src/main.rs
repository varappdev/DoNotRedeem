mod scan;
mod tooltip;

use dioxus::prelude::*;

use ui::{Checker, Theme};
use scan::PageScan;
use tooltip::Tooltip;
use wasm_bindgen::prelude::*;

const POPUP_CSS: Asset = asset!("/assets/popup.css");

#[wasm_bindgen(module = "/bridge.js")]
extern "C" {
    #[wasm_bindgen(js_name = listenToExtensionMessages)]
    fn listen_to_messages(callback: &js_sys::Function);

    #[wasm_bindgen(js_name = locationHash)]
    fn location_hash() -> String;

    #[wasm_bindgen(js_name = postToPage)]
    fn post_to_page(kind: &str);

    #[wasm_bindgen(js_name = watchSize)]
    fn watch_size();

    #[wasm_bindgen(js_name = requestPage, catch)]
    async fn request_page() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = getElementCheck)]
    async fn get_element_check() -> JsValue;

    #[wasm_bindgen(js_name = setElementCheck)]
    async fn set_element_check(enabled: bool);
}

fn main() {
    dioxus::launch(App);
}

/// The extension popup. No router here: the popup is served from `/index.html`,
/// so path-based routes would never match.
#[component]
fn App() -> Element {
    if let Some(id) = location_hash().strip_prefix("#tooltip:") {
        return rsx! {
            Theme {}
            Tooltip { id }
        };
    }

    rsx! { Popup {} }
}

#[component]
fn Popup() -> Element {
    let mut enable_tooltip = use_signal(|| false);

    use_future(move || async move {
        enable_tooltip.set(get_element_check().await.as_bool().unwrap_or(false));
    });

    rsx! {
        Theme {}
        document::Link { rel: "stylesheet", href: POPUP_CSS }
        div {
            class: "window popup-window",
            div {
                class: "title-bar",
                div { class: "title-bar-text", "Scamvoidance" }
            }
            div {
                class: "window-body",
                button {
                    onclick: move |_| async move {
                        let enabled = !enable_tooltip();
                        set_element_check(enabled).await;
                        enable_tooltip.set(enabled);
                    },
                    if enable_tooltip() { "Disable Element Check" } else { "Enable Element Check" }
                },
                PageScan {}
                // Checker {}
            }
        }
    }
}
