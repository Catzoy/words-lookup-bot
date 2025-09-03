use crate::stands4::entities::{
    AbbreviationDefinition, PhraseDefinition, ToEntity, WordDefinition,
};
use crate::stands4::SynAntDefinitions;
use serde::Deserialize;
use shuttle_runtime::__internals::serde_json;

#[derive(Deserialize, Debug)]
pub struct Results<T>
where
    T: ToEntity,
{
    pub(crate) error: Option<StringMixedType>,
    pub(crate) result: Option<VecMixedType<T>>,
}

#[derive(Deserialize, Debug)]
pub struct WordResult {
    term: StringMixedType,
    definition: StringMixedType,
    #[serde(rename = "partofspeech")]
    part_of_speech: StringMixedType,
    example: StringMixedType,
}

#[derive(Deserialize, Debug)]
pub struct PhraseResult {
    term: StringMixedType,
    explanation: StringMixedType,
    example: StringMixedType,
}
#[derive(Deserialize, Debug)]
pub struct AbbreviationResult {
    definition: StringMixedType,
    category: StringMixedType,
}

#[derive(Deserialize, Debug)]
pub struct SynAntResult {
    term: StringMixedType,
    definition: StringMixedType,
    #[serde(rename = "partofspeech")]
    part_of_speech: StringMixedType,
    synonyms: VecMixedType<StringMixedType>,
    antonyms: VecMixedType<StringMixedType>,
}

#[derive(Deserialize, Debug)]
// note, this causes deserialization to try the variants top-to-bottom
#[serde(untagged)]
pub enum StringMixedType {
    String(String),
    #[allow(dead_code)]
    Other(serde_json::Value),
}

#[derive(Deserialize, Debug)]
// note, this causes deserialization to try the variants top-to-bottom
#[serde(untagged)]
pub enum VecMixedType<T>
where
    T: ToEntity,
{
    Vec(Vec<T>),
    Single(T),
    #[allow(dead_code)]
    Other(serde_json::Value),
}

impl ToEntity for WordResult {
    type Output = WordDefinition;
    fn to_entity(&self) -> Self::Output {
        WordDefinition {
            term: self.term.to_entity(),
            definition: self.definition.to_entity(),
            part_of_speech: self.part_of_speech.to_entity(),
            example: self.example.to_entity(),
        }
    }
}

impl ToEntity for PhraseResult {
    type Output = PhraseDefinition;
    fn to_entity(&self) -> Self::Output {
        PhraseDefinition {
            term: self.term.to_entity(),
            explanation: self.explanation.to_entity(),
            example: self.example.to_entity(),
        }
    }
}

impl ToEntity for AbbreviationResult {
    type Output = AbbreviationDefinition;

    fn to_entity(&self) -> Self::Output {
        AbbreviationDefinition {
            definition: self.definition.to_entity(),
            category: self.category.to_entity(),
        }
    }
}

impl ToEntity for SynAntResult {
    type Output = SynAntDefinitions;

    fn to_entity(&self) -> Self::Output {
        SynAntDefinitions {
            term: self.term.to_entity(),
            definition: self.definition.to_entity(),
            part_of_speech: self.part_of_speech.to_entity(),
            synonyms: self.synonyms.to_entity(),
            antonyms: self.antonyms.to_entity(),
        }
    }
}

impl<T> ToEntity for VecMixedType<T>
where
    T: ToEntity,
{
    type Output = Vec<T::Output>;
    fn to_entity(&self) -> Self::Output {
        match self {
            VecMixedType::Vec(vec) => vec.iter().map(ToEntity::to_entity).collect(),
            VecMixedType::Single(value) => vec![value.to_entity()],
            VecMixedType::Other(_) => Vec::default(),
        }
    }
}

impl<T> ToEntity for Results<T>
where
    T: ToEntity,
{
    type Output = anyhow::Result<Vec<T::Output>>;
    fn to_entity(&self) -> Self::Output {
        if let Some(result) = &self.result {
            Ok(result.to_entity())
        } else if let Some(error) = &self.error {
            Err(anyhow::anyhow!("{}", error.to_entity()))
        } else {
            Ok(Vec::default()) // such bullshit
        }
    }
}

impl ToEntity for StringMixedType {
    type Output = String;

    fn to_entity(&self) -> Self::Output {
        match self {
            StringMixedType::String(it) => it.into(),
            StringMixedType::Other(_) => String::default(),
        }
    }
}
