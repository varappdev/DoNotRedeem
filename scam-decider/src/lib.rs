mod aggregate;
mod reply;
mod scam_types;

use std::{
    collections::{BTreeMap, HashMap},
    str::FromStr,
};

use aggregate::{average_choice, mean_noul};
use donotredeem_shared::Context;
pub use reply::*;
pub use scam_types::*;
use strum::VariantArray;
use systemone::{Answer, Content, Question, Questions, State, SystemOneClient, SystemOneResponse};

#[derive(Debug, thiserror::Error)]
pub enum DecideError {
    #[error("SystemOne error: {0}")]
    SystemOne(#[from] systemone::SystemOneError),
    #[error("SystemOne client error: {0}")]
    Client(#[from] systemone::BuildError),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error("No decision making models configured")]
    NoModels,
    #[error("Invalid kind: {0}")]
    InvalidKind(String),
    #[error("Unexpected answer for '{0}'")]
    UnexpectedAnswer(String),
    #[error("State is not a JSON object")]
    StateNotObject,
}

// asks every configured model the same questions and averages their decisions
pub async fn decide(ctx: &Context, state: HashMap<String, String>) -> Result<Reply, DecideError> {
    let systemone_context = &ctx.systemone;

    if systemone_context.models().is_empty() {
        return Err(DecideError::NoModels);
    }

    let mut clients = Vec::with_capacity(systemone_context.models().len());
    for model in systemone_context.models() {
        clients.push(
            SystemOneClient::builder()
                .base_url(systemone_context.url())
                .api_key(systemone_context.api_key())
                .model(model.clone())
                .build()?,
        );
    }

    let is_scam_criteria = systemone::NoulCriteria {
        when_true: Content::Text("This is a scam.".to_owned()),
        when_false: Content::Text("This is not a scam.".to_owned()),
    };
    let is_scam_question = Question::Noul {
        instructions: Content::Text("Determine if the following is a scam or not.".to_owned()),
        criteria: Some(is_scam_criteria),
    };

    let mut kind_criteria = BTreeMap::new();

    for variant in ScamType::VARIANTS {
        kind_criteria.insert(
            <&scam_types::ScamType as Into<&'static str>>::into(variant).to_owned(),
            Content::Text(format!(
                "This is a {} scam. {}",
                variant,
                variant.description()
            )),
        );
    }
    let kind_question = Question::Choice {
        instructions: Content::Text("Determine what kind of scam it is.".to_owned()),
        criteria: kind_criteria,
    };

    let first_questions = BTreeMap::from([
        ("is_scam".to_owned(), is_scam_question),
        ("kind".to_owned(), kind_question),
    ]);

    let mut state_value = serde_json::to_value(state)?;

    let first_responses = ask_all(
        &clients,
        &State::Json(state_value.clone()),
        &first_questions,
    )
    .await?;

    let is_scam = mean_noul(&first_responses, "is_scam")?;
    let kind = average_choice(&first_responses, "kind")?;
    let determined_kind = ScamType::from_str(&kind.choice)
        .map_err(|_| DecideError::InvalidKind(kind.choice.clone()))?;

    let mut prev_answers = BTreeMap::new();
    prev_answers.insert("is_scam".to_owned(), Answer::Noul { noul: is_scam });
    prev_answers.insert(
        "kind".to_owned(),
        Answer::Choice {
            choice: <ScamType as Into<&'static str>>::into(determined_kind).to_owned(),
            confidence: None,
            probabilities: Some(kind.probabilities),
        },
    );

    let mut signs_criteria = BTreeMap::new();
    for sign in determined_kind.telltales() {
        signs_criteria.insert(
            sign.to_string(),
            Content::Text(format!(
                "You can tell this is a {} scam because {}.",
                determined_kind, sign
            )),
        );
    }

    let signs_question = Question::Choice {
        instructions: Content::Text("Determine what signs indicate this is a scam.".to_owned()),
        criteria: signs_criteria,
    };

    let second_questions = BTreeMap::from([("sign".to_owned(), signs_question)]);

    state_value
        .as_object_mut()
        .ok_or(DecideError::StateNotObject)?
        .insert(
            "prev_answers".to_owned(),
            serde_json::Value::String(serde_json::to_string(&prev_answers)?),
        );

    let second_responses = ask_all(&clients, &State::Json(state_value), &second_questions).await?;

    Ok(Reply {
        is_scam,
        kind: determined_kind,
        sign: average_choice(&second_responses, "sign")?.choice,
    })
}

// sends the same request through every model, keeping the answers in model order
async fn ask_all(
    clients: &[SystemOneClient],
    state: &State,
    questions: &Questions,
) -> Result<Vec<SystemOneResponse>, DecideError> {
    let mut responses = Vec::with_capacity(clients.len());
    for client in clients {
        let request = client.request(state.clone(), questions.clone());
        responses.push(client.send(&request).await?);
    }
    Ok(responses)
}
