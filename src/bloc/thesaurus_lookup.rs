use crate::bloc::common::{HandlerOwner, LookupError};
use crate::bot::LookupBot;
use crate::commands::CommandHandler;
use crate::format::LookupFormatter;
use crate::stands4::{Stands4Client, SynAntDefinitions};
use teloxide::dptree::entry;

pub struct ThesaurusLookup;
impl ThesaurusLookup {
    async fn get_definitions(
        client: Stands4Client,
        term: String,
    ) -> Result<Vec<SynAntDefinitions>, LookupError> {
        client.search_syn_ant(&term).await.map_err(|e| {
            log::error!("term lookup error: {:?}", e);
            LookupError::FailedRequest
        })
    }

    fn compose_response<Formatter>(
        term: String,
        mut formatter: Formatter,
        defs: Vec<SynAntDefinitions>,
    ) -> Result<Formatter::Value, LookupError>
    where
        Formatter: LookupFormatter,
    {
        formatter.append_title(format!(
            "Found {} different definitions with respective information",
            defs.len()
        ));
        for (i, def) in defs.iter().take(5).enumerate() {
            formatter.visit_syn_ant(i, def)
        }
        if defs.len() > 5 {
            formatter.append_link(formatter.link_provider().syn_ant_link(&term))
        }

        formatter.build().map_err(|err| {
            log::error!("Failed to construct a response: {:?}", err);
            LookupError::FailedResponseBuilder
        })
    }
}

impl HandlerOwner for ThesaurusLookup {
    fn handler<Bot>() -> CommandHandler
    where
        Bot: LookupBot + Clone + Send + Sync + 'static,
    {
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
