use crate::bloc::common::{CommandHandler, LookupError};
use crate::bot::LookupBot;
use crate::datamuse::client::DatamuseClient;
use crate::format::{LookupFormatter, ToEscaped};
use teloxide::dptree::entry;

pub trait WordFinderBot<Response>
where
    Response: Send + Default,
{
    fn on_empty() -> Response {
        Default::default()
    }

    fn on_length_invalid() -> Response {
        Default::default()
    }

    fn on_unknown_character() -> Response {
        Default::default()
    }

    fn on_invalid_query() -> Response {
        Default::default()
    }
}

pub trait WordFinderHandler {
    /// Fetches candidate words that match a pattern from the Datamuse service.
    ///
    /// The `mask` uses `'_'` to represent unknown letters (blanks); other characters are treated as filled letters.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // Example usage (requires async runtime):
    /// // let client = DatamuseClient::new();
    /// // let words = get_possible_words(client, "_at".into()).await?;
    /// // assert!(words.iter().any(|w| w.ends_with("at")));
    /// ```
    ///
    /// # Returns
    ///
    /// A `Vec<String>` containing words matching the provided mask, or `LookupError::FailedRequest` if the remote request fails.
    async fn get_possible_words(
        client: DatamuseClient,
        mask: String,
    ) -> Result<Vec<String>, LookupError> {
        client.find(mask).await.map_err(|err| {
            log::error!("WF failed request: {}", err);
            LookupError::FailedRequest
        })
    }

    async fn ensure_valid(&self, mask: String) -> bool;

    fn word_finder_handler() -> CommandHandler;
}

trait WordFinderFormatter<Value> {
    fn compose_word_finder_response(self, defs: Vec<String>) -> Result<Value, LookupError>;
}

impl<Formatter> WordFinderFormatter<Formatter::Value> for Formatter
where
    Formatter: LookupFormatter,
{
    /// Builds a formatted response containing the provided word definitions.
    ///
    /// The formatter will escape each definition, prepend a title of the form
    /// "Found N words" where N is the number of definitions, and visit each
    /// definition in order before building the final response value.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // Given a formatter that implements `LookupFormatter`:
    /// let defs = vec!["apple".to_string(), "angle".to_string()];
    /// let response = Formatter::new().compose_word_finder_response(defs).unwrap();
    /// ```
    fn compose_word_finder_response(
        mut self,
        defs: Vec<String>,
    ) -> Result<Formatter::Value, LookupError> {
        let defs = defs.to_escaped();
        self.append_title(format!("Found {} words", defs.len()));
        for (i, def) in defs.iter().enumerate() {
            self.visit_word_finder_definition(i, def);
        }
        self.build().map_err(|err| {
            log::error!("Failed to construct a response: {:?}", err);
            LookupError::FailedResponseBuilder
        })
    }
}

impl<Bot, Formatter> WordFinderHandler for Bot
where
    Bot: WordFinderBot<Bot::Response> + LookupBot<Formatter = Formatter> + Send + Sync + 'static,
    Formatter: LookupFormatter<Value = Bot::Response>,
{
    /// Validates a word-mask pattern and notifies the user on invalid input.
    ///
    /// The mask must be 2–15 characters long, contain at least one underscore (`_`)
    /// and at least one ASCII letter (`A`–`Z`, `a`–`z`), and may contain only
    /// letters or underscores. When the mask is invalid, this method signals a
    /// generic error to the user by calling `answer_generic_err()` and returns
    /// `false`.
    ///
    /// # Parameters
    ///
    /// - `mask`: A pattern where `_` represents a blank and letters represent known
    ///   characters.
    ///
    /// # Returns
    ///
    /// `true` if the mask meets length and content requirements, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # async fn example<B: ?Sized + std::marker::Send>(_bot: &B) {}
    /// # async {
    /// // Typical usage inside an async context:
    /// // let ok = bot.ensure_valid("a__le".to_string()).await;
    /// # }
    /// ```
    async fn ensure_valid(&self, mask: String) -> bool {
        if mask.len() < 2 || mask.len() > 15 {
            let _ = self.answer(Self::on_length_invalid()).await;
            return false;
        }

        let mut has_blank = false;
        let mut has_filled = false;
        for letter in mask.chars() {
            match letter {
                '_' => {
                    has_blank = true;
                }
                'a'..='z' | 'A'..='Z' => {
                    has_filled = true;
                }
                _ => {
                    let _ = self.answer(Self::on_unknown_character()).await;
                    return false;
                }
            }
        }
        if !has_blank || !has_filled {
            let _ = self.answer(Self::on_invalid_query()).await;
            false
        } else {
            true
        }
    }
    /// Builds the Telegram command pipeline for the word-finder feature.
    ///
    /// The pipeline validates and filters the input mask, queries possible words,
    /// formats the results into a response, and sends the response to the user.
    ///
    /// # Examples
    ///
    /// ```
    /// // Construct the handler; run-time wiring (bot, dispatcher) is required to execute it.
    /// let _handler: CommandHandler = word_finder_handler();
    /// ```
    fn word_finder_handler() -> CommandHandler {
        entry()
            .filter_async(|bot: Bot, mask: String| async move {
                bot.drop_empty(mask, Self::on_empty).await
            })
            .filter_async(|bot: Bot, mask: String| async move { bot.ensure_valid(mask).await })
            .map_async(Self::get_possible_words)
            .filter_map_async(
                |bot: Bot, response: Result<Vec<String>, LookupError>| async move {
                    bot.ensure_request_success(response).await
                },
            )
            .map(move |bot: Bot, defs: Vec<String>| {
                bot.formatter().compose_word_finder_response(defs)
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
