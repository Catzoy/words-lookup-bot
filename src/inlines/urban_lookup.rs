use crate::{
    format::formatter::compose_urban_defs,
    inlines::{drop_empty, formatting::InlineFormatter, InlineHandler, QueryCommands},
    stands4::LinksProvider,
    urban::UrbanDictionaryClient,
};
use teloxide::{
    prelude::{InlineQuery, Requester},
    Bot,
};

async fn urban_lookup_handler(
    bot: Bot,
    query: InlineQuery,
    client: UrbanDictionaryClient,
    term: String,
) -> anyhow::Result<()> {
    log::info!("Looking up word {}", term);

    let defs = client.search_term(&term).await?;
    let formatter = InlineFormatter::default();
    let msg = compose_urban_defs(formatter, &term, &defs)?;

    bot.answer_inline_query(query.id, msg).await?;
    Ok(())
}
pub fn urban_lookup() -> InlineHandler {
    teloxide::dptree::case![QueryCommands::UrbanLookup(word)]
        .filter_async(drop_empty)
        .endpoint(urban_lookup_handler)
}
