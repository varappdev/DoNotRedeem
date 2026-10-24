// live test against the real SystemOne API, one decision from three models
//
// set the api key, then run it explicitly:
//   SYSTEMONE_API_KEY=sk-or-... cargo test -p scam-decider --test live -- --ignored --nocapture
//
// SYSTEMONE_URL overrides the endpoint, OPENROUTER_API_KEY works instead of
// SYSTEMONE_API_KEY

use std::collections::HashMap;

use donotredeem_shared::{Context, ContextBuilder, OpenaiContextBuilder, SystemoneContextBuilder};
use scam_decider::decide;

const MODELS: [&str; 3] = [
    "baseten/mammoth-1:free",
    "microsoft/microsoft-decision-1",
    "perplexity/pplx-decider-v1.2",
];

fn api_key() -> String {
    std::env::var("SYSTEMONE_API_KEY")
        .or_else(|_| std::env::var("OPENROUTER_API_KEY"))
        .expect("set SYSTEMONE_API_KEY (or OPENROUTER_API_KEY) to run the live test")
}

fn context() -> Context {
    let mut systemone = SystemoneContextBuilder::new()
        .url(
            std::env::var("SYSTEMONE_URL")
                .unwrap_or_else(|_| systemone::DEFAULT_BASE_URL.to_owned()),
        )
        .api_key(api_key());

    for model in MODELS {
        systemone = systemone.add_model(model);
    }

    ContextBuilder::new()
        // decide only talks to system one, so openai just needs to be present
        .openai(
            OpenaiContextBuilder::new()
                .model("unused")
                .url("http://localhost")
                .api_key("unused"),
        )
        .systemone(systemone)
        .build()
        .expect("valid context")
}

#[tokio::test]
async fn averages_the_decisions_of_three_models() {
    let state = HashMap::from([("message".to_owned(), r#"send gift card"#.to_owned())]);

    let reply = decide(&context(), state).await.expect("decision succeeds");

    dbg!(&reply);

    eprintln!("reply: {reply:#?}");

    assert!(
        (0.0..=1.0).contains(&reply.is_scam),
        "is_scam outside 0..=1: {}",
        reply.is_scam
    );
    assert!(
        reply.kind.is_scam(),
        "a blatant scam notice should not be labelled {:?}",
        reply.kind
    );
    assert!(
        reply.kind.telltales().contains(&reply.sign.as_str()),
        "sign {:?} is not a telltale of {:?}",
        reply.sign,
        reply.kind
    );
}
