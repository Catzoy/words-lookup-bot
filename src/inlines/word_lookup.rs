use crate::format::formatter::{compose_abbr_defs, compose_word_defs, compose_words_with_abbrs};
use crate::stands4::Stands4LinksProvider;
use crate::{
    inlines::{
        drop_empty,
        formatting::InlineFormatter,
        InlineHandler,
        QueryCommands,
    },
    stands4::Stands4Client,
};
use teloxide::{
    prelude::{InlineQuery, Requester},
    types::InlineQueryResult,
    Bot,
};

fn empty_result() -> Vec<InlineQueryResult> {
    vec![]
}
pub fn word_lookup() -> InlineHandler {
    teloxide::dptree::case![QueryCommands::WordLookup(word)]
        .filter_async(drop_empty)
        .endpoint(|bot: Bot, stands4_client: Stands4Client, query: InlineQuery, word: String| async move {
            log::info!("Looking up word {}", word);
            let word = word.as_str();

            let results = futures::future::join(
                stands4_client.search_word(word),
                stands4_client.search_abbreviation(word),
            ).await;

            let formatter = InlineFormatter::new(Stands4LinksProvider {});
            let msg = match results {
                (Ok(words), Ok(abbrs)) =>
                    match (words.len(), abbrs.len()) {
                        (0, 0) => empty_result(),
                        (0, _) => compose_abbr_defs(formatter, word, abbrs)?,
                        (_, 0) => compose_word_defs(formatter, word, words)?,
                        (_, _) => compose_words_with_abbrs(formatter, word, words, abbrs)?
                    }

                (Ok(words), _) =>
                    compose_word_defs(formatter, word, words)?,

                (_, Ok(abbrs)) =>
                    compose_abbr_defs(formatter, word, abbrs)?,

                (Err(_), Err(_)) =>
                    empty_result(),
            };

            bot.answer_inline_query(query.id, msg).await?;
            Ok(())
        })
}