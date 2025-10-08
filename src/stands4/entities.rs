use crate::format::ToEscaped;

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
            term: self.term.to_escaped(),
            definition: self.definition.to_escaped(),
            example: self.example.to_escaped(),
            part_of_speech: self.part_of_speech.to_escaped(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PhraseDefinition {
    pub(crate) term: String,
    pub(crate) example: String,
    pub(crate) explanation: String,
}

impl ToEscaped for PhraseDefinition {
    fn to_escaped(&self) -> Self {
        Self {
            term: self.term.to_escaped(),
            example: self.example.to_escaped(),
            explanation: self.explanation.to_escaped(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AbbreviationDefinition {
    pub(crate) definition: String,
    pub(crate) category: String,
}

impl ToEscaped for AbbreviationDefinition {
    fn to_escaped(&self) -> Self {
        Self {
            definition: self.definition.to_escaped(),
            category: self.category.to_escaped(),
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
            term: self.term.to_escaped(),
            definition: self.definition.to_escaped(),
            part_of_speech: self.part_of_speech.to_escaped(),
            synonyms: self.synonyms.to_escaped(),
            antonyms: self.antonyms.to_escaped(),
        }
    }
}
