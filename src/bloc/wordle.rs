use crate::bloc::common::{CommandHandler, LookupError};
use crate::bloc::word_lookup::WordLookupFormatter;
use crate::bot::{LookupBot, LookupBotX};
use crate::format::LookupFormatter;
use crate::wordle::WordleDayAnswer;
use crate::wordle::cache::WordleCache;
use teloxide::dptree::entry;

pub trait WordleBot<Response> {
    fn wordle_error_response() -> Response;
}

pub trait WordleHandler {
    /// Obtain a fresh WordleDayAnswer from the provided cache.
    ///
    /// Attempts to fetch a fresh answer via the cache and maps any underlying retrieval
    /// error to `LookupError::FailedRequest`.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn run_example() {
    /// let cache: WordleCache = unimplemented!(); // provide a real cache in real code
    /// let result = ensure_wordle_answer(cache).await;
    /// match result {
    ///     Ok(answer) => { /* use `answer` */ }
    ///     Err(LookupError::FailedRequest) => { /* handle failed retrieval */ }
    /// }
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// `Ok(WordleDayAnswer)` with the fresh answer, `Err(LookupError::FailedRequest)` if retrieval failed.
    async fn ensure_wordle_answer(mut cache: WordleCache) -> Result<WordleDayAnswer, LookupError> {
        cache.require_fresh_answer().await.map_err(|e| {
            log::error!("Couldn't retrieve wordle answer: {:?}", e);
            LookupError::FailedRequest
        })
    }

    fn retrieve_or_failed_cache(
        &self,
        answer: Result<WordleDayAnswer, LookupError>,
    ) -> impl Future<Output = Option<WordleDayAnswer>>;

    fn wordle_handler() -> CommandHandler;
}

trait WordleFormatter<Value> {
    fn compose_wordle_response(self, answer: WordleDayAnswer) -> Result<Value, LookupError>;
}

impl<Formatter> WordleFormatter<Formatter::Value> for Formatter
where
    Formatter: LookupFormatter,
{
    /// Compose a formatted response value for a Wordle day answer.
    ///
    /// Appends a title containing the day's solution (uppercased) and then delegates
    /// to the formatter's `compose_word_defs` to build the rest of the response.
    ///
    /// # Returns
    ///
    /// `Ok` with the formatter's output value on success, `Err(LookupError::FailedResponseBuilder)`
    /// if the response could not be built.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // `formatter` must implement `LookupFormatter`.
    /// let result = formatter.compose_wordle_response(answer);
    /// assert!(result.is_ok());
    /// ```
    fn compose_wordle_response(
        mut self,
        WordleDayAnswer {
            answer,
            definitions,
            ..
        }: WordleDayAnswer,
    ) -> Result<Formatter::Value, LookupError> {
        self.append_title(format!(
            "Today's answer: `{}`",
            answer.solution.to_uppercase()
        ));
        self.compose_word_defs(&answer.solution, &definitions)
            .map_err(|err| {
                log::error!("Failed to build wordle response {:?}", err);
                LookupError::FailedResponseBuilder
            })
    }
}

impl<Bot, Formatter> WordleHandler for Bot
where
    Bot: WordleBot<Bot::Response> + LookupBot<Formatter = Formatter> + Send + Sync + 'static,
    Formatter: LookupFormatter<Value = Bot::Response>,
{
    /// Attempts to extract the day's Wordle answer from a cache lookup result, sending an error response when the lookup failed.
    ///
    /// If `answer` is `Ok`, returns `Some(WordleDayAnswer)`. If `answer` is `Err`, logs the failure, attempts to send the bot's configured wordle error response, logs any send error, and returns `None`.
    ///
    /// # Parameters
    /// - `answer`: The result of a cache lookup for the day's Wordle answer.
    ///
    /// # Returns
    /// `Some(WordleDayAnswer)` when `answer` is `Ok`, `None` when `answer` is `Err` after attempting to notify users.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // given a `bot` that implements `WordleHandler`
    /// let present = bot.retrieve_or_failed_cache(Ok(day_answer)).await;
    /// assert!(present.is_some());
    /// ```
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

    /// Builds the command handler pipeline that retrieves the day's Wordle answer, formats it, and sends the response.
    ///
    /// The pipeline:
    /// - obtains or computes the `WordleDayAnswer`;
    /// - short-circuits on lookup failures while attempting to send an error response;
    /// - formats a successful answer into the bot's response type, mapping formatting errors;
    /// - sends the final response via the bot's `respond` method.
    ///
    /// # Examples
    ///
    /// ```
    /// // Obtain a CommandHandler wired for the Wordle flow.
    /// let handler = Bot::wordle_handler();
    /// ```
    fn wordle_handler() -> CommandHandler {
        entry()
            .map_async(Self::ensure_wordle_answer)
            .filter_map_async(
                |bot: Bot, answer: Result<WordleDayAnswer, LookupError>| async move {
                    bot.retrieve_or_failed_cache(answer).await
                },
            )
            .map(move |bot: Bot, answer: WordleDayAnswer| {
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