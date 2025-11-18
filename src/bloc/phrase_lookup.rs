use crate::bloc::common::{CommandHandler, LookupError};
use crate::bot::LookupBot;
use crate::format::LookupFormatter;
use crate::stands4::{PhraseDefinition, Stands4Client};
use teloxide::dptree::entry;

pub trait PhraseLookupBot {}
pub trait PhraseLookupHandler {
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
