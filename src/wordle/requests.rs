use crate::wordle::WordleAnswer;
use rustify_derive::Endpoint;

#[derive(Endpoint)]
#[endpoint(path = "/{self.date}.json", response = "WordleAnswer")]
pub struct WordleAnswerRequest {
    #[endpoint(skip)]
    pub date: String,
}
