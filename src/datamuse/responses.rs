use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Word {
    pub word: String,
}
