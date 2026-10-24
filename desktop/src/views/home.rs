use dioxus::prelude::*;
use ui::Checker;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "home",
            Checker {}
        }
    }
}
