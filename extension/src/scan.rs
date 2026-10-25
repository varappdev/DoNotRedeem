use std::collections::HashMap;

use dioxus::prelude::*;
use scamvoidance_shared::Context;
use explainer::{make_explained, ExplainError, ExplainedReply};
use scam_decider::Verdict;
use serde::Deserialize;
use ui::{scam_context, Status};

use crate::request_page;

#[derive(Debug, Deserialize)]
struct Page {
    html: String,
    text: String,
    url: String,
}

#[derive(Debug, Clone, PartialEq)]
enum ScanState {
    Idle,
    Scanning,
    Done(ExplainedReply),
    Failed(String),
}

pub async fn scan(
    ctx: &Context,
    html: &str,
    text: String,
    url: String,
) -> Result<ExplainedReply, ExplainError> {
    let page = match parser::serialise(html, &url) {
        Ok(doc) => doc.into(),
        Err(_) => HashMap::from([
            ("content".to_owned(), text),
            ("security".to_owned(), url),
        ]),
    };
    make_explained(ctx, page).await
}

/// Scans the whole page in the active tab.
#[component]
pub fn PageScan() -> Element {
    let mut state = use_signal(|| ScanState::Idle);

    let start = move |_| {
        state.set(ScanState::Scanning);
        spawn(async move {
            let page = request_page()
                .await
                .ok()
                .and_then(|json| json.as_string())
                .and_then(|json| serde_json::from_str::<Page>(&json).ok());
            let Some(Page { html, text, url }) = page else {
                state.set(ScanState::Failed(
                    "Couldn't read this page. Try reloading it.".to_owned(),
                ));
                return;
            };
            match scan(&scam_context(), &html, text, url).await {
                Ok(reply) => state.set(ScanState::Done(reply)),
                Err(e) => state.set(ScanState::Failed(e.to_string())),
            }
        });
    };

    rsx! {
        div {
            class: "field-row-stacked page-scan",
            button {
                disabled: *state.read() == ScanState::Scanning,
                onclick: start,
                "Scan for Scam"
            }
            match state() {
                ScanState::Idle => rsx! {},
                ScanState::Scanning => rsx! { p { "Scanning page..." } },
                ScanState::Done(reply) => rsx! {
                    Status {
                        safe: matches!(reply.reply.score.verdict, Verdict::Safe),
                        reply: Some(reply.clone()),
                    }
                },
                ScanState::Failed(err) => rsx! { p { "Couldn't scan this page: {err}" } },
            }
        }
    }
}
