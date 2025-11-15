use crate::bloc::common::{HandlerOwner, LookupError};
use crate::bot::LookupBotX;
use crate::commands::CommandHandler;
use crate::format::LookupFormatter;
use crate::urban::{UrbanDefinition, UrbanDictionaryClient};
use teloxide::dptree::entry;

pub struct UrbanLookup;
impl UrbanLookup {
    async fn get_definitions(
        client: UrbanDictionaryClient,
        term: String,
    ) -> Result<Vec<UrbanDefinition>, LookupError> {
        client.search_term(&term).await.map_err(|e| {
            log::error!("term lookup error: {:?}", e);
            LookupError::FailedRequest
        })
    }

    fn compose_response<Formatter>(
        term: String,
        mut formatter: Formatter,
        defs: Vec<UrbanDefinition>,
    ) -> Result<Formatter::Value, LookupError>
    where
        Formatter: LookupFormatter,
    {
        formatter.append_title(format!(
            "Found {} definitions from Urban Dictionary",
            defs.len()
        ));

        for (i, def) in defs.iter().take(5).enumerate() {
            formatter.visit_urban_definition(i, def);
        }
        if defs.len() > 5 {
            formatter.append_link(formatter.link_provider().urban_link(&term))
        }
        formatter.build().map_err(|err| {
            log::error!("Failed to construct a response: {:?}", err);
            LookupError::FailedResponseBuilder
        })
    }
}

impl HandlerOwner for UrbanLookup {
    fn handler<Bot>() -> CommandHandler
    where
        Bot: LookupBotX + Clone + Send + Sync + 'static,
    {
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
                    Self::compose_response(phrase, bot.formatter(), defs)
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
