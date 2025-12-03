use crate::bloc::common::{CommandHandler, LookupError};
use crate::bot::LookupBot;
use crate::format::LookupFormatter;
use crate::stands4::{Stands4Client, SynAntDefinitions};
use teloxide::dptree::entry;

pub trait ThesaurusLookupBot {}
pub trait ThesaurusLookupHandler {
    /// Retrieve synonym and antonym definitions for a term from the Stands4 service.
    ///
    /// On success returns a vector of `SynAntDefinitions`. If the underlying client request
    /// fails the function logs the error and returns `LookupError::FailedRequest`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crate::clients::stands4::Stands4Client;
    /// # use crate::bloc::thesaurus_lookup::get_definitions;
    /// # async fn example() -> Result<(), crate::bloc::common::LookupError> {
    /// let client = Stands4Client::new(); // create/configure client as appropriate
    /// let defs = get_definitions(client, "happy".to_string()).await?;
    /// assert!(defs.len() >= 0);
    /// # Ok(()) }
    /// ```
    async fn get_definitions(
        client: Stands4Client,
        term: String,
    ) -> Result<Vec<SynAntDefinitions>, LookupError> {
        client.search_syn_ant(&term).await.map_err(|e| {
            log::error!("term lookup error: {:?}", e);
            LookupError::FailedRequest
        })
    }

    fn thesaurus_lookup_handler() -> CommandHandler;
}
trait ThesaurusLookupFormatter<Value> {
    fn compose_thesaurus_response(
        self,
        term: String,
        defs: Vec<SynAntDefinitions>,
    ) -> Result<Value, LookupError>;
}

impl<Formatter> ThesaurusLookupFormatter<Formatter::Value> for Formatter
where
    Formatter: LookupFormatter,
{
    /// Builds a formatted thesaurus response for a term from a list of synonym/antonym definitions.
    ///
    /// The formatter will append a title indicating how many definitions were found, include up to
    /// the first five definitions via `visit_syn_ant`, and append a link to additional definitions if
    /// more than five were returned. Returns the formatter's built value or `LookupError::FailedResponseBuilder`
    /// if the builder fails.
    ///
    /// # Returns
    ///
    /// `Formatter::Value` on success, `LookupError::FailedResponseBuilder` on builder failure.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Given a formatter `fmt: Formatter` and definitions `defs: Vec<SynAntDefinitions>`:
    /// let value = fmt.compose_thesaurus_response("example".to_string(), defs)?;
    /// // `value` is the built formatter output ready to be returned by the bot.
    /// ```
    fn compose_thesaurus_response(
        mut self,
        term: String,
        defs: Vec<SynAntDefinitions>,
    ) -> Result<Formatter::Value, LookupError> {
        self.append_title(format!(
            "Found {} different definitions with respective information",
            defs.len()
        ));
        for (i, def) in defs.iter().take(5).enumerate() {
            self.visit_syn_ant(i, def)
        }
        if defs.len() > 5 {
            self.append_link(self.link_provider().syn_ant_link(&term))
        }

        self.build().map_err(|err| {
            log::error!("Failed to construct a response: {:?}", err);
            LookupError::FailedResponseBuilder
        })
    }
}

impl<Bot> ThesaurusLookupHandler for Bot
where
    Bot: ThesaurusLookupBot + LookupBot + Send + Sync + 'static,
{
    /// Constructs a teloxide command handler that performs a complete thesaurus lookup flow:
    /// it validates the incoming phrase, fetches synonym/antonym definitions, formats a response,
    /// and sends the response via the bot.
    ///
    /// # Examples
    ///
    /// ```
    /// // Create the handler and assert its type.
    /// let handler = thesaurus_lookup_handler();
    /// let _: CommandHandler = handler;
    /// ```
    fn thesaurus_lookup_handler() -> CommandHandler {
        entry()
            .filter_async(|bot: Bot, phrase: String| async move { bot.drop_empty(phrase).await })
            .map_async(Self::get_definitions)
            .filter_map_async(
                |bot: Bot, response: Result<Vec<SynAntDefinitions>, LookupError>| async move {
                    bot.ensure_request_success(response).await
                },
            )
            .map(
                |bot: Bot, phrase: String, defs: Vec<SynAntDefinitions>| async move {
                    bot.formatter().compose_thesaurus_response(phrase, defs)
                },
            )
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