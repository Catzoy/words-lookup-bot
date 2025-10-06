use crate::format::ToEscaped;
use crate::stands4::WordDefinition;
use serde::Deserialize;
use teloxide::utils::markdown::escape;

#[derive(Deserialize, Clone, Debug)]
pub struct WordleAnswer {
    pub(crate) solution: String,
    pub(crate) editor: String,
    pub(crate) days_since_launch: i32,
}


impl ToEscaped for WordleAnswer {
    fn to_escaped(&self) -> Self {
        Self {
            solution: escape(&self.solution),
            editor: escape(&self.editor),
            days_since_launch: self.days_since_launch,
        }
    }
}

#[derive(Clone, Debug)]
pub struct WordleDayAnswer {
    pub(crate) day: chrono::DateTime<chrono::Utc>,
    pub(crate) answer: WordleAnswer,
    pub(crate) definitions: Vec<WordDefinition>,
}


impl ToEscaped for WordleDayAnswer {
    fn to_escaped(&self) -> Self {
        Self {
            day: self.day,
            answer: self.answer.to_escaped(),
            definitions: self.definitions.to_escaped(),
        }
    }
}