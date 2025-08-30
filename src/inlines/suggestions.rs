use crate::commands::FullMessageFormatter;
use crate::format::formatter::LookupFormatter;
use crate::wordle::WordleAnswer;
use crate::{
    commands::wordle::ensure_wordle_answer,
    format::formatter::compose_word_defs,
    inlines::inlines::{InlineHandler, QueryCommands},
    stands4::Stands4LinksProvider,
    wordle::cache::WordleCache,
};
use teloxide::types::{InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText};
use teloxide::{
    prelude::Requester,
    types::InlineQuery,
    Bot,
};

pub fn suggestions() -> InlineHandler {
    teloxide::dptree::case![QueryCommands::Suggestions]
        .map_async(ensure_wordle_answer)
        .endpoint(|bot: Bot, query: InlineQuery, cache: WordleCache| async move {
            let msg = cache.with_answer(|it| {
                let WordleAnswer { solution, editor, days_since_launch } = &it.answer;
                let mut formatter = FullMessageFormatter::new(Stands4LinksProvider {});
                let wordle_title = format!("#{} WORDLE solution:\n{}, by {}", days_since_launch, solution, editor);
                formatter.append_title(wordle_title);
                compose_word_defs(formatter, &it.answer.solution, &it.definitions)
            }).await??;
            let msg = InputMessageContent::Text(
                InputMessageContentText::new(msg),
            );
            let answer = InlineQueryResult::Article(
                InlineQueryResultArticle::new(
                    "wordle_answer",
                    "Send definition of today's wordle answer!",
                    msg,
                )
            );

            bot.answer_inline_query(query.id, vec![answer]).await?;
            Ok(())
        })
}