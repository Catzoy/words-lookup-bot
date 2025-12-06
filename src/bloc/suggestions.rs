use crate::bloc::common::CommandHandler;
use crate::bloc::word_lookup::WordLookupFormatter;
use crate::bot::LookupBot;
use crate::format::ToEscaped;
use crate::wordle::WordleDayAnswer;
use crate::{
    commands::{FullMessageFormatter, MessageCommands},
    format::LookupFormatter,
    wordle::cache::WordleCache,
    wordle::WordleAnswer,
};
use teloxide::dptree::entry;
use teloxide::types::{
    InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText,
    ParseMode,
};
use teloxide::utils::command::BotCommands;

trait SuggestionOwner {
    fn produce(self) -> Option<InlineQueryResult>;
}

struct HelpSuggestion;
impl SuggestionOwner for HelpSuggestion {
    /// Builds an inline help suggestion that explains how to look up words or phrases.
    ///
    /// The result is an `InlineQueryResult::Article` with id `"help"`, a short prompt title,
    /// and a prepared, escaped help message suitable for sending as inline query content.
    ///
    /// # Examples
    ///
    /// ```
    /// let sugg = crate::inlines::suggestions::HelpSuggestion;
    /// let result = sugg.produce();
    /// assert!(result.is_some());
    /// ```
    ///
    /// # Returns
    ///
    /// `Some(InlineQueryResult::Article)` containing the help message.
    fn produce(self) -> Option<InlineQueryResult> {
        let text = "Continue writing to look up a word or a phrase";
        let msg = MessageCommands::descriptions().to_string();
        let msg = InputMessageContentText::new(msg.to_escaped());
        let msg = InputMessageContent::Text(msg);
        let msg = InlineQueryResultArticle::new("help", text, msg);
        Some(InlineQueryResult::Article(msg))
    }
}

struct UrbanSuggestion;
impl SuggestionOwner for UrbanSuggestion {
    /// Builds an inline query article suggesting an UrbanDictionary lookup.
    ///
    /// The returned article prompts the user to write `u.PHRASE` to look up a phrase
    /// on UrbanDictionary and is suitable for use as an `InlineQueryResult`.
    ///
    /// # Examples
    ///
    /// ```
    /// let res = UrbanSuggestion {}.produce();
    /// assert!(matches!(res, Some(InlineQueryResult::Article(_))));
    /// ```
    fn produce(self) -> Option<InlineQueryResult> {
        let text = "Or write \"u.PHRASE\" to look up in UrbanDictionary";
        let msg = InputMessageContentText::new(
            "Write @WordsLookupBot \"u.PHRASE\" to look up in UrbanDictionary",
        );
        let msg = InputMessageContent::Text(msg);
        let msg = InlineQueryResultArticle::new("urban", text, msg);
        Some(InlineQueryResult::Article(msg))
    }
}

struct ThesaurusSuggestion;
impl SuggestionOwner for ThesaurusSuggestion {
    /// Builds an inline query article that instructs the user to send `sa.WORD` to look up synonyms and antonyms.
    ///
    /// Returns `Some(InlineQueryResult::Article(_))` containing an article with id `"syn_ant"`, a short title
    /// prompting `sa.WORD`, and a text message explaining how to use the Thesaurus lookup.
    ///
    /// # Examples
    ///
    /// ```
    /// // Constructing and using the suggestion
    /// let suggestion = ThesaurusSuggestion;
    /// let result = suggestion.produce();
    /// assert!(matches!(result, Some(InlineQueryResult::Article(_))));
    /// ```
    fn produce(self) -> Option<InlineQueryResult> {
        let text = "Or write \"sa.WORD\" to look up synonyms & antonyms";
        let msg = InputMessageContentText::new(
            "Write @WordsLookupBot \"sa.WORD\" to look up synonyms & antonyms in the Thesaurus",
        );
        let msg = InputMessageContent::Text(msg);
        let msg = InlineQueryResultArticle::new("syn_ant", text, msg);
        Some(InlineQueryResult::Article(msg))
    }
}

struct WordleSuggestion {
    wordle: Option<WordleDayAnswer>,
}
impl WordleSuggestion {
    /// Builds a MarkdownV2-formatted message containing the Wordle title and its definitions.
    ///
    /// Composes a title of the form "#<day> WORDLE solution:\n<solution>, by <editor>" and appends
    /// the formatted definitions produced from the provided `WordleDayAnswer`. Returns `Some` with
    /// the composed message when definition composition succeeds, or `None` if composition fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // Construct or obtain a `WordleDayAnswer` from your application code.
    /// let answer = /* WordleDayAnswer instance */ unimplemented!();
    /// let message = crate::inlines::suggestions::WordleSuggestion::compose_message(answer);
    /// // `message` will be `Some(String)` when composition succeeds.
    /// ```
    fn compose_message(answer: WordleDayAnswer) -> Option<String> {
        let WordleAnswer {
            solution,
            editor,
            days_since_launch,
        } = answer.answer;
        let mut formatter = FullMessageFormatter::default();
        let wordle_title = format!(
            "\\#{} WORDLE solution:\n{}, by {}",
            days_since_launch, solution, editor
        );
        formatter.append_title(wordle_title);
        formatter
            .compose_word_defs(&solution, &answer.definitions)
            .ok()
    }

    /// Create an inline query article that sends the provided Wordle definition message.
    ///
    /// The returned `InlineQueryResult::Article` contains an article with the fixed title
    /// "Send definition of today's wordle answer!" and the given message formatted using
    /// MarkdownV2.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = compose_response("*Wordle* definition here".to_string());
    /// // `result` is an `InlineQueryResult::Article` ready to be returned to Telegram.
    /// ```
    fn compose_response(message: String) -> InlineQueryResult {
        let title = "Send definition of today's wordle answer!";
        let msg = InputMessageContentText::new(message).parse_mode(ParseMode::MarkdownV2);
        let msg = InputMessageContent::Text(msg);
        let article = InlineQueryResultArticle::new("wordle", title, msg);
        InlineQueryResult::Article(article)
    }
}

impl SuggestionOwner for WordleSuggestion {
    /// Create an inline query result for the current Wordle answer when one is available.
    ///
    /// The method attempts to compose a message from the stored `WordleDayAnswer` and,
    /// if successful, wraps it in an `InlineQueryResult` ready to be returned to Telegram.
    ///
    /// # Returns
    ///
    /// `Some(InlineQueryResult)` with a Wordle article if a Wordle answer exists and the message was composed successfully, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let suggestion = WordleSuggestion { wordle: None };
    /// assert!(suggestion.produce().is_none());
    /// ```
    fn produce(self) -> Option<InlineQueryResult> {
        self.wordle
            .and_then(Self::compose_message)
            .map(Self::compose_response)
    }
}

pub trait SuggestionsBot {}
pub trait SuggestionsHandler {
    /// Attempts to obtain a fresh WordleDayAnswer from the provided cache.
    ///
    /// On failure the error is logged and `None` is returned; on success returns `Some(WordleDayAnswer)`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crate::wordle::WordleCache;
    /// # async fn example(cache: WordleCache) {
    /// let answer = crate::inlines::suggestions::ensure_wordle_answer(cache).await;
    /// match answer {
    ///     Some(a) => println!("Got wordle answer for day: {:?}", a),
    ///     None => println!("No wordle answer available"),
    /// }
    /// # }
    /// ```
    async fn ensure_wordle_answer(mut cache: WordleCache) -> Option<WordleDayAnswer> {
        cache
            .require_fresh_answer()
            .await
            .inspect_err(|err| {
                log::error!("Failed to get today's wordle, err: {}", err);
            })
            .ok()
    }

    /// Send a set of inline suggestion articles in response to an inline query.
    ///
    /// Builds Help, UrbanDictionary, Thesaurus suggestions and, if available, a Wordle suggestion;
    /// filters out any missing suggestions and answers the inline query with the resulting articles.
    ///
    /// The `wordle` argument supplies an optional WordleDayAnswer; if `Some`, a Wordle suggestion will
    /// be included when it can produce a result.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the answers were successfully sent, an error otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn _example(bot: teloxide::Bot, query: teloxide::types::InlineQuery) -> anyhow::Result<()> {
    /// suggestions_handler(bot, query, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn send_suggestions(&self, wordle: Option<WordleDayAnswer>) -> anyhow::Result<()>;

    fn suggestions_handler() -> CommandHandler;
}

impl<Bot> SuggestionsHandler for Bot
where
    Bot: SuggestionsBot + LookupBot<Response = Vec<InlineQueryResult>> + Send + Sync + 'static,
{
    /// Send inline query suggestions assembled from the available suggestion owners.
    ///
    /// The handler gathers Help, Urban, Thesaurus, and (optionally) Wordle suggestions,
    /// filters out any missing entries, and forwards the collected InlineQueryResult
    /// list to the bot's inline answer responder.
    ///
    /// # Parameters
    ///
    /// - `wordle`: Optional cached WordleDayAnswer used to produce a Wordle suggestion.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, an error otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # use your_crate::bloc::suggestions::WordleDayAnswer;
    /// # async fn example<B: your_crate::bloc::suggestions::SuggestionsHandler + Sync>(bot: &B) -> Result<()> {
    /// bot.send_suggestions(None).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn send_suggestions(&self, wordle: Option<WordleDayAnswer>) -> anyhow::Result<()> {
        let suggestions = vec![
            HelpSuggestion.produce(),
            UrbanSuggestion.produce(),
            ThesaurusSuggestion.produce(),
            WordleSuggestion { wordle }.produce(),
        ];
        let answers = suggestions.into_iter().flatten().collect::<Vec<_>>();
        self.answer(answers).await?;
        Ok(())
    }

    /// Builds a CommandHandler that prepares and sends inline suggestions, supplying a fresh Wordle answer if available.
    ///
    /// The handler first resolves an optional `WordleDayAnswer` via `Self::ensure_wordle_answer`, then invokes
    /// `send_suggestions` on the bot with that optional answer.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // Obtain a handler for the bot type and register it with your dispatcher.
    /// let handler = Bot::suggestions_handler();
    /// // register `handler` with your bot framework...
    /// ```
    fn suggestions_handler() -> CommandHandler {
        entry().map_async(Self::ensure_wordle_answer).endpoint(
            |bot: Bot, wordle: Option<WordleDayAnswer>| async move {
                bot.send_suggestions(wordle).await
            },
        )
    }
}