use crate::bloc::common::{CommandHandler, LookupError};
use crate::bot::LookupBot;
use crate::format::LookupFormatter;
use crate::urban::{UrbanDefinition, UrbanDictionaryClient};
use teloxide::dptree::entry;

pub trait UrbanLookupBot {}
pub trait UrbanLookupHandler {
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

impl<Bot> UrbanLookupHandler for Bot
where
    Bot: LookupBot + Send + Sync + 'static,
{
    fn urban_lookup_handler() -> CommandHandler {
        entry()
            .filter_async(|bot: Bot, phrase: String| async move { bot.drop_empty(phrase).await })
            .map_async(Self::get_definitions)
            .filter_map_async(
                |bot: Bot, response: Result<Vec<UrbanDefinition>, LookupError>| async move {
                    bot.ensure_request_success(response).await
                },
            )
            .map(
                |bot: Bot, phrase: String, defs: Vec<UrbanDefinition>| async move {
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
