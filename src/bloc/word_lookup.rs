use crate::bloc::common::{CommandHandler, LookupError};
use crate::bot::{LookupBot, LookupBotX};
use crate::format::LookupFormatter;
use crate::stands4::{AbbreviationDefinition, Stands4Client, VecAbbreviationsExt, WordDefinition};
use futures::TryFutureExt;
use teloxide::dptree::entry;

type Entity = (Vec<WordDefinition>, Vec<AbbreviationDefinition>);

pub trait WordLookupBot<Response>
where
    Response: Send + Default,
{
    /// Provide the bot's default empty response value.
    ///
    /// # Returns
    ///
    /// The default `Response` value.
    ///
    /// # Examples
    ///
    /// ```
    /// struct MyBot;
    /// impl WordLookupBot<String> for MyBot {}
    /// let empty = <MyBot as WordLookupBot<String>>::on_empty();
    /// assert_eq!(empty, String::default());
    /// ```
    fn on_empty() -> Response {
        Default::default()
    }
}

pub trait WordLookupHandler {
    /// Performs concurrent lookups for both word definitions and abbreviation definitions.
    ///
    /// If either lookup fails the function logs the error and substitutes an empty vector for that result.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Example usage — `Stands4Client` must implement the async search methods used here.
    /// // This snippet demonstrates the call pattern; replace with a real client in tests.
    /// # async fn example(client: Stands4Client) {
    /// let (words, abbrs) = get_definitions(client, "rust".to_string()).await;
    /// // `words` is a Vec<WordDefinition>, `abbrs` is a Vec<AbbreviationDefinition>.
    /// # }
    /// ```
    async fn get_definitions(client: Stands4Client, word: String) -> Entity {
        futures::future::join(
            client.search_word(&word).unwrap_or_else(|err| {
                log::error!("Failed to retrieve definitions of a word: {:?}", err);
                vec![]
            }),
            client.search_abbreviation(&word).unwrap_or_else(|err| {
                log::error!("Failed to retrieve definitions of an abbr: {:?}", err);
                vec![]
            }),
        )
        .await
    }

    fn word_lookup_handler() -> CommandHandler;
}

pub trait WordLookupFormatter<Value, Error> {
    fn compose_word_defs(self, word: &str, defs: &Vec<WordDefinition>) -> Result<Value, Error>;

    fn compose_abbr_defs(
        self,
        word: &str,
        defs: &Vec<AbbreviationDefinition>,
    ) -> Result<Value, Error>;

    fn compose_words_with_abbrs(
        self,
        word: &str,
        words: &Vec<WordDefinition>,
        abbrs: &Vec<AbbreviationDefinition>,
    ) -> Result<Value, Error>;

    fn compose_word_response(self, word: String, entity: Entity) -> Result<Value, LookupError>;
}

impl<Formatter> WordLookupFormatter<Formatter::Value, Formatter::Error> for Formatter
where
    Formatter: LookupFormatter,
{
    /// Appends a title and up to five word definitions to the formatter, adding a link if more exist.
    ///
    /// Adds a heading "Found N definitions", visits up to five definitions with `visit_word`, appends a link to the word when more than five definitions are present, then builds and returns the formatter's value.
    ///
    /// # Returns
    ///
    /// The formatter's built value on success.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assuming a type `F` implements the required `LookupFormatter` methods:
    /// // let result = F::new().compose_word_defs("example", &definitions)?;
    /// ```
    fn compose_word_defs(
        mut self,
        word: &str,
        defs: &Vec<WordDefinition>,
    ) -> Result<Formatter::Value, Formatter::Error> {
        self.append_title(format!("Found {} definitions", defs.len()));

        for (i, def) in defs.iter().take(5).enumerate() {
            self.visit_word(i, def);
        }
        if defs.len() > 5 {
            self.append_link(self.link_provider().word_link(word))
        }
        self.build()
    }

    /// Formats abbreviation definitions for a word by categorizing them and composing the formatter's output.
    ///
    /// The formatter will include up to five categories of abbreviations and, if more categories exist,
    /// append a link to the full abbreviation list for the given word.
    ///
    /// # Parameters
    ///
    /// - `word`: the queried word used for link generation when more categories exist.
    /// - `defs`: the list of abbreviation definitions to organize and format.
    ///
    /// # Returns
    ///
    /// `Formatter::Value` containing the composed representation of the abbreviation definitions.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assume `MyFormatter` implements `LookupFormatter` and has `Value` = String, `Error` = std::convert::Infallible
    /// let formatter = MyFormatter::new();
    /// let word = "cpu";
    /// let abbr_defs: Vec<AbbreviationDefinition> = vec![/* ... */];
    /// let result = formatter.compose_abbr_defs(word, &abbr_defs).unwrap();
    /// // `result` is the formatted output (e.g., a message or document) containing categorized abbreviations
    /// ```
    fn compose_abbr_defs(
        mut self,
        word: &str,
        defs: &Vec<AbbreviationDefinition>,
    ) -> Result<Formatter::Value, Formatter::Error> {
        self.append_title(format!("Found {} definitions", defs.len()));

        let categorized = defs.categorized();
        for (i, (category, defs)) in categorized.iter().take(5).enumerate() {
            self.visit_abbreviations(i, category, defs);
        }
        if categorized.len() > 5 {
            self.append_link(self.link_provider().abbr_link(word))
        }
        self.build()
    }

    /// Compose a formatted response containing both word definitions and categorized abbreviations for a query.
    ///
    /// This appends a title for word definitions, visits up to five word definitions (adding a "more" link if there are more),
    /// then appends a title for abbreviations, visits up to five abbreviation categories (adding an abbreviation "more" link if there are more),
    /// and finally builds the formatter's output.
    ///
    /// Parameters:
    /// - `word`: the queried word used when building "more" links.
    /// - `words`: list of word definitions to include.
    /// - `abbrs`: list of abbreviation definitions to categorize and include.
    ///
    /// # Returns
    ///
    /// The formatter's built value on success, or the formatter's error type on failure.
    ///
    /// # Examples
    ///
    /// ```
    /// // Prepare inputs (types from the surrounding crate are not required here; this is illustrative).
    /// let word = "test";
    /// let words: Vec<crate::WordDefinition> = Vec::new();
    /// let abbrs: Vec<crate::AbbreviationDefinition> = Vec::new();
    ///
    /// // Typical usage:
    /// // let result = formatter.compose_words_with_abbrs(word, &words, &abbrs);
    /// // assert!(result.is_ok());
    ///
    /// // No-op assertion to keep this example self-contained.
    /// assert!(true);
    /// ```
    fn compose_words_with_abbrs(
        mut self,
        word: &str,
        words: &Vec<WordDefinition>,
        abbrs: &Vec<AbbreviationDefinition>,
    ) -> Result<Formatter::Value, Formatter::Error> {
        self.append_title(format!("Found {} definitions", words.len()));

        for (i, def) in words.iter().take(5).enumerate() {
            self.visit_word(i, def);
        }
        if words.len() > 5 {
            self.append_link(self.link_provider().word_link(word))
        }

        self.append_title(format!("Found {} abbreviations", abbrs.len()));

        let categorized = abbrs.categorized();
        for (i, (category, defs)) in categorized.iter().take(5).enumerate() {
            self.visit_abbreviations(i, category, defs);
        }
        if categorized.len() > 5 {
            self.append_link(self.link_provider().abbr_link(word))
        }

        self.build()
    }

    /// Selects and builds the appropriate formatted response for a lookup result containing
    /// word definitions and abbreviations.
    ///
    /// This method dispatches to one of the formatter's composing helpers based on which parts
    /// of the composite `Entity` are present:
    /// - If both word definitions and abbreviations are empty, returns `Self::on_empty()`.
    /// - If only abbreviations are present, delegates to `compose_abbr_defs`.
    /// - If only word definitions are present, delegates to `compose_word_defs`.
    /// - If both are present, delegates to `compose_words_with_abbrs`.
    ///
    /// On any builder error the function logs the failure and maps the error to
    /// `LookupError::FailedResponseBuilder`.
    ///
    /// # Returns
    ///
    /// `Ok(Value)` containing the composed response when building succeeds, or
    /// `Err(LookupError::FailedResponseBuilder)` when the response builder fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // Assume `formatter` implements the trait providing this method.
    /// // The call picks the appropriate composition path from the tuple contents.
    /// let entity_empty: (Vec<_>, Vec<_>) = (Vec::new(), Vec::new());
    /// // let result = formatter.compose_word_response("term".to_string(), entity_empty);
    /// ```
    fn compose_word_response(
        self,
        word: String,
        (words, abbrs): Entity,
    ) -> Result<Formatter::Value, LookupError> {
        let text = match (words.len(), abbrs.len()) {
            (0, 0) => Ok(Self::on_empty()),
            (0, _) => self.compose_abbr_defs(&word, &abbrs),
            (_, 0) => self.compose_word_defs(&word, &words),
            (_, _) => self.compose_words_with_abbrs(&word, &words, &abbrs),
        };
        text.map_err(|err| {
            log::error!("Failed to construct a response: {:?}", err);
            LookupError::FailedResponseBuilder
        })
    }
}

impl<Bot, Formatter> WordLookupHandler for Bot
where
    Bot: WordLookupBot<Bot::Response> + LookupBot<Formatter = Formatter> + Send + Sync + 'static,
    Formatter: LookupFormatter<Value = Bot::Response>,
{
    /// Builds a teloxide dptree CommandHandler that handles a phrase lookup and sends the bot's formatted response.
    ///
    /// The handler chain drops empty phrases (using `Bot::on_empty`), retrieves word and abbreviation definitions,
    /// composes a formatted response or substitutes a generic error response, and sends the result via the bot.
    ///
    /// # Examples
    ///
    /// ```
    /// let handler = Bot::word_lookup_handler();
    /// // Mount `handler` into a teloxide dispatcher dptree.
    /// ```
    fn word_lookup_handler() -> CommandHandler {
        entry()
            .filter_async(|bot: Bot, phrase: String| async move {
                bot.drop_empty(phrase, Bot::on_empty).await
            })
            .map_async(Self::get_definitions)
            .map(move |bot: Bot, phrase: String, defs: Entity| {
                bot.formatter().compose_word_response(phrase, defs)
            })
            .filter_map_async(
                |bot: Bot, response: Result<Bot::Response, LookupError>| async move {
                    bot.retrieve_or_generic_err(response).await
                },
            )
            .endpoint(
                |bot: Bot, response: Bot::Response| async move { bot.respond(response).await },
            )
    }
}
