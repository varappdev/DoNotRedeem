use dioxus::prelude::*;

/// A navigation bar. Platforms pass in their own `Link`s so each can use its own `Route` enum.
#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        div { class: "navbar", {children} }
    }
}
