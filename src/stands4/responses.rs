use crate::stands4::{AbbreviationDefinition, PhraseDefinition, SynAntDefinitions, WordDefinition};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Results<T> {
    pub(crate) error: Option<StringMixedType>,
    pub(crate) result: Option<VecMixedType<T>>,
}

impl<X, R> From<Results<X>> for anyhow::Result<Vec<R>>
where
    R: From<X>,
{
    fn from(value: Results<X>) -> Self {
        if let Some(result) = value.result {
            Ok(result.into())
        } else if let Some(error) = value.error {
            let str: String = error.into();
            Err(anyhow::anyhow!("{}", str))
        } else {
            Ok(Vec::default()) // such bullshit
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct WordResult {
    term: StringMixedType,
    definition: StringMixedType,
    #[serde(rename = "partofspeech")]
    part_of_speech: StringMixedType,
    example: StringMixedType,
}

impl From<WordResult> for WordDefinition {
    fn from(value: WordResult) -> Self {
        WordDefinition {
            term: value.term.into(),
            definition: value.definition.into(),
            part_of_speech: value.part_of_speech.into(),
            example: value.example.into(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct PhraseResult {
    term: StringMixedType,
    explanation: StringMixedType,
    example: StringMixedType,
}

impl From<PhraseResult> for PhraseDefinition {
    fn from(value: PhraseResult) -> Self {
        PhraseDefinition {
            term: value.term.into(),
            explanation: value.explanation.into(),
            example: value.example.into(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct AbbreviationResult {
    definition: StringMixedType,
    category: StringMixedType,
}

impl From<AbbreviationResult> for AbbreviationDefinition {
    fn from(value: AbbreviationResult) -> Self {
        AbbreviationDefinition {
            definition: value.definition.into(),
            category: value.category.into(),
        }
    }
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

impl From<SynAntResult> for SynAntDefinitions {
    fn from(value: SynAntResult) -> Self {
        SynAntDefinitions {
            term: value.term.into(),
            definition: value.definition.into(),
            part_of_speech: value.part_of_speech.into(),
            synonyms: value.synonyms.into(),
            antonyms: value.antonyms.into(),
        }
    }
}

#[derive(Deserialize, Debug)]
// note, this causes deserialization to try the variants top-to-bottom
#[serde(untagged)]
pub enum VecMixedType<T> {
    Vec(Vec<T>),
    Single(T),
    #[allow(dead_code)]
    Other(serde_json::Value),
}

impl<X, R> From<VecMixedType<X>> for Vec<R>
where
    R: From<X>,
{
    fn from(value: VecMixedType<X>) -> Self {
        match value {
            VecMixedType::Vec(vec) => vec.into_iter().map(R::from).collect(),
            VecMixedType::Single(value) => vec![value.into()],
            VecMixedType::Other(_) => Vec::default(),
        }
    }
}

#[derive(Deserialize, Debug)]
// note, this causes deserialization to try the variants top-to-bottom
#[serde(untagged)]
pub enum StringMixedType {
    String(String),
    #[allow(dead_code)]
    Other(serde_json::Value),
}

impl From<StringMixedType> for String {
    fn from(value: StringMixedType) -> Self {
        match value {
            StringMixedType::String(it) => it.into(),
            StringMixedType::Other(_) => String::default(),
        }
    }
}
