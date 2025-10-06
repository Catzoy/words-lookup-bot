use crate::format::ToEscaped;
use teloxide::utils::markdown::escape;

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

impl ToEscaped for WordDefinition {
    fn to_escaped(&self) -> Self {
        Self {
            term: escape(&self.term),
            definition: escape(&self.definition),
            example: escape(&self.example),
            part_of_speech: escape(&self.part_of_speech),
        }
    }
}

#[derive(Debug)]
pub struct PhraseDefinition {
    pub(crate) term: String,
    pub(crate) example: String,
    pub(crate) explanation: String,
}

impl ToEscaped for PhraseDefinition {
    fn to_escaped(&self) -> Self {
        Self {
            term: escape(&self.term),
            example: escape(&self.example),
            explanation: escape(&self.explanation),
        }
    }
}

#[derive(Debug)]
pub struct AbbreviationDefinition {
    pub(crate) definition: String,
    pub(crate) category: String,
}

impl ToEscaped for AbbreviationDefinition {
    fn to_escaped(&self) -> Self {
        Self {
            definition: escape(&self.definition),
            category: escape(&self.category),
        }
    }
}

#[derive(Debug)]
pub struct SynAntDefinitions {
    pub(crate) term: String,
    pub(crate) definition: String,
    pub(crate) part_of_speech: String,
    pub(crate) synonyms: Vec<String>,
    pub(crate) antonyms: Vec<String>,
}

impl ToEscaped for SynAntDefinitions {
    fn to_escaped(&self) -> Self {
        Self {
            term: escape(&self.term),
            definition: escape(&self.definition),
            part_of_speech: escape(&self.part_of_speech),
            synonyms: self.synonyms.iter().map(|i| escape(i)).collect(),
            antonyms: self.antonyms.iter().map(|i| escape(i)).collect(),
        }
    }
}
