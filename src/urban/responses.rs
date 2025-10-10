use crate::format::ToEscaped;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UrbanResponse {
    pub status_code: i32,
    #[serde(default)]
    pub data: Vec<UrbanDefinition>,
    pub message: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UrbanDefinition {
    pub word: String,
    pub meaning: String,
    pub example: Option<String>,
}

impl ToEscaped for UrbanDefinition {
    fn to_escaped(&self) -> Self {
        Self {
            word: self.word.to_escaped(),
            meaning: self.meaning.to_escaped(),
            example: self.example.to_escaped(),
        }
    }
}
