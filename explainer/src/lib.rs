use std::collections::HashMap;

use async_openai::{config::OpenAIConfig, types::responses::CreateResponseArgs};
use donotredeem_shared::Context;
use scam_decider::Reply;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExplainedReply {
    pub reply: Reply,
    pub explanation: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ExplainError {
    #[error(transparent)]
    DecideError(#[from] scam_decider::DecideError),
    #[error(transparent)]
    OpenAIError(#[from] async_openai::error::OpenAIError),
    #[error("No output text from OpenAI response")]
    NoOutputText,
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

pub async fn make_explained(
    ctx: &Context,
    state: HashMap<String, String>,
) -> Result<ExplainedReply, ExplainError> {
    let reply = scam_decider::decide(ctx, state.clone()).await?;

    let client_cfg = OpenAIConfig::new()
        .with_api_key(ctx.openai.api_key())
        .with_api_base(ctx.openai.url());

    let client = async_openai::Client::with_config(client_cfg);

    let request = CreateResponseArgs::default()
        .model(ctx.openai.model())
        .input(format!(
            "Explain why the following state was classified as {:?} with a score of {} as a short \
            paragraph as to why, what kind of scam it is and how to tell. Do not explicitly mention the programmatic\
            score or type this is user facing. If it's identified as not a scam just say it looks safe:\n\n{}",
            reply.kind,
            reply.score.value,
            serde_json::to_string_pretty(&state)?
        ))
        .max_output_tokens(1024u32)
        .build()?;

    let response = client.responses().create(request).await?;

    let explanation = response.output_text().ok_or(ExplainError::NoOutputText)?;

    Ok(ExplainedReply { reply, explanation })
}
