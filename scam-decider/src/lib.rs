mod reply;
mod scam_types;

use std::collections::{BTreeMap, HashMap};

use donotredeem_shared::Context;
pub use reply::*;
pub use scam_types::*;
use strum::VariantArray;
use systemone::SystemOneClient;

pub async fn decide(ctx: &Context, state: HashMap<String, String>) -> Reply {
    let systemone_context = &ctx.systemone;

    let client = SystemOneClient::builder()
        .api_key(systemone_context.api_key())
        .model(systemone_context.model())
        .build()
        .unwrap();

    let is_scam_criteria = systemone::NoulCriteria {
        when_true: systemone::Content::Text("This is a scam.".to_owned()),
        when_false: systemone::Content::Text("This is not a scam.".to_owned()),
    };
    let is_scam_question = systemone::Question::Noul {
        instructions: systemone::Content::Text(
            "Determine if the following is a scam or not.".to_owned(),
        ),
        criteria: Some(is_scam_criteria),
    };

    let mut kind_criteria = BTreeMap::new();

    for variant in ScamType::VARIANTS {
        kind_criteria.insert(
            variant.to_string(),
            systemone::Content::Text(format!(
                "This is a {} scam. {}",
                variant,
                variant.description()
            )),
        );
    }
    let kind_question = systemone::Question::Choice {
        instructions: systemone::Content::Text("Determine what kind of scam it is.".to_owned()),
        criteria: kind_criteria,
    };

    let state_value = serde_json::to_value(state).unwrap();

    let mut first_questions = BTreeMap::new();
    first_questions.insert("is_scam".to_owned(), is_scam_question);
    first_questions.insert("kind".to_owned(), kind_question);

    client.request(systemone::State::Json(state_value), first_questions);

    todo!()
}
