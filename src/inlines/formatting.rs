use crate::format::formatter::{LinkProvider, LookupFormatter};
use crate::stands4::{AbbreviationDefinition, PhraseDefinition, WordDefinition};
use crate::urban::UrbanDefinition;
use teloxide::types::{InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText, ParseMode};

struct InlineAnswer {
    title: String,
    meaning: String,
    description: Option<String>,
}
pub struct InlineFormatter<T: LinkProvider> {
    answers: Vec<InlineAnswer>,
    link_provider: T,
}

impl<T: LinkProvider> InlineFormatter<T> {
    pub fn new(link_provider: T) -> Self {
        InlineFormatter {
            answers: Vec::new(),
            link_provider,
        }
    }
}

impl<T: LinkProvider> LookupFormatter<Result<Vec<InlineQueryResult>, std::string::FromUtf8Error>> for InlineFormatter<T> {
    fn link_provider(&self) -> &dyn LinkProvider {
        &self.link_provider
    }

    fn visit_word(&mut self, i: usize, def: &WordDefinition) {
        let part_of_speech = match def.part_of_speech.is_empty() {
            true => &"?".to_string(),
            false => &def.part_of_speech,
        };

        let answer = InlineAnswer {
            title: format!("#{} - {} ({})", i + 1, def.term, part_of_speech),
            meaning: format!("Meaning \"{}\"", def.definition),
            description: match def.example.is_empty() {
                true => None,
                false => Some(format!("As in {}", def.example)),
            },
        };
        self.answers.push(answer);
    }

    fn visit_phrase(&mut self, i: usize, def: &PhraseDefinition) {
        let answer = InlineAnswer {
            title: format!("#{} - {}", i + 1, def.term),
            meaning: format!("Meaning \"{}\"", def.explanation),
            description: match def.example.is_empty() {
                true => None,
                false => Some(format!("As in {}", def.example)),
            },
        };
        self.answers.push(answer);
    }

    fn visit_abbreviations(&mut self, i: usize, category: &str, defs: &Vec<&AbbreviationDefinition>) {
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
            title: format!("#{} in [{}] stands for: ", i + 1, category),
            meaning: meaning.string().unwrap_or_else(|_|
                "Cannot describe, try this word in bot's chat".to_string()
            ),
            description: None,
        };
        self.answers.push(answer);
    }

    fn visit_urban_definition(&mut self, i: usize, def: &UrbanDefinition) {
        let answer = InlineAnswer {
            title: format!("#{} - {}", i + 1, def.word),
            meaning: format!("Meaning \"{}\"", def.meaning),
            description: def.example.as_ref()
                .map(|e| format!("As in \"{}\"", e)),
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
            let article = InlineQueryResultArticle::new(
                format!("answer-{}", i),
                answer.title,
                InputMessageContent::Text(
                    InputMessageContentText::new(
                        teloxide::utils::markdown::escape(full_text.as_str())
                    ).parse_mode(ParseMode::MarkdownV2)
                ),
            ).description(
                answer.meaning.as_str()
            );
            articles.push(InlineQueryResult::Article(article));
        }
        Ok(articles)
    }
}