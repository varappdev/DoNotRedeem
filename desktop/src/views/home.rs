use dioxus::prelude::*;
use crate::ui::Hero;

const CHECK_ICON: Asset = asset!("/assets/icons/check-0.png");
const RESTRICT_ICON: Asset = asset!("/assets/icons/restrict-1.png");

#[component]
pub fn Home() -> Element {
    let mut status = use_signal(|| true);
    
    rsx! {
        div {
            class: "field-row-stacked home",
            
            label {
                for: "text-20",
                "Enter Data"
            }
            textarea {
                rows: "8"
            }
            input {
                type: "submit"
            }

            button {
                onclick: move |_| status.toggle(),
                "Change Status"
            }
            
            if *status.read() {
                div {
                    class: "status-container",
                    img {
                        src: CHECK_ICON
                    },
                    "NOT SCAM"
                }
            } else {
                div {
                    class: "status-container",
                    img {
                        src: RESTRICT_ICON
                    },
                    "SCAM ALERT"
                }
            }
        }

    }
}
