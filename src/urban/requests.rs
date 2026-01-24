use crate::urban::UrbanResponse;
use rustify_derive::Endpoint;

#[derive(Endpoint)]
#[endpoint(path = "/search", response = "UrbanResponse")]
pub struct SearchUrbanRequest {
    #[endpoint(query)]
    pub(crate) term: String,
}
