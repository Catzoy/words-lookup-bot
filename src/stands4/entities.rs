pub trait ToEntity {
    type Output;
    fn to_entity(&self) -> Self::Output;
}

#[derive(Debug)]
pub struct WordDefinition {
    pub(crate) term: String,
    pub(crate) definition: String,
    pub(crate) example: String,
    pub(crate) part_of_speech: String,
}

#[derive(Debug)]
pub struct PhraseDefinition {
    pub(crate) term: String,
    pub(crate) example: String,
    pub(crate) explanation: String,
}

#[derive(Debug)]
pub struct AbbreviationDefinition {
    pub(crate) term: String,
    pub(crate) definition: String,
    pub(crate) category: String,
}