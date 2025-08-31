use crate::format::formatter::{LinkProvider, LookupFormatter};
use crate::stands4::entities::{AbbreviationDefinition, PhraseDefinition, WordDefinition};
use crate::urban::UrbanDefinition;
use std::ops::Not;

pub struct FullMessageFormatter<T: LinkProvider> {
    builder: string_builder::Builder,
    link_provider: T,
}

impl<T: LinkProvider> FullMessageFormatter<T> {
    pub fn new(link_provider: T) -> FullMessageFormatter<T> {
        FullMessageFormatter {
            builder: string_builder::Builder::default(),
            link_provider,
        }
    }
}

impl<T: LinkProvider> LookupFormatter<Result<String, std::string::FromUtf8Error>>
for FullMessageFormatter<T> {
    fn link_provider(&self) -> &dyn LinkProvider {
        &self.link_provider
    }

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
        if let Some(d1) = defs.first() {
            self.builder.append(d1.definition.as_str());

            let len = defs.len();
            if len > 1 {
                for def in defs.iter().skip(1) {
                    self.builder.append(", ");
                    self.builder.append(def.definition.as_str());
                }
            }
        }
        self.builder.append("\n");
    }

    fn visit_urban_definition(&mut self, i: usize, def: &UrbanDefinition) {
        self.builder.append(format!("#{} - {}\n", i + 1, def.word));
        self.builder.append(format!("Meaning \"{}\"\n", def.meaning));
        if let Some(example) = &def.example {
            self.builder.append(format!("As in {}\n", example));
        }
        self.builder.append("\n");
    }

    fn append_title(&mut self, title: String) {
        self.builder.append(format!("{}\n\n", title));
    }

    fn append_link(&mut self, link: String) {
        self.builder.append(format!("Check out other definitions at {}\n\n", link));
    }

    fn build(self) -> Result<String, std::string::FromUtf8Error> {
        self.builder.string()
            .map(|str| teloxide::utils::markdown::escape(&str))
    }
}