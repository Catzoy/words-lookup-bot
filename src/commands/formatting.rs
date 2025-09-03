use crate::format::formatter::LookupFormatter;
use crate::stands4::entities::{AbbreviationDefinition, PhraseDefinition, WordDefinition};
use crate::stands4::{LinksProvider, SynAntDefinitions};
use crate::urban::UrbanDefinition;
use std::ops::Not;

#[derive(Default)]
pub struct FullMessageFormatter {
    builder: string_builder::Builder,
    link_provider: LinksProvider,
}

impl FullMessageFormatter {
    fn combine_synonyms(&mut self, synonyms: &Vec<String>) {
        self.builder.append("Synonyms: ");
        self.builder.join(
            &synonyms,
            |builder, syn| builder.append(syn.to_string()),
            |builder| builder.append(", "),
        );
        self.builder.append("\n");
    }

    fn combine_antonyms(&mut self, synonyms: &Vec<String>) {
        self.builder.append("Antonyms: ");
        self.builder.join(
            &synonyms,
            |builder, ant| builder.append(ant.to_string()),
            |builder| builder.append(", "),
        );
        self.builder.append("\n");
    }
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
            .append(format!("#{} - {} ({})\n", i + 1, def.term, part_of_speech));
        self.builder
            .append(format!("Meaning \"{}\"\n", def.definition));
        if def.example.is_empty().not() {
            self.builder.append(format!("As in {}\n", def.example));
        }
        self.builder.append("\n");
    }

    fn visit_phrase(&mut self, i: usize, def: &PhraseDefinition) {
        self.builder.append(format!("#{} - {}\n", i + 1, def.term));
        self.builder
            .append(format!("Meaning \"{}\"\n", def.explanation));
        if def.example.is_empty().not() {
            self.builder.append(format!("As in \"{}\n\"", def.example));
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
            .append(format!("#{} in [{}] stands for: ", i + 1, category));
        self.builder.join(
            defs,
            |builder, def| builder.append(def.definition.as_str()),
            |builder| builder.append(", "),
        );
        self.builder.append("\n");
    }

    fn visit_syn_ant(&mut self, i: usize, def: &SynAntDefinitions) {
        self.builder.append(format!("#{} - {}\n", i + 1, def.term));
        self.builder
            .append(format!("Meaning \"{}\"\n", def.definition));
        let mut cmds: Vec<Box<dyn FnMut(&mut string_builder::Builder)>> = vec![];
        if !def.synonyms.is_empty() {
            let handler = |builder: &mut string_builder::Builder| {
                builder.append("Synonyms: ");
                builder.list_words(&def.synonyms);
                builder.append("\n");
            };
            cmds.push(Box::new(handler));
        }
        if !def.antonyms.is_empty() {
            let handler = |builder: &mut string_builder::Builder| {
                builder.append("Antonyms: ");
                builder.list_words(&def.antonyms);
                builder.append("\n");
            };
            cmds.push(Box::new(handler));
        }
        if cmds.is_empty() {
            self.builder.append(
                "Surprisingly, there are no other ways to express neither something similar, nor the opposite!"
            )
        } else {
            for mut expr in cmds {
                expr(&mut self.builder);
            }
        }
    }

    fn visit_urban_definition(&mut self, i: usize, def: &UrbanDefinition) {
        self.builder.append(format!("#{} - {}\n", i + 1, def.word));
        self.builder
            .append(format!("Meaning \"{}\"\n", def.meaning));
        if let Some(example) = &def.example {
            self.builder.append(format!("As in {}\n", example));
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
        self.builder
            .string()
            .map(|str| teloxide::utils::markdown::escape(&str))
    }
}

trait StringBuilderExt {
    fn join<T, Action, Separator>(&mut self, arr: &Vec<T>, action: Action, separator: Separator)
    where
        Action: FnMut(&mut Self, &T),
        Separator: FnMut(&mut Self);

    fn list_words(&mut self, arr: &Vec<String>);
}

impl StringBuilderExt for string_builder::Builder {
    fn join<T, Action, Separator>(
        &mut self,
        arr: &Vec<T>,
        mut action: Action,
        mut separator: Separator,
    ) where
        Action: FnMut(&mut Self, &T),
        Separator: FnMut(&mut Self),
    {
        if let Some(first) = arr.first() {
            action(self, first);
            if arr.len() > 1 {
                for item in arr.iter().skip(1) {
                    separator(self);
                    action(self, &item);
                }
            }
        }
    }

    fn list_words(&mut self, arr: &Vec<String>) {
        self.join(
            arr,
            |it, word| it.append(format!("`{}`", word)),
            |it| it.append(", "),
        )
    }
}
