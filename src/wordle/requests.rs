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
    /// Creates a `WordleAnswerRequest` for the provided local date.
    ///
    /// The request embeds the given `date` formatted as `YYYY-MM-DD`.
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::Local;
    /// let req = crate::wordle::requests::WordleAnswerRequest::new(&Local::now());
    /// let _ = req; // request ready to use with the endpoint
    /// ```
    pub(crate) fn new(date: &DateTime<Local>) -> Self {
        WordleAnswerRequest {
            date: date.format("%Y-%m-%d").to_string(),
        }
    }
}