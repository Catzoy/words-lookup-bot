use crate::bloc::common::{CommandHandler, LookupError};
use crate::bot::LookupBot;
use crate::format::LookupFormatter;
use crate::stands4::{PhraseDefinition, Stands4Client};
use teloxide::dptree::entry;

pub trait PhraseLookupBot {}
pub trait PhraseLookupHandler {
    /// Fetches definitions for a phrase from the Stands4 API.
    ///
    /// On HTTP or request failure this logs an error and yields `LookupError::FailedRequest`.
    ///
    /// # Parameters
    ///
    /// - `phrase`: Phrase to search for.
    ///
    /// # Returns
    ///
    /// A `Vec<PhraseDefinition>` with the found definitions, or `LookupError::FailedRequest` if the request fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # use crate::bloc::phrase_lookup::{get_definitions, Stands4Client};
    /// let client = Stands4Client::new("API_KEY");
    /// let defs = get_definitions(client, "hello".to_string()).await?;
    /// assert!(defs.len() >= 0);
    /// # Ok(()) }
    /// ```
    async fn get_definitions(
        client: Stands4Client,
        phrase: String,
    ) -> Result<Vec<PhraseDefinition>, LookupError> {
        client.search_phrase(phrase.as_str()).await.map_err(|e| {
            log::error!("phrase search error: {:?}", e);
            LookupError::FailedRequest
        })
    }

    fn phrase_lookup_handler() -> CommandHandler;
}

trait PhraseLookupFormatter<Value> {
    fn compose_phrase_response(
        self,
        phrase: String,
        defs: Vec<PhraseDefinition>,
    ) -> Result<Value, LookupError>;
}

impl<Formatter> PhraseLookupFormatter<Formatter::Value> for Formatter
where
    Formatter: LookupFormatter,
{
    /// Builds a formatted response for a phrase from its definitions.
    ///
    /// The formatter will append a title reporting the total number of definitions,
    /// include up to the first five definitions, and append a phrase link when more
    /// than five definitions are available. On success returns the formatter's
    /// built value; on failure returns `LookupError::FailedResponseBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// // Given a formatter `fmt`, a phrase and collected definitions:
    /// // let result = fmt.compose_phrase_response("hello".to_string(), defs);
    /// // `result` is `Ok(value)` when the formatter could build a response,
    /// // or `Err(LookupError::FailedResponseBuilder)` on build failure.
    /// ```
    fn compose_phrase_response(
        mut self,
        phrase: String,
        defs: Vec<PhraseDefinition>,
    ) -> Result<Formatter::Value, LookupError> {
        self.append_title(format!("Found {} definitions", defs.len()));

        for (i, def) in defs.iter().take(5).enumerate() {
            self.visit_phrase(i, def);
        }
        if defs.len() > 5 {
            self.append_link(self.link_provider().phrase_link(&phrase));
        }

        self.build().map_err(|err| {
            log::error!("Failed to construct a response: {:?}", err);
            LookupError::FailedResponseBuilder
        })
    }
}
impl<Bot> PhraseLookupHandler for Bot
where
    Bot: PhraseLookupBot + LookupBot + Send + Sync + 'static,
{
    /// Constructs a command handler pipeline that processes phrase lookup requests by validating input, retrieving definitions, formatting a response, and sending that response to the user.
    ///
    /// The handler performs the full phrase-lookup flow and normalizes errors into the bot's expected response path.
    ///
    /// # Examples
    ///
    /// ```
    /// // Build the handler and bind its type
    /// let _handler: CommandHandler = phrase_lookup_handler();
    /// ```
    fn phrase_lookup_handler() -> CommandHandler {
        entry()
            .filter_async(|bot: Bot, phrase: String| async move { bot.drop_empty(phrase).await })
            .map_async(Self::get_definitions)
            .filter_map_async(
                |bot: Bot, response: Result<Vec<PhraseDefinition>, LookupError>| async move {
                    bot.ensure_request_success(response).await
                },
            )
            .map(|bot: Bot, phrase: String, defs: Vec<PhraseDefinition>| {
                bot.formatter().compose_phrase_response(phrase, defs)
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