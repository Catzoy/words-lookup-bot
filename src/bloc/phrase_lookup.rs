use crate::bloc::common::{HandlerOwner, LookupError};
use crate::bot::LookupBot;
use crate::commands::CommandHandler;
use crate::format::LookupFormatter;
use crate::stands4::{PhraseDefinition, Stands4Client};
use teloxide::dptree::entry;

#[derive(Clone, Debug)]
pub struct PhraseLookup;
impl PhraseLookup {
    async fn get_definitions(
        client: Stands4Client,
        phrase: String,
    ) -> Result<Vec<PhraseDefinition>, LookupError> {
        client.search_phrase(phrase.as_str()).await.map_err(|e| {
            log::error!("phrase search error: {:?}", e);
            LookupError::FailedRequest
        })
    }

    fn compose_response<Formatter>(
        phrase: String,
        mut formatter: Formatter,
        defs: Vec<PhraseDefinition>,
    ) -> Result<Formatter::Value, LookupError>
    where
        Formatter: LookupFormatter,
    {
        formatter.append_title(format!("Found {} definitions", defs.len()));

        for (i, def) in defs.iter().take(5).enumerate() {
            formatter.visit_phrase(i, def);
        }
        if defs.len() > 5 {
            formatter.append_link(formatter.link_provider().phrase_link(&phrase));
        }

        formatter.build().map_err(|err| {
            log::error!("Failed to construct a response: {:?}", err);
            LookupError::FailedResponseBuilder
        })
    }
}

impl HandlerOwner for PhraseLookup {
    fn handler<Bot>() -> CommandHandler
    where
        Bot: LookupBot + Clone + Send + Sync + 'static,
    {
        entry()
            .filter_async(|bot: Bot, phrase: String| async move { bot.drop_empty(phrase).await })
            .map_async(Self::get_definitions)
            .filter_map_async(
                |bot: Bot, response: Result<Vec<PhraseDefinition>, LookupError>| async move {
                    bot.ensure_request_success(response).await
                },
            )
            .map(
                |bot: Bot, phrase: String, defs: Vec<PhraseDefinition>| async move {
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
