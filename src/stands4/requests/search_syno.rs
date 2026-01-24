use crate::stands4::responses::Results;
use crate::stands4::responses::SynAntResult;
use rustify_derive::Endpoint;
use serde::Serialize;

#[derive(Endpoint, Serialize)]
#[endpoint(path = "/syno.php", response = "Results<SynAntResult>")]
pub struct SearchSynoRequest {
    #[endpoint(query)]
    pub word: String,
}
