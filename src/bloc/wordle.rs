use crate::bloc::common::{CommandHandler, LookupError};
use crate::bloc::word_lookup::WordLookupFormatter;
use crate::bot::LookupBot;
use crate::format::LookupFormatter;
use crate::wordle::cache::WordleCache;
use crate::wordle::WordleDayAnswer;
use shuttle_runtime::async_trait;
use teloxide::dptree::entry;

pub trait WordleBot<Response> {
    fn wordle_error_response() -> Response;
}

#[async_trait]
pub trait WordleHandler {
    async fn ensure_wordle_answer(mut cache: WordleCache) -> Result<WordleDayAnswer, LookupError> {
        cache.require_fresh_answer().await.map_err(|e| {
            log::error!("Couldn't retrieve wordle answer: {:?}", e);
            LookupError::FailedRequest
        })
    }

    async fn retrieve_or_failed_cache(
        &self,
        answer: Result<WordleDayAnswer, LookupError>,
    ) -> Option<WordleDayAnswer>;

    fn wordle_handler() -> CommandHandler;
}

trait WordleFormatter<Value> {
    fn compose_wordle_response(self, answer: WordleDayAnswer) -> Result<Value, LookupError>;
}

impl<Formatter> WordleFormatter<Formatter::Value> for Formatter
where
    Formatter: LookupFormatter,
{
    fn compose_wordle_response(
        self,
        answer: WordleDayAnswer,
    ) -> Result<Formatter::Value, LookupError> {
        self.compose_word_defs(&answer.answer.solution, &answer.definitions)
            .map_err(|err| {
                log::error!("Failed to build wordle response {:?}", err);
                LookupError::FailedResponseBuilder
            })
    }
}

#[async_trait]
impl<Bot> WordleHandler for Bot
where
    Bot: LookupBot + WordleBot<Bot::Response> + Send + Sync + 'static,
{
    async fn retrieve_or_failed_cache(
        &self,
        answer: Result<WordleDayAnswer, LookupError>,
    ) -> Option<WordleDayAnswer> {
        match answer {
            Ok(latest) => Some(latest),
            Err(err) => {
                log::error!("Failed to get today's wordle, err: {:?}", err);
                let resp = self.answer(Self::wordle_error_response()).await;
                if let Err(err) = resp {
                    log::error!("Failed to respond with err: {:?}", err);
                }
                None
            }
        }
    }

    fn wordle_handler() -> CommandHandler {
        entry()
            .map_async(Self::ensure_wordle_answer)
            .filter_map_async(
                |bot: Bot, answer: Result<WordleDayAnswer, LookupError>| async move {
                    bot.retrieve_or_failed_cache(answer).await
                },
            )
            .map(|bot: Bot, answer: WordleDayAnswer| {
                bot.formatter().compose_wordle_response(answer)
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
