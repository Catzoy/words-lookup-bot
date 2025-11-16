use crate::bloc::common::{HandlerOwner, LookupError};
use crate::bloc::word_lookup::WordLookupFormatter;
use crate::bot::LookupBot;
use crate::wordle::WordleDayAnswer;
use crate::{
    commands::{CommandHandler, FullMessageFormatter},
    wordle::cache::WordleCache,
};
use teloxide::dptree::entry;
use teloxide::{
    prelude::{Message, Requester},
    Bot,
};

#[derive(Clone, Debug)]
pub struct WordleLookup;
impl WordleLookup {
    async fn ensure_wordle_answer(mut cache: WordleCache) -> Result<WordleDayAnswer, LookupError> {
        cache.require_fresh_answer().await.map_err(|e| {
            log::error!("Couldn't retrieve wordle answer: {:?}", e);
            LookupError::FailedRequest
        })
    }
    async fn retrieve_or_failed_cache(
        bot: Bot,
        message: Message,
        answer: Result<WordleDayAnswer, LookupError>,
    ) -> Option<WordleDayAnswer> {
        match answer {
            Ok(latest) => Some(latest),
            Err(err) => {
                log::error!("Failed to get today's wordle, err: {:?}", err);
                let text = "Could not get today's wordle, sorry, try again in an hour or so.";
                let resp = bot.send_message(message.chat.id, text).await;
                if let Err(err) = resp {
                    log::error!("Failed to respond with err: {:?}", err);
                }
                None
            }
        }
    }
    fn compose_response(answer: WordleDayAnswer) -> Result<String, LookupError> {
        FullMessageFormatter::default()
            .compose_word_defs(&answer.answer.solution, &answer.definitions)
            .map_err(|err| {
                log::error!("Failed to build wordle response {:?}", err);
                LookupError::FailedResponseBuilder
            })
    }
}

impl HandlerOwner for WordleLookup {
    fn handler<Bot>() -> CommandHandler
    where
        Bot: LookupBot + Clone + Send + Sync + 'static,
    {
        entry()
            .map_async(Self::ensure_wordle_answer)
            .filter_map_async(Self::retrieve_or_failed_cache)
            .map(Self::compose_response)
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
