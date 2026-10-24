use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum SystemOneError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("api error {status} ({code}): {message}")]
    Api {
        status: u16,
        code: i64,
        message: String,
        metadata: Option<Value>,
    },
    #[error("decode error: {0}")]
    Decode(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Deserialize)]
pub struct ErrorBody {
    pub error: ErrorDetail,
    #[serde(default)]
    pub openrouter_metadata: Option<Value>,
    #[serde(default)]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ErrorDetail {
    #[serde(default)]
    pub code: Option<i64>,
    pub message: String,
    #[serde(default)]
    pub metadata: Option<Value>,
}