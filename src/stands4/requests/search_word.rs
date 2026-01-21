use crate::stands4::responses::Results;
use crate::stands4::responses::WordResult;
use rustify_derive::Endpoint;
use serde::Serialize;

#[derive(Endpoint, Serialize)]
#[endpoint(path = "/defs.php", response = "Results<WordResult>")]
pub struct SearchWordRequest {
    #[endpoint(query)]
    pub word: String,
}
