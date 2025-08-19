use crate::stands4::entities::{PhraseDefinition, WordDefinition};
use std::ops::Not;

pub trait LookupFormatter {
    fn visit_word(&mut self, i: usize, def: &WordDefinition);
    fn visit_phrase(&mut self, i: usize, def: &PhraseDefinition);

    fn build(self) -> Result<String, std::string::FromUtf8Error>;
}

pub struct FullMessageFormatter {
    pub(crate) builder: string_builder::Builder,
}

impl LookupFormatter for FullMessageFormatter {
    fn visit_word(&mut self, i: usize, def: &WordDefinition) {
        let part_of_speech = match def.part_of_speech.is_empty() {
            true => &"?".to_string(),
            false => &def.part_of_speech,
        };

        self.builder.append(format!("#{} - {} ({})\n", i, def.term, part_of_speech));
        self.builder.append(format!("Meaning \"{}\"\n", def.definition));
        if def.example.is_empty().not() {
            self.builder.append(format!("As in {}\n", def.example));
        }
        self.builder.append("\n");
    }

    fn visit_phrase(&mut self, i: usize, def: &PhraseDefinition) {
        self.builder.append(format!("#{} - {}\n", i, def.term));
        self.builder.append(format!("Meaning \"{}\"\n", def.explanation));
        if def.example.is_empty().not() {
            self.builder.append(format!("As in \"{}\n\"", def.example));
        }
        self.builder.append("\n");
    }

    fn build(self) -> Result<String, std::string::FromUtf8Error> {
        self.builder.string()
    }
}