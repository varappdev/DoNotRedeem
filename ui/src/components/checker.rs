use dioxus::prelude::*;

use crate::Status;

#[component]
pub fn Checker() -> Element {
    let mut status = use_signal(|| true);

    rsx! {
        div {
            class: "field-row-stacked checker",

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

            Status { safe: status() }
        }
    }
}
