use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Context {
    pub openai: OpenaiContext,
    pub systemone: SystemoneContext,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Error)]
#[allow(clippy::enum_variant_names)]
pub enum ContextError {
    #[error(transparent)]
    Openai(OpenaiContextError),
    #[error(transparent)]
    Systemone(SystemoneContextError),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct ContextBuilder {
    openai: Option<OpenaiContextBuilder>,
    systemone: Option<SystemoneContextBuilder>,
}

impl ContextBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn openai(mut self, openai: OpenaiContextBuilder) -> Self {
        self.openai = Some(openai);
        self
    }

    pub fn systemone(mut self, systemone: SystemoneContextBuilder) -> Self {
        self.systemone = Some(systemone);
        self
    }

    pub fn build(self) -> Result<Context, ContextError> {
        let openai = self
            .openai
            .ok_or(ContextError::Openai(OpenaiContextError::MissingModel))?
            .build()
            .map_err(ContextError::Openai)?;

        let systemone = self
            .systemone
            .ok_or(ContextError::Systemone(SystemoneContextError::MissingModel))?
            .build()
            .map_err(ContextError::Systemone)?;

        Ok(Context { openai, systemone })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OpenaiContext {
    model: String,
    url: String,
    api_key: String,
}

impl OpenaiContext {
    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Error)]
#[allow(clippy::enum_variant_names)]
pub enum OpenaiContextError {
    #[error("Missing model in OpenAI context")]
    MissingModel,
    #[error("Missing URL in OpenAI context")]
    MissingUrl,
    #[error("Missing API key in OpenAI context")]
    MissingApiKey,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct OpenaiContextBuilder {
    model: Option<String>,
    url: Option<String>,
    api_key: Option<String>,
}

impl OpenaiContextBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn build(self) -> Result<OpenaiContext, OpenaiContextError> {
        let model = self.model.ok_or(OpenaiContextError::MissingModel)?;
        let url = self.url.ok_or(OpenaiContextError::MissingUrl)?;
        let api_key = self.api_key.ok_or(OpenaiContextError::MissingApiKey)?;

        Ok(OpenaiContext {
            model,
            url,
            api_key,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SystemoneContext {
    model: String,
    url: String,
    api_key: String,
}

impl SystemoneContext {
    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Error)]
#[allow(clippy::enum_variant_names)]
pub enum SystemoneContextError {
    #[error("Missing model in Systemone context")]
    MissingModel,
    #[error("Missing URL in Systemone context")]
    MissingUrl,
    #[error("Missing API key in Systemone context")]
    MissingApiKey,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct SystemoneContextBuilder {
    model: Option<String>,
    url: Option<String>,
    api_key: Option<String>,
}

impl SystemoneContextBuilder {
    pub fn new() -> Self {
        Self {
            model: None,
            url: None,
            api_key: None,
        }
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn build(self) -> Result<SystemoneContext, SystemoneContextError> {
        let model = self.model.ok_or(SystemoneContextError::MissingModel)?;
        let url = self.url.ok_or(SystemoneContextError::MissingUrl)?;
        let api_key = self.api_key.ok_or(SystemoneContextError::MissingApiKey)?;

        Ok(SystemoneContext {
            model,
            url,
            api_key,
        })
    }
}
