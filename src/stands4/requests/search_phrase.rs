use crate::stands4::responses::PhraseResult;
use crate::stands4::responses::Results;
use rustify_derive::Endpoint;
use serde::Serialize;

#[derive(Endpoint, Serialize)]
#[endpoint(path = "/phrases.php", response = "Results<PhraseResult>")]
pub struct SearchPhraseRequest {
    #[endpoint(query)]
    pub phrase: String,
}
