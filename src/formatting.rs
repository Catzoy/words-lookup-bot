use crate::stands4::entities::{AbbreviationDefinition, PhraseDefinition, WordDefinition};
use std::ops::Not;

pub trait LookupFormatter {
    fn visit_word(&mut self, i: usize, def: &WordDefinition);
    fn visit_phrase(&mut self, i: usize, def: &PhraseDefinition);
    fn visit_abbreviations(&mut self, i: usize, category: &str, defs: &Vec<&AbbreviationDefinition>);
    fn append_link(&mut self, link: String);

    fn build(self) -> Result<String, std::string::FromUtf8Error>;
}

pub struct FullMessageFormatter {
    pub(crate) builder: string_builder::Builder,
}

impl Default for FullMessageFormatter {
    fn default() -> Self {
        FullMessageFormatter {
            builder: string_builder::Builder::default()
        }
    }
}

impl LookupFormatter for FullMessageFormatter {
    fn visit_word(&mut self, i: usize, def: &WordDefinition) {
        let part_of_speech = match def.part_of_speech.is_empty() {
            true => &"?".to_string(),
            false => &def.part_of_speech,
        };

        self.builder.append(format!("#{} - {} ({})\n", i + 1, def.term, part_of_speech));
        self.builder.append(format!("Meaning \"{}\"\n", def.definition));
        if def.example.is_empty().not() {
            self.builder.append(format!("As in {}\n", def.example));
        }
        self.builder.append("\n");
    }

    fn visit_phrase(&mut self, i: usize, def: &PhraseDefinition) {
        self.builder.append(format!("#{} - {}\n", i + 1, def.term));
        self.builder.append(format!("Meaning \"{}\"\n", def.explanation));
        if def.example.is_empty().not() {
            self.builder.append(format!("As in \"{}\n\"", def.example));
        }
        self.builder.append("\n");
    }

    fn visit_abbreviations(&mut self, i: usize, category: &str, defs: &Vec<&AbbreviationDefinition>) {
        let category = match category.len() {
            0 => "uncategorized".to_string(),
            _ => category.to_string(),
        };

        self.builder.append(format!("#{} in [{}] stands for: ", i + 1, category));
        for def in defs.iter() {
            self.builder.append(format!("{}, ", def.definition));
        }
        self.builder.append("\n");
    }

    fn append_link(&mut self, link: String) {
        self.builder.append(format!("Check out other definitions at {}", link));
    }

    fn build(self) -> Result<String, std::string::FromUtf8Error> {
        self.builder.string()
            .map(|str| teloxide::utils::markdown::escape(&str))
    }
}