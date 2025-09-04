use crate::{
    format::compose_word_with_abbrs_determined,
    inlines::{drop_empty, formatting::InlineFormatter, InlineHandler, QueryCommands},
    stands4::Stands4Client,
};
use teloxide::{
    prelude::{InlineQuery, Requester},
    Bot,
};

async fn word_lookup_handler(
    bot: Bot,
    query: InlineQuery,
    stands4_client: Stands4Client,
    word: String,
) -> anyhow::Result<()> {
    log::info!("Looking up word {}", word);

    let results = futures::future::join(
        stands4_client.search_word(&word),
        stands4_client.search_abbreviation(&word),
    )
    .await;

    let formatter = InlineFormatter::default();
    let msg = compose_word_with_abbrs_determined(formatter, &word, &results, || vec![])?;

    bot.answer_inline_query(query.id, msg).await?;
    Ok(())
}
pub fn word_lookup() -> InlineHandler {
    teloxide::dptree::case![QueryCommands::WordLookup(word)]
        .filter_async(drop_empty)
        .endpoint(word_lookup_handler)
}
