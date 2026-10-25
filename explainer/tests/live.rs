// live test against the real SystemOne and OpenAI-compatible APIs, one explained
// decision from three models
//
// set the api key, then run it explicitly:
//   SYSTEMONE_API_KEY=sk-or-... cargo test -p explainer --test live -- --ignored --nocapture
//
// SYSTEMONE_URL overrides the SystemOne endpoint, OPENAI_URL overrides the
// OpenAI-compatible endpoint (defaults to the SystemOne one), OPENAI_API_KEY
// overrides the key used for the explanation. OPENROUTER_API_KEY works instead
// of SYSTEMONE_API_KEY.

use std::collections::HashMap;

use donotredeem_shared::{Context, ContextBuilder, OpenaiContextBuilder, SystemoneContextBuilder};
use explainer::make_explained;

const MODELS: [&str; 3] = [
    "baseten/mammoth-1:free",
    "microsoft/microsoft-decision-1",
    "perplexity/pplx-decider-v1.2",
];

const OPENAI_MODEL: &str = "xiaomi/mimo-v2.6-flash";

fn api_key() -> String {
    std::env::var("SYSTEMONE_API_KEY")
        .or_else(|_| std::env::var("OPENROUTER_API_KEY"))
        .expect("set SYSTEMONE_API_KEY (or OPENROUTER_API_KEY) to run the live test")
}

// the explanation is sent to an OpenAI-compatible endpoint, which is the same
// provider by default, so it falls back to the SystemOne key
fn openai_api_key() -> String {
    std::env::var("OPENAI_API_KEY")
        .or_else(|_| std::env::var("SYSTEMONE_API_KEY"))
        .or_else(|_| std::env::var("OPENROUTER_API_KEY"))
        .expect("set OPENAI_API_KEY (or SYSTEMONE_API_KEY) to run the live test")
}

fn context() -> Context {
    let url =
        std::env::var("SYSTEMONE_URL").unwrap_or_else(|_| systemone::DEFAULT_BASE_URL.to_owned());

    let mut systemone = SystemoneContextBuilder::new().url(&url).api_key(api_key());

    for model in MODELS {
        systemone = systemone.add_model(model);
    }

    ContextBuilder::new()
        .openai(
            OpenaiContextBuilder::new()
                .model(OPENAI_MODEL)
                .url(std::env::var("OPENAI_URL").unwrap_or_else(|_| url.clone()))
                .api_key(openai_api_key()),
        )
        .systemone(systemone)
        .build()
        .expect("valid context")
}

#[tokio::test]
// #[ignore = "hits the live SystemOne and OpenAI APIs"]
async fn explains_the_decision_of_three_models() {
    let state = HashMap::from([("message".to_owned(), r#"send gift card"#.to_owned())]);

    let explained = make_explained(&context(), state)
        .await
        .expect("explained decision succeeds");

    dbg!(&explained.reply);

    eprintln!("explanation: {}", explained.explanation);

    assert!(
        (0.0..=1.0).contains(&explained.reply.score.value),
        "score outside 0..=1: {}",
        explained.reply.score.value
    );
    assert!(
        explained.reply.kind.is_scam(),
        "a blatant scam notice should not be labelled {:?}",
        explained.reply.kind
    );
    assert!(
        explained
            .reply
            .kind
            .telltales()
            .contains(&explained.reply.sign.as_str()),
        "sign {:?} is not a telltale of {:?}",
        explained.reply.sign,
        explained.reply.kind
    );
    assert!(
        !explained.explanation.trim().is_empty(),
        "expected a non-empty explanation for {:?}",
        explained.reply
    );
}
