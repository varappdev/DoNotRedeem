use std::fmt;

use serde::{Deserialize, Serialize};

use crate::ScamType;

// how dangerous the message looks, derived from the aggregated score
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    Safe,
    Caution,
    Scam,
}

impl Verdict {
    // scores below this are treated as safe
    pub const CAUTION_MIN: f64 = 0.4;
    // scores at or above this are treated as a definite scam
    pub const SCAM_MIN: f64 = 0.7;

    pub fn from_score(score: f64) -> Self {
        if score < Self::CAUTION_MIN {
            Self::Safe
        } else if score < Self::SCAM_MIN {
            Self::Caution
        } else {
            Self::Scam
        }
    }
}

impl fmt::Display for Verdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Verdict::Safe => "Safe",
            Verdict::Caution => "Caution",
            Verdict::Scam => "Scam",
        };

        f.write_str(name)
    }
}

// aggregated model judgement, 0.0 is safe and 1.0 is definitely a scam
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ScamScore {
    pub value: f64,
    pub verdict: Verdict,
}

impl ScamScore {
    pub fn new(value: f64) -> Self {
        let value = value.clamp(0.0, 1.0);

        Self {
            value,
            verdict: Verdict::from_score(value),
        }
    }
}

impl From<f64> for ScamScore {
    fn from(value: f64) -> Self {
        ScamScore::new(value)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reply {
    pub score: ScamScore,
    pub kind: ScamType,
    pub sign: String,
}

impl Reply {
    // true only for the definite scam band, caution is not a clean message
    pub fn is_scam(&self) -> bool {
        self.score.verdict == Verdict::Scam
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verdict_follows_the_score_bands() {
        assert_eq!(Verdict::from_score(0.0), Verdict::Safe);
        assert_eq!(Verdict::from_score(0.39), Verdict::Safe);
        assert_eq!(Verdict::from_score(0.4), Verdict::Caution);
        assert_eq!(Verdict::from_score(0.69), Verdict::Caution);
        assert_eq!(Verdict::from_score(0.7), Verdict::Scam);
        assert_eq!(Verdict::from_score(1.0), Verdict::Scam);
    }

    #[test]
    fn score_is_clamped_to_the_unit_range() {
        assert_eq!(ScamScore::new(-0.5).value, 0.0);
        assert_eq!(ScamScore::new(1.5).value, 1.0);
        assert_eq!(ScamScore::new(1.5).verdict, Verdict::Scam);
    }

    #[test]
    fn reply_only_calls_a_definite_scam_a_scam() {
        let reply = |value| Reply {
            score: ScamScore::new(value),
            kind: ScamType::GiftCardPayment,
            sign: "Payment is requested in gift card codes.".to_owned(),
        };

        assert!(!reply(0.2).is_scam());
        assert!(!reply(0.5).is_scam());
        assert!(reply(0.9).is_scam());
    }
}
