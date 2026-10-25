use std::collections::HashMap;

use async_openai::{config::OpenAIConfig, types::responses::CreateResponseArgs};
use donotredeem_shared::Context;
use scam_decider::{Reply, Verdict};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Explanation {
    pub headline: String,
    pub summary: String,
    pub red_flags: Vec<String>,
    pub what_to_do: Vec<String>,
}

impl Explanation {
    fn parse(text: &str, reply: &Reply) -> Self {
        let json = match (text.find('{'), text.rfind('}')) {
            (Some(start), Some(end)) if start < end => &text[start..=end],
            _ => text,
        };

        serde_json::from_str(json).unwrap_or_else(|_| Self {
            headline: match reply.score.verdict {
                Verdict::Safe => "Looks safe".to_owned(),
                Verdict::Caution => format!("Be careful, this could be {}", reply.kind),
                Verdict::Scam => format!("{} detected", reply.kind),
            },
            summary: text.trim().to_owned(),
            red_flags: match reply.score.verdict {
                Verdict::Safe => Vec::new(),
                _ => vec![reply.sign.clone()],
            },
            what_to_do: Vec::new(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExplainedReply {
    pub reply: Reply,
    pub explanation: Explanation,
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
            "The following state was classified as {} ({}) with a score of {}, the main sign was \"{}\". \
            Explain it to a non-technical user in plain language a 12 year old could follow. Respond with only \
            a JSON object, no markdown, in this shape:\n\
            {{\"headline\": \"a few words naming the problem\", \"summary\": \"one or two sentences\", \
            \"red_flags\": [\"short warning sign quoting the suspicious text or link\"], \
            \"what_to_do\": [\"short instruction\"]}}\n\
            Use at most three red flags and three instructions. Do not mention the programmatic score or type. \
            If it is not a scam, use the headline \"Looks safe\", a one line summary and empty lists:\n\n{}",
            reply.score.verdict,
            reply.kind,
            reply.score.value,
            reply.sign,
            serde_json::to_string_pretty(&state)?
        ))
        .max_output_tokens(1024u32)
        .build()?;

    let response = client.responses().create(request).await?;

    let text = response.output_text().ok_or(ExplainError::NoOutputText)?;
    let explanation = Explanation::parse(&text, &reply);

    Ok(ExplainedReply { reply, explanation })
}

#[cfg(test)]
mod tests {
    use scam_decider::{ScamScore, ScamType};

    use super::*;

    fn reply(value: f64) -> Reply {
        Reply {
            score: ScamScore::new(value),
            kind: ScamType::GiftCardPayment,
            sign: "Payment is requested in gift card codes.".to_owned(),
        }
    }

    #[test]
    fn parses_json_wrapped_in_markdown() {
        let text = "```json\n{\"headline\": \"Gift card scam\", \"summary\": \"They want gift cards.\", \
            \"red_flags\": [\"Asks for gift card codes\"], \"what_to_do\": [\"Do not reply\"]}\n```";

        let explanation = Explanation::parse(text, &reply(0.9));

        assert_eq!(explanation.headline, "Gift card scam");
        assert_eq!(explanation.red_flags, vec!["Asks for gift card codes"]);
        assert_eq!(explanation.what_to_do, vec!["Do not reply"]);
    }

    #[test]
    fn falls_back_to_the_raw_text() {
        let explanation = Explanation::parse("This looks like a gift card scam.", &reply(0.9));

        assert_eq!(explanation.summary, "This looks like a gift card scam.");
        assert_eq!(explanation.red_flags, vec!["Payment is requested in gift card codes."]);
    }

    #[test]
    fn safe_fallback_has_no_red_flags() {
        let explanation = Explanation::parse("Looks fine.", &reply(0.1));

        assert_eq!(explanation.headline, "Looks safe");
        assert!(explanation.red_flags.is_empty());
    }
}
