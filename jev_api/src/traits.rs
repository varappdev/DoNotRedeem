use std::collections::HashMap;

pub trait NoulProvider {
    fn nouls(&self) -> (String, String);
}

pub trait ChoiceProvider {
    fn choices(&self) -> HashMap<String, String>;
}

pub trait ScoreProvider {
    fn score(&self) -> Vec<String>;
}
