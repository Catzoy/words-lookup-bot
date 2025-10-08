use crate::bloc::common::HandlerOwner;
use crate::commands::CommandHandler;
use crate::format::ToEscaped;
use crate::{
    commands::{FullMessageFormatter, MessageCommands},
    format::{compose_word_defs, LookupFormatter},
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

pub struct SuggestionsOwner;
impl SuggestionsOwner {
    async fn ensure_wordle_answer(mut cache: WordleCache) -> anyhow::Result<()> {
        let fresh_result = cache.require_fresh_answer().await;
        if let Err(err) = fresh_result {
            log::error!("Failed to get today's wordle, err: {}", err);
        }
        Ok(())
    }

    async fn suggestions_handler(
        bot: Bot,
        query: InlineQuery,
        mut cache: WordleCache,
    ) -> anyhow::Result<()> {
        let mut answers = Vec::<InlineQueryResult>::new();
        let help = {
            let text = "Continue writing to look up a word or a phrase";
            let msg = MessageCommands::descriptions().to_string();
            let msg = InputMessageContentText::new(msg.to_escaped());
            let msg = InputMessageContent::Text(msg);
            let msg = InlineQueryResultArticle::new("help", text, msg);
            InlineQueryResult::Article(msg)
        };
        answers.push(help);

        let urban = {
            let text = "Or write \"u.PHRASE\" to look up in UrbanDictionary";
            let msg = InputMessageContentText::new(
                "Write @WordsLookupBot \"u.PHRASE\" to look up in UrbanDictionary",
            );
            let msg = InputMessageContent::Text(msg);
            let msg = InlineQueryResultArticle::new("urban", text, msg);
            InlineQueryResult::Article(msg)
        };
        answers.push(urban);

        let syn_ant = {
            let text = "Or write \"sa.WORD\" to look up synonyms & antonyms";
            let msg = InputMessageContentText::new(
                "Write @WordsLookupBot \"sa.WORD\" to look up synonyms & antonyms in the Thesaurus",
            );
            let msg = InputMessageContent::Text(msg);
            let msg = InlineQueryResultArticle::new("syn_ant", text, msg);
            InlineQueryResult::Article(msg)
        };
        answers.push(syn_ant);

        let answer = cache.require_fresh_answer().await?;
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
        let msg = compose_word_defs(formatter, &solution, &answer.definitions);
        if let Ok(wordle_message) = msg {
            let title = "Send definition of today's wordle answer!";
            let msg =
                InputMessageContentText::new(wordle_message).parse_mode(ParseMode::MarkdownV2);
            let msg = InputMessageContent::Text(msg);
            let article = InlineQueryResultArticle::new("wordle", title, msg);
            let answer = InlineQueryResult::Article(article);
            answers.push(answer);
        }

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
