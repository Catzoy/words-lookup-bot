use crate::wordle::WordleAnswer;
use chrono::{DateTime, Local};
use rustify_derive::Endpoint;

#[derive(Endpoint)]
#[endpoint(path = "/{self.date}.json", response = "WordleAnswer")]
pub struct WordleAnswerRequest {
    #[endpoint(skip)]
    date: String,
}

impl WordleAnswerRequest {
    pub(crate) fn new(date: &DateTime<Local>) -> Self {
        WordleAnswerRequest {
            date: date.format("%Y-%m-%d").to_string(),
        }
    }
}
