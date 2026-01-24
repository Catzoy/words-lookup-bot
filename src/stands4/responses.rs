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
    /// Convert a deserialized API results container into either a vector of entities or an error.
    ///
    /// If the container carries a `result`, it is converted into a `Vec<R>` and returned as `Ok`.
    /// If it carries an `error`, the error is converted to a `String` and returned as `Err(anyhow::Error)`.
    /// If neither `result` nor `error` is present, an empty `Vec` is returned as `Ok`.
    ///
    /// # Examples
    ///
    /// ```
    /// use anyhow::Result;
    ///
    /// // Construct a Results containing a single item; `R = String`, `X = String`.
    /// let results = Results {
    ///     result: Some(VecMixedType::Single("value".to_string())),
    ///     error: None,
    /// };
    ///
    /// let converted: Result<Vec<String>> = From::from(results);
    /// assert_eq!(converted.unwrap(), vec!["value".to_string()]);
    /// ```
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
    /// Converts a `WordResult` into a `WordDefinition`.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = WordResult {
    ///     term: StringMixedType::String("word".into()),
    ///     definition: StringMixedType::String("a short definition".into()),
    ///     part_of_speech: StringMixedType::String("noun".into()),
    ///     example: StringMixedType::String("This is an example.".into()),
    /// };
    /// let def: WordDefinition = result.into();
    /// assert_eq!(def.term, "word");
    /// assert_eq!(def.part_of_speech, "noun");
    /// ```
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
    /// Creates a `PhraseDefinition` from a `PhraseResult`.
    ///
    /// Converts each `StringMixedType` field of the `PhraseResult` into a plain `String`
    /// and places them into a new `PhraseDefinition`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::{PhraseResult, PhraseDefinition, StringMixedType};
    ///
    /// let pr = PhraseResult {
    ///     term: StringMixedType::String("break the ice".into()),
    ///     explanation: StringMixedType::String("to initiate social interaction".into()),
    ///     example: StringMixedType::String("He told a joke to break the ice.".into()),
    /// };
    ///
    /// let pd: PhraseDefinition = pr.into();
    /// assert_eq!(pd.term, "break the ice");
    /// assert_eq!(pd.explanation, "to initiate social interaction");
    /// assert_eq!(pd.example, "He told a joke to break the ice.");
    /// ```
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
    /// Converts an `AbbreviationResult` into an `AbbreviationDefinition`.
    ///
    /// The resulting `AbbreviationDefinition` contains string values extracted from the
    /// `definition` and `category` fields of the source `AbbreviationResult`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::stands4::AbbreviationDefinition;
    /// use crate::stands4_client::responses::AbbreviationResult;
    /// use crate::stands4_client::responses::StringMixedType;
    ///
    /// let res = AbbreviationResult {
    ///     definition: StringMixedType::String("for example".into()),
    ///     category: StringMixedType::String("science".into()),
    /// };
    ///
    /// let def = AbbreviationDefinition::from(res);
    /// assert_eq!(def.definition, "for example");
    /// assert_eq!(def.category, "science");
    /// ```
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
    /// Converts a `SynAntResult` into a `SynAntDefinitions`.
    ///
    /// # Returns
    ///
    /// The converted `SynAntDefinitions` populated from the source `SynAntResult`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::stands4::SynAntDefinitions;
    /// // construct a SynAntResult with minimal fields (assuming public fields for the example)
    /// let src = SynAntResult {
    ///     term: StringMixedType::String("fast".into()),
    ///     definition: StringMixedType::String("moving quickly".into()),
    ///     part_of_speech: StringMixedType::String("adjective".into()),
    ///     synonyms: VecMixedType::Single(StringMixedType::String("quick".into())),
    ///     antonyms: VecMixedType::Vec(vec![StringMixedType::String("slow".into())]),
    /// };
    /// let def: SynAntDefinitions = src.into();
    /// assert_eq!(def.term, "fast");
    /// assert_eq!(def.part_of_speech, "adjective");
    /// ```
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
    /// Convert a `VecMixedType<X>` into a `Vec<R>` by converting contained elements to `R`.
    ///
    /// When the source is `VecMixedType::Vec`, each element is converted via `R::from` and collected.
    /// When the source is `VecMixedType::Single`, a single-element vector is returned containing the converted value.
    /// When the source is `VecMixedType::Other`, an empty vector is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::Value;
    ///
    /// // Vec case
    /// let src = crate::VecMixedType::Vec(vec!["a".to_string(), "b".to_string()]);
    /// let out: Vec<String> = Vec::from(src);
    /// assert_eq!(out, vec!["a".to_string(), "b".to_string()]);
    ///
    /// // Single case
    /// let src = crate::VecMixedType::Single("x".to_string());
    /// let out: Vec<String> = Vec::from(src);
    /// assert_eq!(out, vec!["x".to_string()]);
    ///
    /// // Other case
    /// let src = crate::VecMixedType::Other(Value::Null);
    /// let out: Vec<String> = Vec::from(src);
    /// assert!(out.is_empty());
    /// ```
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
    /// Convert a `StringMixedType` into a `String`.
    ///
    /// If the variant contains a string, returns that string; otherwise returns an empty string.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::json;
    ///
    /// let s = crate::StringMixedType::String("hello".into());
    /// assert_eq!(String::from(s), "hello");
    ///
    /// let s2 = crate::StringMixedType::Other(json!(null));
    /// assert_eq!(String::from(s2), "");
    /// ```
    fn from(value: StringMixedType) -> Self {
        match value {
            StringMixedType::String(it) => it.into(),
            StringMixedType::Other(_) => String::default(),
        }
    }
}