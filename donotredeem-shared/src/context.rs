use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Context {
    decisions_url: String,
    openai_url: String,
    decisions_api_key: Option<String>,
    openai_api_key: Option<String>,
    both_api_key: Option<String>,
}

impl Context {
    #[inline]
    pub fn decisions_url(&self) -> &str {
        &self.decisions_url
    }

    #[inline]
    pub fn openai_url(&self) -> &str {
        &self.openai_url
    }

    #[inline]
    pub fn decisions_api_key(&self) -> &str {
        self.decisions_api_key
            .as_deref()
            .or(self.both_api_key.as_deref())
            .unwrap()
    }

    #[inline]
    pub fn openai_api_key(&self) -> &str {
        self.openai_api_key
            .as_deref()
            .or(self.both_api_key.as_deref())
            .unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Error)]
pub enum ContextError {
    #[error("Missing decisions URL")]
    MissingDecisionsUrl,
    #[error("Missing OpenAI URL")]
    MissingOpenAiUrl,
    #[error("Missing decisions API key")]
    MissingDecisionsApiKey,
    #[error("Missing OpenAI API key")]
    MissingOpenAiApiKey,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct ContextBuilder {
    decisions_url: Option<String>,
    openai_url: Option<String>,
    decisions_api_key: Option<String>,
    openai_api_key: Option<String>,
    both_api_key: Option<String>,
}

impl ContextBuilder {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn decisions_url(self, url: String) -> Self {
        Self {
            decisions_url: Some(url),
            ..self
        }
    }

    pub fn openai_url(self, url: String) -> Self {
        Self {
            openai_url: Some(url),
            ..self
        }
    }

    pub fn decisions_api_key(self, key: String) -> Self {
        Self {
            decisions_api_key: Some(key),
            ..self
        }
    }

    pub fn openai_api_key(self, key: String) -> Self {
        Self {
            openai_api_key: Some(key),
            ..self
        }
    }

    pub fn both_api_key(self, key: String) -> Self {
        Self {
            both_api_key: Some(key),
            ..self
        }
    }

    pub fn build(self) -> Result<Context, ContextError> {
        let decisions_url = self
            .decisions_url
            .ok_or(ContextError::MissingDecisionsUrl)?;
        let openai_url = self.openai_url.ok_or(ContextError::MissingOpenAiUrl)?;

        let has_decisions_api_key = self.decisions_api_key.is_some() || self.both_api_key.is_some();
        let has_openai_api_key = self.openai_api_key.is_some() || self.both_api_key.is_some();

        if !has_decisions_api_key {
            return Err(ContextError::MissingDecisionsApiKey);
        }
        if !has_openai_api_key {
            return Err(ContextError::MissingOpenAiApiKey);
        }

        Ok(Context {
            decisions_url,
            openai_url,
            decisions_api_key: self.decisions_api_key,
            openai_api_key: self.openai_api_key,
            both_api_key: self.both_api_key,
        })
    }
}
