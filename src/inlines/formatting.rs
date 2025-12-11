use crate::bloc::formatting::SynAntFormatterExt;
use crate::format::{as_in, meaning, StringBuilderExt, ToEscaped};
use crate::{
    format::{LinksProvider, LookupFormatter},
    stands4::{AbbreviationDefinition, PhraseDefinition, SynAntDefinitions, WordDefinition},
    urban::UrbanDefinition,
};
use std::string::FromUtf8Error;
use teloxide::types::{
    InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText,
    ParseMode,
};
use teloxide::utils::markdown::escape;

enum Desc {
    Building(string_builder::Builder),
    Done(Result<String, FromUtf8Error>),
}

struct InlineAnswer {
    title: String,
    meaning: Option<String>,
    description: Desc,
}

impl Default for InlineAnswer {
    /// Create an `InlineAnswer` initialized with an empty title and an empty description builder.
    ///
    /// # Examples
    ///
    /// ```
    /// let answer = InlineAnswer::default();
    /// // `answer` is ready to be configured with `.title()`, `.meaning()`, `.description()` etc.
    /// ```
    fn default() -> Self {
        InlineAnswer::new(String::default())
    }
}

impl InlineAnswer {
    /// Create a new `InlineAnswer` with the given title and an empty description buffer.
    ///
    /// The returned value has `meaning` set to `None` and `description` initialized in the
    /// `Building` state with an empty `string_builder::Builder`.
    ///
    /// # Examples
    ///
    /// ```
    /// let ans = InlineAnswer::new("example".into());
    /// assert_eq!(ans.title, "example");
    /// assert!(ans.meaning.is_none());
    /// ```
    fn new(title: String) -> Self {
        Self {
            title,
            meaning: None,
            description: Desc::Building(string_builder::Builder::default()),
        }
    }

    /// Set the inline answer's title and return the modified builder.
    ///
    /// # Examples
    ///
    /// ```
    /// let ans = InlineAnswer::new("initial".into()).title("final title".into());
    /// assert_eq!(ans.title, "final title");
    /// ```
    fn title(mut self, string: String) -> Self {
        self.title = string;
        self
    }

    /// Sets the short meaning (brief description) for the inline answer.
    ///
    /// # Examples
    ///
    /// ```
    /// let ans = InlineAnswer::new("word".to_string()).meaning("a short definition".to_string());
    /// assert_eq!(ans.meaning, Some("a short definition".to_string()));
    /// ```
    fn meaning(mut self, string: String) -> Self {
        self.meaning = Some(string);
        self
    }

    /// Set the answer's description to the given string when the description is still being built; do nothing if the description is already finalized.
    ///
    /// If the description is in the Building state and its buffer already contains text, the buffer is reset before the provided string is appended. If the description is in the Done state, the method returns the same value unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// let ans = InlineAnswer::new("title".into())
    ///     .description("first".into())
    ///     .description("second".into());
    /// ```
    fn description(mut self, string: String) -> Self {
        match &self.description {
            Desc::Building(builder) => {
                if builder.len() > 0 {
                    self.description = Desc::Building(string_builder::Builder::default());
                }
                self.append_description(string)
            }
            Desc::Done(_) => self,
        }
    }

    /// Appends text to the answer's in-progress description buffer.

    ///

    /// If the answer's description is in the `Building` state, the provided string is

    /// appended to that builder. If the description is already `Done`, this method

    /// has no effect.

    ///

    /// # Examples

    ///

    /// ```

    /// let ans = InlineAnswer::new("title".into())

    ///     .append_description("first".into())

    ///     .append_description(" second".into())

    ///     .build_description();

    /// ```
    fn append_description(mut self, string: String) -> Self {
        if let Desc::Building(ref mut builder) = self.description {
            builder.append(string);
        }
        self
    }

    /// Finalizes the accumulated description buffer into a finished string.
    ///
    /// Converts `Desc::Building(buffer)` into `Desc::Done(buffer.string())`; leaves `Desc::Done` unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// let ans = InlineAnswer::new("title".into())
    ///     .append_description("part1")
    ///     .append_description("part2")
    ///     .build_description();
    /// assert!(matches!(ans.description, Desc::Done(_)));
    /// ```
    fn build_description(mut self) -> Self {
        self.description = match self.description {
            Desc::Building(buffer) => Desc::Done(buffer.string()),
            Desc::Done(str) => Desc::Done(str),
        };
        self
    }
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

    /// Adds a word definition to the formatter's accumulated inline answers.
    ///
    /// The new answer's title is formatted as "#<index> - <term> (<part_of_speech>)" where an empty
    /// part of speech is replaced with `"?"`. The answer's meaning is set from `def.definition`.
    /// If `def.example` is non-empty, it is appended as the answer's description.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::{InlineFormatter, WordDefinition};
    /// let mut fmt = InlineFormatter::default();
    /// let def = WordDefinition {
    ///     term: "run".into(),
    ///     part_of_speech: "".into(),
    ///     definition: "move swiftly on foot".into(),
    ///     example: "He can run very fast.".into(),
    /// };
    /// fmt.visit_word(0, &def);
    /// assert_eq!(fmt.answers.len(), 1);
    /// ```
    fn visit_word(&mut self, i: usize, def: &WordDefinition) {
        let part_of_speech = match def.part_of_speech.is_empty() {
            true => &"?".to_string(),
            false => &def.part_of_speech,
        };

        let mut answer =
            InlineAnswer::new(format!("#{} - {} ({})", i + 1, def.term, part_of_speech))
                .meaning(def.definition.clone());
        if !def.example.is_empty() {
            answer = answer.description(def.example.clone());
        }
        self.answers.push(answer);
    }

    /// Appends an inline answer representing a phrase definition, using the index to build the title.
    ///
    /// The created answer's title is "#<index+1> - <term>", its meaning is set from `def.explanation`,
    /// and if `def.example` is non-empty the example is attached as the description.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut fmt = InlineFormatter::default();
    /// let def = PhraseDefinition {
    ///     term: "break a leg".into(),
    ///     explanation: "an expression of good luck".into(),
    ///     example: "Break a leg on your performance!".into(),
    /// };
    /// fmt.visit_phrase(0, &def);
    /// assert_eq!(fmt.answers.len(), 1);
    /// assert!(fmt.answers[0].title.contains("break a leg"));
    /// assert_eq!(fmt.answers[0].meaning.as_deref(), Some("an expression of good luck"));
    /// ```
    fn visit_phrase(&mut self, i: usize, def: &PhraseDefinition) {
        let mut answer = InlineAnswer::new(format!("#{} - {}", i + 1, def.term))
            .meaning(def.explanation.clone());
        if !def.example.is_empty() {
            answer = answer.description(as_in(&def.example));
        }

        self.answers.push(answer);
    }

    /// Creates and appends an InlineAnswer representing an abbreviation category by joining the
    /// provided abbreviation definitions into a comma-separated meaning.
    ///
    /// The `category` is normalized to `"uncategorized"` when empty. The meaning is produced by
    /// concatenating `defs`' `definition` fields separated by ", ". If assembling the meaning fails
    /// due to a UTF-8 conversion error, a fallback message "Cannot describe, try this word in bot's chat"
    /// is used. The created answer's title is "#<index> in [<category>]" (1-based index).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // Assume `fmt` is an InlineFormatter and `defs` is a Vec<&AbbreviationDefinition>.
    /// // This will push a new InlineAnswer onto `fmt.answers`.
    /// // fmt.visit_abbreviations(0, "abbrs", &defs);
    /// ```
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

        let answer = InlineAnswer::new(format!("#{} in [{}]", i + 1, category)).meaning(
            meaning
                .string()
                .unwrap_or_else(|_| "Cannot describe, try this word in bot's chat".to_string()),
        );
        self.answers.push(answer);
    }

    /// Adds a synonym/antonym entry to the formatter as an inline answer.
    ///
    /// The created answer's title is "#{i} {term} [{part_of_speech}]" and its meaning is set
    /// to the definition text from `def`. A description is built from the synonyms and
    /// antonyms; if description assembly succeeds the description is attached to the answer.
    /// The answer is appended to the formatter's internal `answers` list.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut fmt = InlineFormatter::default();
    /// let def = SynAntDefinitions {
    ///     term: "bright".to_string(),
    ///     part_of_speech: "adj".to_string(),
    ///     definition: "giving out or reflecting much light".to_string(),
    ///     synonyms: vec!["luminous".to_string()],
    ///     antonyms: vec!["dull".to_string()],
    /// };
    /// fmt.visit_syn_ant(1, &def);
    /// assert_eq!(fmt.answers.len(), 1);
    /// assert!(fmt.answers[0].title.contains("bright"));
    /// ```
    fn visit_syn_ant(&mut self, i: usize, def: &SynAntDefinitions) {
        let mut description = string_builder::Builder::default();
        Self::push_syn_ant(&mut description, def, || {
            "Surprisingly, there are no synonyms or antonyms to this!".to_string()
        });
        let mut answer =
            InlineAnswer::new(format!("#{} {} [{}]", i + 1, def.term, def.part_of_speech))
                .meaning(def.definition.clone());
        if let Ok(description) = description.string() {
            answer = answer.description(description);
        }
        self.answers.push(answer);
    }

    /// Adds an InlineAnswer for an UrbanDefinition to the formatter's accumulated answers.
    ///
    /// The answer's title is "#<i+1> - <word>", its `meaning` is set from the definition,
    /// and its `description` is set to the formatted example when one exists.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut fmt = InlineFormatter::default();
    /// let def = UrbanDefinition {
    ///     word: "yeet".into(),
    ///     meaning: "to throw".into(),
    ///     example: Some("He yeeted it.".into()),
    /// };
    /// fmt.visit_urban_definition(0, &def);
    /// assert_eq!(fmt.answers.len(), 1);
    /// assert!(fmt.answers[0].title.contains("yeet"));
    /// ```
    fn visit_urban_definition(&mut self, i: usize, def: &UrbanDefinition) {
        let mut answer =
            InlineAnswer::new(format!("#{} - {}", i + 1, def.word)).meaning(def.meaning.clone());
        if let Some(example) = &def.example {
            answer = answer.description(as_in(example));
        }
        self.answers.push(answer);
    }

    /// Appends a word-finder match to the most recent answer and updates its title.
    ///
    /// If there is an existing last answer it is popped, renamed to `Found <i+1> words`,
    /// and the escaped `def` string is appended to its description. If the description
    /// is a building buffer and empty, the escaped `def` is appended as-is; otherwise
    /// it is appended prefixed with `", "`. If there is no existing answer, a default
    /// answer is created and treated the same way.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut fmt = crate::inlines::formatting::InlineFormatter::default();
    /// fmt.visit_word_finder_definition(0, &"pattern".to_string());
    /// ```
    fn visit_word_finder_definition(&mut self, i: usize, def: &String) {
        let def = def.to_escaped();
        let mut answer = self
            .answers
            .pop()
            .unwrap_or_default()
            .title(format!("Found {} words", i + 1));
        answer = match &answer.description {
            Desc::Building(builder) if builder.len() == 0 => {
                answer.append_description(def.to_string())
            }
            Desc::Building(_) => answer.append_description(format!(", {}", def)),
            _ => answer,
        };
        self.answers.push(answer);
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

    /// Finalizes accumulated answers and converts them into inline query result articles.
    ///
    /// This consumes the formatter, finalizes each answer's description, composes the
    /// full MarkdownV2 message text for each answer, and wraps each as an
    /// `InlineQueryResult::Article`.
    ///
    /// # Examples
    ///
    /// ```
    /// let f = InlineFormatter::default();
    /// let results = f.build().unwrap();
    /// assert!(results.is_empty());
    /// ```
    ///
    /// # Returns
    ///
    /// `Ok(Vec<InlineQueryResult>)` with one article per accumulated answer, or
    /// `Err(FromUtf8Error)` if finalizing any answer's description fails.
    fn build(self) -> Result<Self::Value, Self::Error> {
        self.answers
            .into_iter()
            .enumerate()
            .try_fold(Vec::new(), |mut acc, (i, answer)| {
                let answer = answer.build_description();
                let full_text = compose_inline_answer(&answer)?;
                let article = compose_inline_result(i, &answer, full_text);
                acc.push(article);
                Ok(acc)
            })
    }
}

/// Builds the MarkdownV2-formatted message text for an inline answer.
///
/// The returned string starts with the escaped title, optionally followed by a formatted
/// meaning (separated by two newlines), and optionally followed by the escaped description
/// on a new line. If the answer's description is a `Desc::Done(Err(_))`, that error is
/// propagated.
///
/// # Examples
///
/// ```
/// let answer = InlineAnswer {
///     title: "hello".into(),
///     meaning: Some("a greeting".into()),
///     description: Desc::Done(Ok("used in examples".into())),
/// };
/// let text = compose_inline_answer(&answer).unwrap();
/// assert!(text.contains("hello"));
/// assert!(text.contains("a greeting"));
/// assert!(text.contains("used in examples"));
/// ```
fn compose_inline_answer(answer: &InlineAnswer) -> Result<String, FromUtf8Error> {
    let mut full_text = string_builder::Builder::default();
    full_text.appendl(escape(&answer.title));
    if let Some(text) = &answer.meaning {
        full_text.append("\n");
        full_text.append(meaning(&escape(text)));
    }
    if let Desc::Done(desc) = &answer.description {
        match desc {
            Ok(desc) => {
                full_text.append("\n");
                full_text.append(desc.as_str());
            }
            Err(err) => {
                return Err(err.clone());
            }
        }
    }
    full_text.string()
}

/// Builds an InlineQueryResult::Article from an InlineAnswer and its preformatted MarkdownV2 text.
///
/// The article's title is taken from `answer.title`, the article description is set from `answer.meaning` when present,
/// and the provided `full_text` is used as the message content with MarkdownV2 parse mode. The result id is formatted as `answer-<i>`.
///
/// # Examples
///
/// ```
/// let answer = InlineAnswer::new("Example".into()).meaning("A short meaning".into());
/// let full_text = "Example\n\nA short meaning".to_string();
/// let res = compose_inline_result(0, &answer, full_text);
/// assert!(matches!(res, InlineQueryResult::Article(_)));
/// ```
fn compose_inline_result(i: usize, answer: &InlineAnswer, full_text: String) -> InlineQueryResult {
    let content = InputMessageContentText::new(&full_text).parse_mode(ParseMode::MarkdownV2);
    let content = InputMessageContent::Text(content);
    let id = format!("answer-{}", i);
    let mut article = InlineQueryResultArticle::new(id, &answer.title, content);
    if let Some(meaning) = &answer.meaning {
        article = article.description(meaning);
    }
    InlineQueryResult::Article(article)
}
