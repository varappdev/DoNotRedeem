use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

// a plain string, or json. whatever.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Content {
    Text(String),
    Json(Value),
}

impl From<&str> for Content {
    fn from(value: &str) -> Self {
        Content::Text(value.to_owned())
    }
}

impl From<String> for Content {
    fn from(value: String) -> Self {
        Content::Text(value)
    }
}

impl From<Value> for Content {
    fn from(value: Value) -> Self {
        Content::Json(value)
    }
}

// the thing you want evaluated
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum State {
    Text(String),
    Json(Value),
}

impl From<&str> for State {
    fn from(value: &str) -> Self {
        State::Text(value.to_owned())
    }
}

impl From<String> for State {
    fn from(value: String) -> Self {
        State::Text(value)
    }
}

impl From<Value> for State {
    fn from(value: Value) -> Self {
        State::Json(value)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoulCriteria {
    // wire keys are literally "true" / "false"
    #[serde(rename = "true")]
    pub when_true: Content,
    #[serde(rename = "false")]
    pub when_false: Content,
}

impl NoulCriteria {
    pub fn new(when_true: impl Into<Content>, when_false: impl Into<Content>) -> Self {
        Self {
            when_true: when_true.into(),
            when_false: when_false.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Question {
    // boolean, answer is 0.0..1.0
    Noul {
        instructions: Content,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        criteria: Option<NoulCriteria>,
    },
    Choice {
        instructions: Content,
        criteria: BTreeMap<String, Content>,
    },
    // score across the criteria steps, low -> high
    Score {
        instructions: Content,
        criteria: Vec<Content>,
    },
}

impl Question {
    pub fn noul(instructions: impl Into<Content>) -> Self {
        Question::Noul {
            instructions: instructions.into(),
            criteria: None,
        }
    }

    pub fn noul_with_criteria(
        instructions: impl Into<Content>,
        when_true: impl Into<Content>,
        when_false: impl Into<Content>,
    ) -> Self {
        Question::Noul {
            instructions: instructions.into(),
            criteria: Some(NoulCriteria::new(when_true, when_false)),
        }
    }

    pub fn choice<K, C, I>(instructions: impl Into<Content>, criteria: I) -> Self
    where
        I: IntoIterator<Item = (K, C)>,
        K: Into<String>,
        C: Into<Content>,
    {
        Question::Choice {
            instructions: instructions.into(),
            criteria: criteria
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        }
    }

    pub fn score<C, I>(instructions: impl Into<Content>, criteria: I) -> Self
    where
        I: IntoIterator<Item = C>,
        C: Into<Content>,
    {
        Question::Score {
            instructions: instructions.into(),
            criteria: criteria.into_iter().map(Into::into).collect(),
        }
    }

    pub fn instructions(&self) -> &Content {
        match self {
            Question::Noul { instructions, .. }
            | Question::Choice { instructions, .. }
            | Question::Score { instructions, .. } => instructions,
        }
    }
}

pub type Questions = BTreeMap<String, Question>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemOneRequest {
    pub model: String,
    pub state: State,
    pub questions: Questions,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl SystemOneRequest {
    pub fn new(
        model: impl Into<String>,
        state: impl Into<State>,
        questions: Questions,
    ) -> Self {
        Self {
            model: model.into(),
            state: state.into(),
            questions,
            user: None,
        }
    }

    pub fn with_user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Answer {
    Noul {
        noul: f64,
    },
    Choice {
        choice: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        confidence: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        probabilities: Option<BTreeMap<String, f64>>,
    },
    Score {
        score: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        confidence: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        legend: Option<BTreeMap<String, Content>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        probabilities: Option<BTreeMap<String, f64>>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Usage {
    pub input_tokens: u64,
    pub output_tokens: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemOneResponse {
    pub model: String,
    pub answers: BTreeMap<String, Answer>,
    pub usage: Usage,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}