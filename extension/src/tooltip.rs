use std::rc::Rc;

use dioxus::{core::Runtime, prelude::*};
use explainer::ExplainedReply;
use scam_decider::Verdict;
use serde::Deserialize;
use ui::{scam_context, Status};
use wasm_bindgen::{prelude::*, JsCast};

use crate::{listen_to_messages, post_to_page, scan::scan, watch_size};

const TOOLTIP_CSS: Asset = asset!("/assets/tooltip.css");

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum PageMessage {
    Element {
        id: String,
        html: String,
        text: String,
        url: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
enum CheckState {
    Idle,
    Checking,
    Done(ExplainedReply),
    Failed(String),
}

/// Rendered inside the iframe the content script places next to the hovered element.
#[component]
pub fn Tooltip(id: String) -> Element {
    let mut state = use_signal(|| CheckState::Idle);

    use_hook(move || {
        watch_size();
        let context = Rc::new(scam_context());
        let runtime = Runtime::current();
        let scope = runtime.current_scope_id();

        let cb = Closure::<dyn FnMut(String)>::new(move |msg: String| {
            let Ok(PageMessage::Element { id: from, html, text, url }) = serde_json::from_str(&msg)
            else {
                return;
            };
            if from != id || *state.peek() != CheckState::Checking {
                return;
            }

            let ctx = context.clone();
            runtime.in_scope(scope, || {
                spawn(async move {
                    match scan(&ctx, &html, text, url).await {
                        Ok(reply) => state.set(CheckState::Done(reply)),
                        Err(e) => state.set(CheckState::Failed(e.to_string())),
                    }
                });
            });
        });
        listen_to_messages(cb.as_ref().unchecked_ref());
        cb.forget();
    });

    let close = move |_| {
        state.set(CheckState::Idle);
        post_to_page("unlock");
    };

    rsx! {
        document::Link { rel: "stylesheet", href: TOOLTIP_CSS }
        div {
            class: "window tooltip-window",
            div {
                class: "title-bar",
                div { class: "title-bar-text", "Scamvoidance" }
                if *state.read() != CheckState::Idle {
                    div {
                        class: "title-bar-controls",
                        button { aria_label: "Close", onclick: close }
                    }
                }
            }
            div {
                class: "window-body",
                match state() {
                    CheckState::Idle => rsx! {
                        p { "Scam check this element?" }
                        div {
                            class: "tooltip-actions",
                            button { onclick: move |_| post_to_page("parent"), "Parent" }
                            button { onclick: move |_| post_to_page("child"), "Child" }
                            button {
                                onclick: move |_| {
                                    state.set(CheckState::Checking);
                                    post_to_page("lock");
                                },
                                "Check"
                            }
                        }
                    },
                    CheckState::Checking => rsx! {
                        p { "Checking..." }
                        div {
                            class: "progress-indicator segmented",
                            span { class: "progress-indicator-bar", style: "width: 40%" }
                        }
                    },
                    CheckState::Done(reply) => rsx! {
                        Status {
                            safe: matches!(reply.reply.score.verdict, Verdict::Safe),
                            reply: Some(reply.clone()),
                        }
                    },
                    CheckState::Failed(err) => rsx! {
                        p { "Couldn't check this element: {err}" }
                    },
                }
            }
        }
    }
}
