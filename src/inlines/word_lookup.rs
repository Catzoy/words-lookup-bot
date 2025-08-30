use crate::{
    format::formatter::{compose_word_defs, compose_words_with_abbrs},
    format::word_with_abbr_ext::compose_word_with_abbrs_determined,
    inlines::{
        drop_empty,
        formatting::InlineFormatter,
        InlineHandler,
        QueryCommands,
    },
    stands4::{
        Stands4Client,
        Stands4LinksProvider,
    },
};
use teloxide::{
    prelude::{InlineQuery, Requester},
    Bot,
};

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
            let msg = compose_word_with_abbrs_determined(
                formatter, word, &results, || vec![],
            )?;

            bot.answer_inline_query(query.id, msg).await?;
            Ok(())
        })
}