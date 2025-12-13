use crate::bloc::common::{CommandHandler, LookupError};
use crate::bot::{LookupBot, LookupBotX};
use crate::format::LookupFormatter;
use crate::urban::{UrbanDefinition, UrbanDictionaryClient};
use teloxide::dptree::entry;

pub trait UrbanLookupBot<Response>
where
    Response: Send + Default,
{
    /// Provides the default response returned when a lookup phrase is empty.
    ///
    /// # Returns
    ///
    /// A `Response` value constructed with `Response::default()`.
    ///
    /// # Examples
    ///
    /// ```
    /// struct Bot;
    /// impl UrbanLookupBot<String> for Bot {}
    /// let empty = Bot::on_empty();
    /// assert_eq!(empty, String::default());
    /// ```
    fn on_empty() -> Response {
        Response::default()
    }
}

pub trait UrbanLookupHandler {
    /// Fetches definitions for `term` from Urban Dictionary using the provided client.
    ///
    /// # Parameters
    ///
    /// - `term`: The search term to query Urban Dictionary for.
    ///
    /// # Returns
    ///
    /// `Ok` with a vector of `UrbanDefinition` when the lookup succeeds; `Err(LookupError::FailedRequest)` if the remote request fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use your_crate::{get_definitions, UrbanDictionaryClient, UrbanDefinition, LookupError};
    /// # async fn example() -> Result<(), LookupError> {
    /// let client = UrbanDictionaryClient::new(); // construct client appropriately
    /// let defs: Vec<UrbanDefinition> = get_definitions(client, "rust".to_string()).await?;
    /// assert!(!defs.is_empty() || defs.is_empty()); // placeholder assertion
    /// # Ok(())
    /// # }
    /// ```
    async fn get_definitions(
        client: UrbanDictionaryClient,
        term: String,
    ) -> Result<Vec<UrbanDefinition>, LookupError> {
        client.search_term(&term).await.map_err(|e| {
            log::error!("term lookup error: {:?}", e);
            LookupError::FailedRequest
        })
    }

    fn urban_lookup_handler() -> CommandHandler;
}
trait UrbanFormatter<Value> {
    fn compose_urban_response(
        self,
        term: String,
        defs: Vec<UrbanDefinition>,
    ) -> Result<Value, LookupError>;
}

impl<Formatter> UrbanFormatter<Formatter::Value> for Formatter
where
    Formatter: LookupFormatter,
{
    /// Compose a formatted response for Urban Dictionary search results.
    ///
    /// The response includes a title stating the total number of definitions, up to the first five definitions, and — when more than five definitions exist — a link to the full Urban Dictionary entry for the term.
    ///
    /// # Returns
    ///
    /// `Ok(Formatter::Value)` with the constructed response on success, `Err(LookupError::FailedResponseBuilder)` if the formatter fails to build the final value.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let formatter = Formatter::new();
    /// let defs: Vec<UrbanDefinition> = Vec::new();
    /// let _ = formatter.compose_urban_response("rust".into(), defs);
    /// ```
    fn compose_urban_response(
        mut self,
        term: String,
        defs: Vec<UrbanDefinition>,
    ) -> Result<Formatter::Value, LookupError> {
        self.append_title(format!(
            "Found {} definitions from Urban Dictionary",
            defs.len()
        ));

        for (i, def) in defs.iter().take(5).enumerate() {
            self.visit_urban_definition(i, def);
        }
        if defs.len() > 5 {
            self.append_link(self.link_provider().urban_link(&term))
        }
        self.build().map_err(|err| {
            log::error!("Failed to construct a response: {:?}", err);
            LookupError::FailedResponseBuilder
        })
    }
}

impl<Bot, Formatter> UrbanLookupHandler for Bot
where
    Formatter: LookupFormatter<Value = Bot::Response>,
    Bot: UrbanLookupBot<Bot::Response> + LookupBot<Formatter = Formatter> + Send + Sync + 'static,
{
    /// Creates a Teloxide command handler that processes Urban Dictionary lookups by validating the input phrase, retrieving definitions, formatting a response, and sending it via the bot.
    ///
    /// # Examples
    ///
    /// ```
    /// // Construct the command handler for urban lookups.
    /// let handler = Bot::urban_lookup_handler();
    /// ```
    fn urban_lookup_handler() -> CommandHandler {
        entry()
            .filter_async(|bot: Bot, phrase: String| async move {
                bot.drop_empty(phrase, Self::on_empty).await
            })
            .map_async(Self::get_definitions)
            .filter_map_async(
                |bot: Bot, response: Result<Vec<UrbanDefinition>, LookupError>| async move {
                    bot.ensure_request_success(response).await
                },
            )
            .map(
                move |bot: Bot, phrase: String, defs: Vec<UrbanDefinition>| {
                    bot.formatter().compose_urban_response(phrase, defs)
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
