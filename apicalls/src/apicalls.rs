/*
 * API call
 * Provided data: Scam_likeliness, Scam_Type, Scam_Reasons, .env data
 * Convert into text to pass into API call
 * AI produces output
 * Return output
 *
 * Deprecated: I was too tired to continue; since been lapsed by a complete version by Victor
 */

use std::collections::HashMap;

use async_openai::{Client, config::OpenAIConfig, types::responses::CreateResponseArgs};
use dotenv::EnvLoader;

pub struct Dummy {
    Scam_likeliness: f32,
    Scam_Type: HashMap<String, f32>,
    Scam_Reasons: HashMap<String, f32>,
}

pub fn parse_string(
    Dummy {
        Scam_likeliness,
        Scam_Type,
        Scam_Reasons,
    }: Dummy,
) -> (String, String, String) {
    let likely = Scam_likeliness.to_string();
    let mut typ = String::from_iter(Scam_Type.iter().map(|e| format!("{}: {}", e.0, e.1)));
    let mut reas = String::from_iter(Scam_Reasons.iter().map(|e| format!("{}: {}", e.0, e.1)));
    (likely, typ, reas)
}

pub async fn api_call(set: (String, String, String)) -> Result<()> {
    let (likely, typ, reas) = set;
    let client = Client::new();

    let config = OpenAIConfig::default();
    let client = Client::with_config(config);

    let env = EnvLoader::new().load()?;
    let api_key = format!("{:?}", env.var("OPENAI_API_KEY"));
    let config = OpenAIConfig::new().with_api_key(api_key);
    let request = CreateResponseArgs::default()
        .model("nvidia/neomotron-3.5-super-v1:free")
        .input("test")
        .max_output_tokens(512u32)
        .build()?;

    let response = client.responses().create(request).await?;

    Ok(())
}
