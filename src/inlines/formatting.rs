use crate::bloc::formatting::SynAntFormatterExt;
use crate::format::{as_in, meaning};
use crate::{
    format::{LinksProvider, LookupFormatter},
    stands4::{AbbreviationDefinition, PhraseDefinition, SynAntDefinitions, WordDefinition},
    urban::UrbanDefinition,
};
use teloxide::types::{
    InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText,
    ParseMode,
};
use teloxide::utils::markdown::escape;

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

impl LookupFormatter for InlineFormatter {
    type Error = std::string::FromUtf8Error;
    type Value = Vec<InlineQueryResult>;

    /// Create an empty collection of inline query results.
    ///
    /// Used when a lookup produces no answers.
    ///
    /// # Examples
    ///
    /// ```
    /// let empty = InlineFormatter::on_empty();
    /// assert!(empty.is_empty());
    /// ```
    fn on_empty() -> Self::Value {
        vec![]
    }

    /// Access the formatter's link provider.
    ///
    /// Returns a reference to the internal LinksProvider.
    ///
    /// # Examples
    ///
    /// ```
    /// let lp: &LinksProvider = formatter.link_provider();
    /// ```
    fn link_provider(&self) -> &LinksProvider {
        &self.link_provider
    }

    fn visit_word(&mut self, i: usize, def: &WordDefinition) {
        let part_of_speech = match def.part_of_speech.is_empty() {
            true => &"?".to_string(),
            false => &def.part_of_speech,
        };

        let answer = InlineAnswer {
            title: format!("#{} - {} ({})", i + 1, def.term, part_of_speech),
            meaning: def.definition.clone(),
            description: match def.example.is_empty() {
                true => None,
                false => Some(def.example.clone()),
            },
        };
        self.answers.push(answer);
    }

    fn visit_phrase(&mut self, i: usize, def: &PhraseDefinition) {
        let answer = InlineAnswer {
            title: format!("#{} - {}", i + 1, def.term),
            meaning: def.explanation.clone(),
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
            title: format!("#{} in [{}]", i + 1, category),
            meaning: meaning
                .string()
                .unwrap_or_else(|_| "Cannot describe, try this word in bot's chat".to_string()),
            description: None,
        };
        self.answers.push(answer);
    }

    fn visit_syn_ant(&mut self, i: usize, def: &SynAntDefinitions) {
        let mut description = string_builder::Builder::default();
        Self::push_syn_ant(&mut description, def, || {
            "Surprisingly, there are no synonyms or antonyms to this!".to_string()
        });
        let answer = InlineAnswer {
            title: format!("#{} {} [{}]", i, def.term, def.part_of_speech),
            meaning: def.definition.clone(),
            description: description.string().ok(),
        };
        self.answers.push(answer);
    }

    /// Adds an inline answer for an UrbanDefinition to the formatter's accumulated answers.
    ///
    /// The created answer uses the title "#<i+1> - <word>", copies the definition into `meaning`,
    /// and sets `description` to the formatted example if one exists.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut fmt = InlineFormatter::default();
    /// let def = UrbanDefinition { word: "yeet".into(), meaning: "to throw".into(), example: Some("He yeeted it.".into()) };
    /// fmt.visit_urban_definition(0, &def);
    /// assert_eq!(fmt.answers.len(), 1);
    /// assert!(fmt.answers[0].title.contains("yeet"));
    /// ```
    fn visit_urban_definition(&mut self, i: usize, def: &UrbanDefinition) {
        let answer = InlineAnswer {
            title: format!("#{} - {}", i + 1, def.word),
            meaning: def.meaning.clone(),
            description: def.example.clone().map(|it| as_in(&&it)),
        };
        self.answers.push(answer);
    }

    /// Ignores a word-finder definition during visitation.
    ///
    /// This method is intentionally a no-op because inline formatting does not support
    /// word-finder entries; calls to it have no effect on the formatter's state.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut fmt = crate::inlines::formatting::InlineFormatter::default();
    /// fmt.visit_word_finder_definition(0, &"pattern".to_string());
    /// ```
    fn visit_word_finder_definition(&mut self, _i: usize, _def: &String) {
        // no support for now
    }

    /// Accepts a title but intentionally performs no action.
    ///
    /// This method is a no-op placeholder; provided titles are ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut fmt = InlineFormatter::default();
    /// fmt.append_title("ignored".to_string());
    /// ```
    fn append_title(&mut self, _title: String) {
        // no support for now
    }

    fn append_link(&mut self, _link: String) {
        // no support for now
    }

    fn build(self) -> Result<Self::Value, Self::Error> {
        self.answers
            .iter()
            .enumerate()
            .try_fold(Vec::new(), |mut acc, (i, answer)| {
                let full_text = compose_inline_answer(answer)?;
                let article = compose_inline_result(i, answer, full_text);
                acc.push(article);
                Ok(acc)
            })
    }
}

fn compose_inline_answer(answer: &InlineAnswer) -> Result<String, std::string::FromUtf8Error> {
    let mut full_text = string_builder::Builder::default();
    full_text.append(escape(&answer.title));
    full_text.append("\n\n");
    full_text.append(meaning(&escape(&answer.meaning)));
    if let Some(description) = &answer.description {
        full_text.append("\n");
        full_text.append(escape(&description));
    }
    full_text.string()
}

/// Builds an `InlineQueryResult::Article` from an `InlineAnswer` and its preformatted MarkdownV2 text.
///
/// The article's title is taken from `answer.title`, its description is `answer.meaning`, and the provided
/// `full_text` is used as the message content with MarkdownV2 parse mode. The result ID is formatted as `answer-<i>`.
///
/// # Examples
///
/// ```
/// let answer = InlineAnswer { title: "Example".into(), meaning: "A short meaning".into(), description: None };
/// let full_text = "Example\n\nA short meaning".to_string();
/// let res = compose_inline_result(0, &answer, full_text);
/// assert!(matches!(res, InlineQueryResult::Article(_)));
/// ```
fn compose_inline_result(i: usize, answer: &InlineAnswer, full_text: String) -> InlineQueryResult {
    let content = InputMessageContentText::new(&full_text).parse_mode(ParseMode::MarkdownV2);
    let content = InputMessageContent::Text(content);
    let id = format!("answer-{}", i);
    let article = InlineQueryResultArticle::new(id, &answer.title, content)
        .description(answer.meaning.as_str());
    InlineQueryResult::Article(article)
}