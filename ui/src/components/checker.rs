use std::{collections::HashMap, rc::Rc};

use dioxus::{logger::tracing, prelude::*};
use donotredeem_shared::{Context, ContextBuilder, OpenaiContextBuilder, SystemoneContextBuilder};
use explainer::{make_explained, ExplainedReply};
use scam_decider::Verdict;

use crate::Status;

// ngl wrap this in rc
pub fn scam_context() -> Context {
    let system_one = SystemoneContextBuilder::new()
        .api_key(option_env!("OPENROUTER_API_KEY").unwrap_or(""))
        .add_model("perplexity/pplx-decider-v1.2")
        .add_model("microsoft/microsoft-decision-1")
        .add_model("baseten/mammoth-1:free")
        .url("https://openrouter.ai/api/v1");
    let openai_context = OpenaiContextBuilder::new()
        .api_key(option_env!("OPENROUTER_API_KEY").unwrap_or(""))
        .model("xiaomi/mimo-v2.6-flash")
        .url("https://openrouter.ai/api/v1");

    ContextBuilder::new()
        .systemone(system_one)
        .openai(openai_context)
        .build()
        .unwrap()
}

#[component]
pub fn Checker() -> Element {
    let mut status = use_signal(|| true);
    let mut status_text = use_signal(|| Option::<ExplainedReply>::None);
    let mut text = use_signal(|| "".to_string());
    let context = Rc::new(scam_context());

    rsx! {
        div {
            class: "field-row-stacked checker",

            label {
                for: "text-20",
                "Enter Data"
            }
            textarea {
                name: "data",
                onchange: move |e| {
                        tracing::info!("{:?}", e);
                        text.set(e.value())
                },
                rows: "8"
            }
            input {
                type: "submit",
                onclick: {
                    move |_| {
                        let ctx = context.clone();
                        spawn( async move {
                            tracing::info!("Sending to explained, {}", text());
                            let explained = make_explained(&ctx, HashMap::from([("message".to_owned(), text().to_owned())])).await.unwrap();
                            tracing::info!("{:?}", explained);
                            status.set(matches!(explained.reply.score.verdict, Verdict::Safe));
                            status_text.set(Some(explained));
                        });
                    }
                }
            }

            button {
                onclick: move |_| status.toggle(),
                "Change Status"
            }

            Status { safe: status(), reply: status_text() }
        }
    }
}
