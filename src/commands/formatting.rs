use crate::format::{as_in, meaning, push_syn_ant};
use crate::{
    format::{LinksProvider, LookupFormatter, StringBuilderExt},
    stands4::entities::{AbbreviationDefinition, PhraseDefinition, WordDefinition},
    stands4::SynAntDefinitions,
    urban::UrbanDefinition,
};
use std::ops::Not;

#[derive(Default)]
pub struct FullMessageFormatter {
    builder: string_builder::Builder,
    link_provider: LinksProvider,
}

impl LookupFormatter<Result<String, std::string::FromUtf8Error>> for FullMessageFormatter {
    fn link_provider(&self) -> &LinksProvider {
        &self.link_provider
    }

    fn visit_word(&mut self, i: usize, def: &WordDefinition) {
        let part_of_speech = match def.part_of_speech.is_empty() {
            true => &"?".to_string(),
            false => &def.part_of_speech,
        };

        self.builder
            .append(format!("\\#{} \\- {} \\({}\\)\n", i + 1, def.term, part_of_speech));
        self.builder.appendl(meaning(&def.definition));
        if def.example.is_empty().not() {
            self.builder.appendl(as_in(&def.example));
        }
        self.builder.append("\n");
    }

    fn visit_phrase(&mut self, i: usize, def: &PhraseDefinition) {
        self.builder.append(format!("\\#{} \\- {}\n", i + 1, def.term));
        self.builder.appendl(meaning(&def.explanation));
        if def.example.is_empty().not() {
            self.builder.appendl(as_in(&def.example));
        }
        self.builder.append("\n");
    }

    fn visit_abbreviations(
        &mut self,
        i: usize,
        category: &str,
        defs: &Vec<&AbbreviationDefinition>,
    ) {
        let category = match category.len() {
            0 => "uncategorized".to_string(),
            _ => category.to_string(),
        };

        self.builder
            .append(format!("\\#{} in \\[{}\\] stands for: ", i + 1, category));
        self.builder.join(
            defs,
            |builder, def| builder.append(def.definition.as_str()),
            |builder| builder.append(", "),
        );
        self.builder.append("\n");
    }

    fn visit_syn_ant(&mut self, i: usize, def: &SynAntDefinitions) {
        self.builder.append(format!("\\#{} \\- {}\n", i + 1, def.term));
        self.builder.appendl(meaning(&def.definition));
        push_syn_ant(&mut self.builder, def, || {
            "Surprisingly, there are no other ways to express neither something similar, nor the opposite!".to_string()
        });
    }

    fn visit_urban_definition(&mut self, i: usize, def: &UrbanDefinition) {
        self.builder.append(format!("\\#{} \\- {}\n", i + 1, def.word));
        self.builder.appendl(meaning(&def.meaning));
        if let Some(example) = &def.example {
            self.builder.appendl(as_in(&example));
        }
        self.builder.append("\n");
    }

    fn append_title(&mut self, title: String) {
        self.builder.append(format!("{}\n\n", title));
    }

    fn append_link(&mut self, link: String) {
        self.builder
            .append(format!("Check out other definitions at {}\n\n", link));
    }

    fn build(self) -> Result<String, std::string::FromUtf8Error> {
        self.builder.string()
    }
}
