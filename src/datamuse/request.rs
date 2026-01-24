use crate::datamuse::responses::Word;
use rustify_derive::Endpoint;

#[derive(Endpoint)]
#[endpoint(path = "/words", response = "Vec<Word>")]
pub struct FindWordByMaskRequest {
    #[endpoint(query)]
    sp: String, // mask
}

impl FindWordByMaskRequest {
    pub fn new(mask: String) -> Self {
        Self { sp: mask }
    }
}
