use crate::format::{as_in, meaning, push_syn_ant};
use crate::{
    format::{LinksProvider, LookupFormatter},
    stands4::{AbbreviationDefinition, PhraseDefinition, SynAntDefinitions, WordDefinition},
    urban::UrbanDefinition,
};
use teloxide::types::{
    InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText,
    ParseMode,
};

struct InlineAnswer {
    title: String,
    meaning: String,
    description: Option<String>,
}
#[derive(Default)]
pub struct InlineFormatter {
    answers: Vec<InlineAnswer>,
    link_provider: LinksProvider,
}

impl LookupFormatter<Result<Vec<InlineQueryResult>, std::string::FromUtf8Error>>
for InlineFormatter
{
    fn link_provider(&self) -> &LinksProvider {
        &self.link_provider
    }

    fn visit_word(&mut self, i: usize, def: &WordDefinition) {
        let part_of_speech = match def.part_of_speech.is_empty() {
            true => &"?".to_string(),
            false => &def.part_of_speech,
        };

        let answer = InlineAnswer {
            title: format!("\\#{} \\- {} \\({}\\)", i + 1, def.term, part_of_speech),
            meaning: meaning(&def.definition),
            description: match def.example.is_empty() {
                true => None,
                false => Some(as_in(&def.example)),
            },
        };
        self.answers.push(answer);
    }

    fn visit_phrase(&mut self, i: usize, def: &PhraseDefinition) {
        let answer = InlineAnswer {
            title: format!("\\#{} \\- {}", i + 1, def.term),
            meaning: meaning(&def.explanation),
            description: match def.example.is_empty() {
                true => None,
                false => Some(as_in(&def.example)),
            },
        };
        self.answers.push(answer);
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
        let mut meaning = string_builder::Builder::default();
        if let Some(d1) = defs.first() {
            meaning.append(d1.definition.as_str());

            let len = defs.len();
            if len > 1 {
                for def in defs.iter().skip(1) {
                    meaning.append(", ");
                    meaning.append(def.definition.as_str());
                }
            }
        }

        let answer = InlineAnswer {
            title: format!("\\#{} in \\[{}\\] stands for: ", i + 1, category),
            meaning: meaning
                .string()
                .unwrap_or_else(|_| "Cannot describe, try this word in bot's chat".to_string()),
            description: None,
        };
        self.answers.push(answer);
    }

    fn visit_syn_ant(&mut self, i: usize, def: &SynAntDefinitions) {
        let mut description = string_builder::Builder::default();
        push_syn_ant(&mut description, def, || {
            "Surprisingly, there are no synonyms or antonyms to this!".to_string()
        });
        let answer = InlineAnswer {
            title: format!("\\#{} {} \\[{}\\]", i, def.term, def.part_of_speech),
            meaning: def.definition.clone(),
            description: description.string().ok(),
        };
        self.answers.push(answer);
    }

    fn visit_urban_definition(&mut self, i: usize, def: &UrbanDefinition) {
        let answer = InlineAnswer {
            title: format!("\\#{} \\- {}", i + 1, def.word),
            meaning: meaning(&def.meaning),
            description: def.example.as_ref().map(as_in),
        };
        self.answers.push(answer);
    }

    fn append_title(&mut self, _title: String) {
        // no support for now
    }

    fn append_link(&mut self, _link: String) {
        // no support for now
    }

    fn build(self) -> Result<Vec<InlineQueryResult>, std::string::FromUtf8Error> {
        let mut articles = Vec::new();
        for (i, answer) in self.answers.into_iter().enumerate() {
            let mut full_text = string_builder::Builder::default();
            full_text.append(answer.title.as_str());
            full_text.append("\n\n");
            full_text.append(answer.meaning.as_str());
            if let Some(description) = answer.description {
                full_text.append("\n");
                full_text.append(description);
            }
            let full_text = full_text.string()?;
            let msg_content = InputMessageContentText::new(&full_text)
                .parse_mode(ParseMode::MarkdownV2);
            let article = InlineQueryResultArticle::new(
                format!("answer-{}", i),
                answer.title,
                InputMessageContent::Text(msg_content),
            ).description(answer.meaning.as_str());
            let article = InlineQueryResult::Article(article);
            articles.push(article);
        }
        Ok(articles)
    }
}
