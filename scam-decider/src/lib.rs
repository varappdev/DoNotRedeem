mod reply;
mod scam_types;

use std::{
    collections::{BTreeMap, HashMap},
    str::FromStr,
};

use donotredeem_shared::Context;
pub use reply::*;
pub use scam_types::*;
use strum::VariantArray;
use systemone::{Answer, SystemOneClient};

#[derive(Debug, thiserror::Error)]
pub enum DecideError {
    #[error("SystemOne error: {0}")]
    SystemOne(#[from] systemone::SystemOneError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error("Invalid kind: {0}")]
    InvalidKind(String),
}

pub async fn decide(ctx: &Context, state: HashMap<String, String>) -> Result<Reply, DecideError> {
    let systemone_context = &ctx.systemone;

    let client = SystemOneClient::builder()
        .base_url(systemone_context.url())
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
            <&scam_types::ScamType as Into<&'static str>>::into(variant).to_owned(),
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

    let mut state_value = serde_json::to_value(state).unwrap();

    let mut first_questions = BTreeMap::new();
    first_questions.insert("is_scam".to_owned(), is_scam_question);
    first_questions.insert("kind".to_owned(), kind_question);

    let req = client.request(systemone::State::Json(state_value.clone()), first_questions);

    let first_resp = client.send(&req).await?;

    let prev_answers = serde_json::to_string(&first_resp.answers)?;

    let determined = first_resp
        .answers
        .get("kind")
        .ok_or_else(|| DecideError::InvalidKind("Missing kind answer".to_owned()))?;

    let determined_kind = if let Answer::Choice { choice, .. } = determined {
        ScamType::from_str(choice).map_err(|_| DecideError::InvalidKind(choice.clone()))?
    } else {
        return Err(DecideError::InvalidKind(
            "Kind answer is not a choice".to_owned(),
        ));
    };

    let mut signs_criteria = BTreeMap::new();
    for sign in determined_kind.telltales() {
        signs_criteria.insert(
            sign.to_string(),
            systemone::Content::Text(format!(
                "You can tell this is a {} scam because {}.",
                determined_kind, sign
            )),
        );
    }

    let signs_question = systemone::Question::Choice {
        instructions: systemone::Content::Text(
            "Determine what signs indicate this is a scam.".to_owned(),
        ),
        criteria: signs_criteria,
    };

    let mut second_questions = BTreeMap::new();
    second_questions.insert("sign".to_owned(), signs_question);

    state_value
        .as_object_mut()
        .ok_or_else(|| DecideError::InvalidKind("State is not an object".to_owned()))?
        .insert(
            "prev_answers".to_owned(),
            serde_json::Value::String(prev_answers),
        );

    let second_req = client.request(systemone::State::Json(state_value), second_questions);

    let second_resp = client.send(&second_req).await?;

    Ok(Reply {
        is_scam: first_resp
            .answers
            .get("is_scam")
            .and_then(|answer| match answer {
                Answer::Noul { noul: score, .. } => Some(*score),
                _ => None,
            })
            .ok_or_else(|| DecideError::InvalidKind("Missing is_scam answer".to_owned()))?,
        kind: determined_kind,
        sign: second_resp
            .answers
            .get("sign")
            .and_then(|answer| match answer {
                Answer::Choice { choice, .. } => Some(choice.clone()),
                _ => None,
            })
            .ok_or_else(|| DecideError::InvalidKind("Missing sign answer".to_owned()))?,
    })
}
