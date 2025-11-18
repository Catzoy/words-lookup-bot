use crate::bloc::common::{CommandHandler, LookupError};
use crate::bot::LookupBot;
use crate::format::LookupFormatter;
use crate::stands4::{Stands4Client, SynAntDefinitions};
use teloxide::dptree::entry;

pub trait ThesaurusLookupBot {}
pub trait ThesaurusLookupHandler {
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
