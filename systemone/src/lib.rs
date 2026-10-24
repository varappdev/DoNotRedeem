mod error;
mod types;

pub use error::{ErrorBody, ErrorDetail, SystemOneError};
pub use types::{
    Answer, Content, NoulCriteria, Question, Questions, State, SystemOneRequest, SystemOneResponse,
    Usage,
};

use std::time::Duration;

// openrouter, but point this anywhere that speaks the same api
pub const DEFAULT_BASE_URL: &str = "https://openrouter.ai/api/v1";

#[derive(Debug, Clone)]
pub struct SystemOneClient {
    http: reqwest::Client,
    base_url: String,
    model: String,
    api_key: String,
    user: Option<String>,
    headers: Vec<(String, String)>,
}

impl SystemOneClient {
    pub fn new(
        api_key: impl Into<String>,
        model: impl Into<String>,
    ) -> Result<SystemOneClient, BuildError> {
        SystemOneClientBuilder::new()
            .api_key(api_key)
            .model(model)
            .build()
    }

    pub fn builder() -> SystemOneClientBuilder {
        SystemOneClientBuilder::new()
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn endpoint(&self) -> String {
        format!("{}/systemone", self.base_url)
    }

    pub fn http(&self) -> &reqwest::Client {
        &self.http
    }

    // fills in the client's model/user
    pub fn request(
        &self,
        state: impl Into<State>,
        questions: impl IntoQuestions,
    ) -> SystemOneRequest {
        let mut request =
            SystemOneRequest::new(self.model.clone(), state, questions.into_questions());
        if request.user.is_none() {
            request.user = self.user.clone();
        }
        request
    }

    pub async fn submit(
        &self,
        state: impl Into<State>,
        questions: impl IntoQuestions,
    ) -> Result<SystemOneResponse, SystemOneError> {
        self.send(&self.request(state, questions)).await
    }

    // empty request.model falls back to the client default
    pub async fn send(
        &self,
        request: &SystemOneRequest,
    ) -> Result<SystemOneResponse, SystemOneError> {
        let mut request = request.clone();
        if request.model.is_empty() {
            request.model = self.model.clone();
        }

        let mut call = self
            .http
            .post(self.endpoint())
            .bearer_auth(&self.api_key)
            .json(&request);

        for (name, value) in &self.headers {
            if let (Ok(name), Ok(value)) = (
                reqwest::header::HeaderName::from_bytes(name.as_bytes()),
                reqwest::header::HeaderValue::from_str(value),
            ) {
                call = call.header(name, value);
            }
        }

        let response = call.send().await?;
        let status = response.status();
        let body = response.text().await?;

        if !status.is_success() {
            return Err(decode_error(status, &body));
        }

        Ok(serde_json::from_str(&body)?)
    }
}

fn decode_error(status: reqwest::StatusCode, body: &str) -> SystemOneError {
    match serde_json::from_str::<ErrorBody>(body) {
        Ok(parsed) => SystemOneError::Api {
            status: status.as_u16(),
            code: parsed.error.code.unwrap_or_else(|| i64::from(status.as_u16())),
            message: parsed.error.message,
            metadata: parsed.error.metadata,
        },
        Err(_) => SystemOneError::Api {
            status: status.as_u16(),
            code: i64::from(status.as_u16()),
            message: body.trim().to_owned(),
            metadata: None,
        },
    }
}

// maps/vecs/arrays of (key, question) all work
pub trait IntoQuestions {
    fn into_questions(self) -> Questions;
}

impl IntoQuestions for Questions {
    fn into_questions(self) -> Questions {
        self
    }
}

impl<K: Into<String>> IntoQuestions for Vec<(K, Question)> {
    fn into_questions(self) -> Questions {
        self.into_iter()
            .map(|(key, question)| (key.into(), question))
            .collect()
    }
}

impl<K: Into<String>, const N: usize> IntoQuestions for [(K, Question); N] {
    fn into_questions(self) -> Questions {
        self.into_iter()
            .map(|(key, question)| (key.into(), question))
            .collect()
    }
}

#[derive(Debug, Clone, Default)]
pub struct SystemOneClientBuilder {
    base_url: Option<String>,
    model: Option<String>,
    api_key: Option<String>,
    user: Option<String>,
    timeout: Option<Duration>,
    http_client: Option<reqwest::Client>,
    headers: Vec<(String, String)>,
}

impl SystemOneClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http_client = Some(client);
        self
    }

    // extra header on every request, bad names/values just get dropped
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    pub fn build(self) -> Result<SystemOneClient, BuildError> {
        let api_key = self.api_key.ok_or(BuildError::MissingApiKey)?;
        let model = self.model.ok_or(BuildError::MissingModel)?;

        let http = match self.http_client {
            Some(client) => client,
            None => {
                let mut http = reqwest::Client::builder();
                if let Some(timeout) = self.timeout {
                    http = http.timeout(timeout);
                }
                http.build()?
            }
        };

        Ok(SystemOneClient {
            http,
            base_url: self
                .base_url
                .unwrap_or_else(|| DEFAULT_BASE_URL.to_owned())
                .trim_end_matches('/')
                .to_owned(),
            model,
            api_key,
            user: self.user,
            headers: self.headers,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    #[error("missing api key")]
    MissingApiKey,
    #[error("missing model")]
    MissingModel,
    #[error("http client build failed: {0}")]
    Http(#[from] reqwest::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let client = SystemOneClient::builder()
            .base_url("http://localhost:1234/api/v1/")
            .api_key("k")
            .model("jev-latest")
            .build()
            .unwrap();
        assert_eq!(client.endpoint(), "http://localhost:1234/api/v1/systemone");

        let request = client.request("some state", [("q", Question::noul("bug?"))]);
        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["model"], "jev-latest");
        assert_eq!(json["questions"]["q"]["type"], "noul");
    }
}