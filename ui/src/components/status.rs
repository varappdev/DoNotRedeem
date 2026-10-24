use dioxus::prelude::*;

const CHECK_ICON: Asset = asset!("/assets/icons/check-0.png");
const RESTRICT_ICON: Asset = asset!("/assets/icons/restrict-1.png");

#[component]
pub fn Status(safe: bool) -> Element {
    rsx! {
        div {
            class: "status-container",
            if safe {
                img { src: CHECK_ICON }
                "NOT SCAM"
            } else {
                img { src: RESTRICT_ICON }
                "SCAM ALERT"
            }
        }
    }
}
