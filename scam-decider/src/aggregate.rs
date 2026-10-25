use std::collections::BTreeMap;

use systemone::{Answer, SystemOneResponse};

use crate::DecideError;

// a choice plus the distribution it was picked from, averaged over every model
pub(crate) struct AveragedChoice {
    pub choice: String,
    pub probabilities: BTreeMap<String, f64>,
}

// mean of the per-model scores, which run 0.0..1.0
pub(crate) fn mean_score(
    responses: &[SystemOneResponse],
    question: &str,
) -> Result<f64, DecideError> {
    if responses.is_empty() {
        return Err(DecideError::NoModels);
    }

    let mut total = 0.0;
    for response in responses {
        match response.answers.get(question) {
            Some(Answer::Score { score, .. }) => total += score,
            _ => return Err(DecideError::UnexpectedAnswer(question.to_owned())),
        }
    }

    Ok(total / responses.len() as f64)
}

// averages the choice distributions of every model and keeps the winner
//
// a model that reports probabilities votes with that distribution, a model that
// only reports a choice votes with its whole weight on that one choice
pub(crate) fn average_choice(
    responses: &[SystemOneResponse],
    question: &str,
) -> Result<AveragedChoice, DecideError> {
    if responses.is_empty() {
        return Err(DecideError::NoModels);
    }

    let mut averaged: BTreeMap<String, f64> = BTreeMap::new();
    for response in responses {
        let Some(Answer::Choice {
            choice,
            probabilities,
            ..
        }) = response.answers.get(question)
        else {
            return Err(DecideError::UnexpectedAnswer(question.to_owned()));
        };

        match probabilities {
            Some(probabilities) => {
                for (name, probability) in probabilities {
                    *averaged.entry(name.clone()).or_default() += probability;
                }
            }
            None => *averaged.entry(choice.clone()).or_default() += 1.0,
        }
    }

    let count = responses.len() as f64;
    for probability in averaged.values_mut() {
        *probability /= count;
    }

    // ties go to the alphabetically first name so the result is deterministic
    let choice = averaged
        .iter()
        .max_by(|(a_name, a), (b_name, b)| a.total_cmp(b).then_with(|| b_name.cmp(a_name)))
        .map(|(name, _)| name.clone())
        .ok_or_else(|| DecideError::UnexpectedAnswer(question.to_owned()))?;

    Ok(AveragedChoice {
        choice,
        probabilities: averaged,
    })
}

#[cfg(test)]
mod tests {
    use systemone::Usage;

    use super::*;
    use crate::DecideError;

    fn response(answers: impl IntoIterator<Item = (&'static str, Answer)>) -> SystemOneResponse {
        SystemOneResponse {
            model: "test".to_owned(),
            answers: answers
                .into_iter()
                .map(|(question, answer)| (question.to_owned(), answer))
                .collect(),
            usage: Usage {
                input_tokens: 0,
                output_tokens: 0,
                cost: None,
            },
            id: None,
            provider: None,
        }
    }

    fn score(value: f64) -> SystemOneResponse {
        response([(
            "score",
            Answer::Score {
                score: value,
                confidence: None,
                legend: None,
                probabilities: None,
            },
        )])
    }

    fn choice(choice: &str, probabilities: Option<BTreeMap<String, f64>>) -> Answer {
        Answer::Choice {
            choice: choice.to_owned(),
            confidence: None,
            probabilities,
        }
    }

    fn distribution(entries: &[(&str, f64)]) -> BTreeMap<String, f64> {
        entries
            .iter()
            .map(|(name, probability)| ((*name).to_owned(), *probability))
            .collect()
    }

    #[test]
    fn mean_score_averages_every_model() {
        let responses = [score(0.9), score(0.5), score(0.7)];
        let mean = mean_score(&responses, "score").unwrap();
        assert!((mean - 0.7).abs() < f64::EPSILON);
    }

    #[test]
    fn mean_score_rejects_other_answers() {
        let responses = [response([("score", choice("scam", None))])];
        assert!(matches!(
            mean_score(&responses, "score"),
            Err(DecideError::UnexpectedAnswer(question)) if question == "score"
        ));
    }

    #[test]
    fn average_choice_picks_the_highest_mean_probability() {
        let responses = [
            response([(
                "kind",
                choice(
                    "advance",
                    Some(distribution(&[("advance", 0.7), ("romance", 0.3)])),
                ),
            )]),
            response([(
                "kind",
                choice(
                    "romance",
                    Some(distribution(&[("advance", 0.2), ("romance", 0.8)])),
                ),
            )]),
        ];

        let averaged = average_choice(&responses, "kind").unwrap();

        assert_eq!(averaged.choice, "romance");
        assert!((averaged.probabilities["advance"] - 0.45).abs() < f64::EPSILON);
        assert!((averaged.probabilities["romance"] - 0.55).abs() < f64::EPSILON);
    }

    #[test]
    fn average_choice_counts_models_without_probabilities_as_one_vote() {
        let responses = [
            response([(
                "kind",
                choice(
                    "advance",
                    Some(distribution(&[("advance", 0.6), ("romance", 0.4)])),
                ),
            )]),
            response([("kind", choice("romance", None))]),
        ];

        let averaged = average_choice(&responses, "kind").unwrap();

        assert_eq!(averaged.choice, "romance");
        assert!((averaged.probabilities["advance"] - 0.3).abs() < f64::EPSILON);
        assert!((averaged.probabilities["romance"] - 0.7).abs() < f64::EPSILON);
    }

    #[test]
    fn average_choice_breaks_ties_by_name() {
        let responses = [
            response([("kind", choice("romance", None))]),
            response([("kind", choice("advance", None))]),
        ];

        let averaged = average_choice(&responses, "kind").unwrap();

        assert_eq!(averaged.choice, "advance");
    }

    #[test]
    fn average_choice_rejects_missing_answers() {
        let responses = [response([("kind", Answer::Noul { noul: 1.0 })])];
        assert!(matches!(
            average_choice(&responses, "kind"),
            Err(DecideError::UnexpectedAnswer(question)) if question == "kind"
        ));
    }
}
