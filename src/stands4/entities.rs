pub trait ToEntity {
    type Output;
    fn to_entity(&self) -> Self::Output;
}

#[derive(Clone, Debug)]
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
    pub(crate) definition: String,
    pub(crate) category: String,
}

#[derive(Debug)]
pub struct SynAntDefinitions {
    pub(crate) term: String,
    pub(crate) definition: String,
    pub(crate) part_of_speech: String,
    pub(crate) synonyms: Vec<String>,
    pub(crate) antonyms: Vec<String>,
}
