use crate::stands4::WordDefinition;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct WordleAnswer {
    pub(crate) solution: String,
    pub(crate) editor: String,
    pub(crate) days_since_launch: i32,
}

#[derive(Clone, Debug)]
pub struct WordleDayAnswer {
    pub(crate) day: chrono::DateTime<chrono::Utc>,
    pub(crate) answer: WordleAnswer,
    pub(crate) definitions: Vec<WordDefinition>,
}

