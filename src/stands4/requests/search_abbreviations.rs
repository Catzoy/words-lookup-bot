use crate::stands4::responses::AbbreviationResult;
use crate::stands4::responses::Results;
use rustify_derive::Endpoint;
use serde::Serialize;

#[derive(Endpoint, Serialize)]
#[endpoint(path = "/abbr.php", response = "Results<AbbreviationResult>")]
pub struct SearchAbbrsRequest {
    #[endpoint(query)]
    pub term: String,
}
