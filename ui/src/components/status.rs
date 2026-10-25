use dioxus::prelude::*;
use explainer::ExplainedReply;
use scam_decider::{ScamType, Verdict};

const CHECK_ICON: Asset = asset!("/assets/icons/check-0.png");
const RESTRICT_ICON: Asset = asset!("/assets/icons/restrict-1.png");

#[component]
pub fn Status(safe: bool, reply: ReadSignal<Option<ExplainedReply>>) -> Element {
    let label = match reply().map(|er| er.reply.score.verdict) {
        Some(Verdict::Caution) => "CAUTION",
        Some(Verdict::Scam) => "SCAM ALERT",
        Some(Verdict::Safe) => "NOT SCAM",
        None if safe => "NOT SCAM",
        None => "SCAM ALERT",
    };

    rsx! {
        div {
            class: "status",
            div {
                class: "status-container",
                if safe {
                    img { src: CHECK_ICON }
                } else {
                    img { src: RESTRICT_ICON }
                }
                p { class: "status-label", "{label}" }
            }
            if let Some(er) = reply() {
                StatusDetails { reply: er }
            }
        }
    }
}

#[component]
fn StatusDetails(reply: ExplainedReply) -> Element {
    let ExplainedReply { reply, explanation } = reply;
    let risk = (reply.score.value * 100.0).round();
    let colour = match reply.score.verdict {
        Verdict::Safe => "green",
        Verdict::Caution => "olive",
        Verdict::Scam => "maroon",
    };

    rsx! {
        div {
            class: "status-headline",
            strong { "{explanation.headline}" }
            if reply.kind != ScamType::NotAScam {
                span { class: "status-kind", "{reply.kind}" }
            }
        }
        div {
            class: "status-risk",
            label { "Scam risk: {risk}%" }
            div {
                class: "progress-indicator",
                span {
                    class: "progress-indicator-bar",
                    style: "width: {risk}%; background-color: {colour}",
                }
            }
        }
        p { class: "status-summary", "{explanation.summary}" }
        if !explanation.red_flags.is_empty() {
            fieldset {
                legend { "Warning signs" }
                ul {
                    for flag in explanation.red_flags {
                        li { "{flag}" }
                    }
                }
            }
        }
        if !explanation.what_to_do.is_empty() {
            fieldset {
                legend { "What to do" }
                ul {
                    for step in explanation.what_to_do {
                        li { "{step}" }
                    }
                }
            }
        }
    }
}
