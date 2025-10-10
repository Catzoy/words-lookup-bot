use crate::bloc::common::HandlerOwner;
use crate::bloc::word_lookup::WordLookupFormatter;
use crate::format::ToEscaped;
use crate::wordle::WordleDayAnswer;
use crate::{
    commands::{FullMessageFormatter, MessageCommands},
    format::LookupFormatter,
    inlines::inlines::{InlineHandler, QueryCommands},
    wordle::cache::WordleCache,
    wordle::WordleAnswer,
};
use teloxide::types::{
    InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText,
    ParseMode,
};
use teloxide::utils::command::BotCommands;
use teloxide::{prelude::Requester, types::InlineQuery, Bot};

trait SuggestionOwner {
    fn produce(&self) -> Option<InlineQueryResult>;
}

struct HelpSuggestion;
impl SuggestionOwner for HelpSuggestion {
    fn produce(&self) -> Option<InlineQueryResult> {
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
    fn produce(&self) -> Option<InlineQueryResult> {
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
    fn produce(&self) -> Option<InlineQueryResult> {
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

    fn compose_response(message: String) -> InlineQueryResult {
        let title = "Send definition of today's wordle answer!";
        let msg = InputMessageContentText::new(message).parse_mode(ParseMode::MarkdownV2);
        let msg = InputMessageContent::Text(msg);
        let article = InlineQueryResultArticle::new("wordle", title, msg);
        InlineQueryResult::Article(article)
    }
}

impl SuggestionOwner for WordleSuggestion {
    fn produce(&self) -> Option<InlineQueryResult> {
        self.wordle
            .clone()
            .and_then(Self::compose_message)
            .map(Self::compose_response)
    }
}

pub struct SuggestionsOwner;
impl SuggestionsOwner {
    async fn ensure_wordle_answer(mut cache: WordleCache) -> Option<WordleDayAnswer> {
        cache
            .require_fresh_answer()
            .await
            .inspect_err(|err| {
                log::error!("Failed to get today's wordle, err: {}", err);
            })
            .ok()
    }

    async fn suggestions_handler(
        bot: Bot,
        query: InlineQuery,
        wordle: Option<WordleDayAnswer>,
    ) -> anyhow::Result<()> {
        let suggestions = vec![
            HelpSuggestion {}.produce(),
            UrbanSuggestion {}.produce(),
            ThesaurusSuggestion {}.produce(),
            WordleSuggestion { wordle }.produce(),
        ];
        let answers = suggestions.into_iter().flatten().collect::<Vec<_>>();
        bot.answer_inline_query(query.id, answers).await?;
        Ok(())
    }
}

impl HandlerOwner for SuggestionsOwner {
    fn handler() -> InlineHandler {
        teloxide::dptree::case![QueryCommands::Suggestions]
            .map_async(Self::ensure_wordle_answer)
            .endpoint(Self::suggestions_handler)
    }
}
