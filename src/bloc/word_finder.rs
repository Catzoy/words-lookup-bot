use crate::bloc::common::{CommandHandler, LookupError};
use crate::bot::LookupBot;
use crate::datamuse::client::DatamuseClient;
use crate::format::{LookupFormatter, ToEscaped};
use teloxide::dptree::entry;

pub trait WordFinderBot<Response>
where
    Response: Send + Default,
{
    /// Provides the default response to use when the user submits an empty query.
    ///
    /// By default this returns `Default::default()` for the response type.
    ///
    /// # Examples
    ///
    /// ```
    /// struct Bot;
    /// impl word_finder::WordFinderBot<String> for Bot {}
    /// let resp = Bot::on_empty();
    /// assert_eq!(resp, String::default());
    /// ```
    fn on_empty() -> Response {
        Default::default()
    }

    /// Provides the default response used when the user-supplied mask has an invalid length.
    ///
    /// Returns the default `Response` value.
    ///
    /// # Examples
    ///
    /// ```
    /// struct BotDummy;
    /// impl WordFinderBot<()> for BotDummy {}
    /// let resp = BotDummy::on_length_invalid();
    /// assert_eq!(resp, ());
    /// ```
    fn on_length_invalid() -> Response {
        Default::default()
    }

    /// Produce a response when the input mask contains an unsupported character.
    ///
    /// This hook is invoked when validation detects a character other than ASCII letters or `_` in the mask.
    ///
    /// # Returns
    ///
    /// `Response` to send to the user; the default implementation returns `Default::default()`.
    fn on_unknown_character() -> Response {
        Default::default()
    }

    /// Produces the response the bot should send when the user's query is invalid.
    ///
    /// The default implementation returns `Default::default()` for the response type.
    /// Implementors may override to provide a custom user-facing reply.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::default::Default;
    /// struct Resp;
    /// impl Default for Resp { fn default() -> Self { Resp } }
    ///
    /// trait WordFinderBot<R> { fn on_invalid_query() -> R where R: Default { Default::default() } }
    ///
    /// struct MyBot;
    /// impl WordFinderBot<Resp> for MyBot {}
    ///
    /// let resp = MyBot::on_invalid_query();
    /// ```
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
    /// Compose a formatted response value from a list of word definitions.
    ///
    /// The formatter adds a title "Found N words" where N is the number of
    /// provided definitions, visits each definition in order, and builds the
    /// final response value.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let defs = vec!["apple".to_string(), "angle".to_string()];
    /// let response = Formatter::new().compose_word_finder_response(defs).unwrap();
    /// ```
    ///
    /// # Returns
    ///
    /// The constructed formatter value on success, or a `LookupError::FailedResponseBuilder` if building fails.
    fn compose_word_finder_response(
        mut self,
        defs: Vec<String>,
    ) -> Result<Formatter::Value, LookupError> {
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
    /// Validate a word-mask pattern and notify the user when the mask is invalid.
    ///
    /// The mask must be 2–15 characters long, contain at least one underscore (`_`)
    /// and at least one ASCII letter (`A`–`Z`, `a`–`z`), and may contain only
    /// letters or underscores. When the mask fails any check this method sends the
    /// corresponding user-facing response (via `Self::on_length_invalid`,
    /// `Self::on_unknown_character`, or `Self::on_invalid_query`) and returns `false`.
    ///
    /// # Returns
    ///
    /// `true` if the mask meets length and content requirements, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::future::Future;
    /// # // This example demonstrates the call shape; replace `Bot` with your bot type.
    /// # async fn example<B: std::marker::Send + ?Sized>(_bot: &B) {}
    /// # async {
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
    /// Create a Teloxide command handler for the word-finder feature.
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