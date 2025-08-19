use crate::stands4::entities::{PhraseDefinition, ToEntity, WordDefinition};
use serde::Deserialize;
use shuttle_runtime::__internals::serde_json;
use std::fmt::Display;

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
}

#[derive(Deserialize, Debug)]
pub struct PhraseResult {
    term: StringMixedType,
    explanation: StringMixedType,
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
    #[allow(dead_code)]
    Other(serde_json::Value),
}

impl ToEntity for WordResult {
    type Output = WordDefinition;
    fn to_entity(&self) -> Self::Output {
        WordDefinition {
            term: self.term.to_string(),
            definition: self.definition.to_string(),
            part_of_speech: self.part_of_speech.to_string(),
        }
    }
}

impl ToEntity for PhraseResult {
    type Output = PhraseDefinition;
    fn to_entity(&self) -> Self::Output {
        PhraseDefinition {
            term: self.term.to_string(),
            explanation: self.explanation.to_string(),
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
            Err(anyhow::anyhow!("{}", error))
        } else {
            Err(anyhow::anyhow!("LookUp failed without an error"))
        }
    }
}

impl Display for StringMixedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            StringMixedType::String(it) => it.into(),
            StringMixedType::Other(_) => String::default(),
        };
        write!(f, "{}", str)
    }
}