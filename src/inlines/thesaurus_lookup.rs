use crate::{
    format::compose_syn_ant_defs,
    inlines::{drop_empty, formatting::InlineFormatter, InlineHandler, QueryCommands},
    stands4::Stands4Client,
};
use teloxide::{
    prelude::{InlineQuery, Requester},
    Bot,
};

async fn thesaurus_lookup_handler(
    bot: Bot,
    query: InlineQuery,
    stands4_client: Stands4Client,
    term: String,
) -> anyhow::Result<()> {
    log::info!("Looking up word {}", term);

    let results = stands4_client.search_syn_ant(&term).await?;

    let formatter = InlineFormatter::default();
    let msg = compose_syn_ant_defs(formatter, &term, &results)?;

    bot.answer_inline_query(query.id, msg).await?;
    Ok(())
}
pub fn thesaurus_lookup() -> InlineHandler {
    teloxide::dptree::case![QueryCommands::ThesaurusLookup(term)]
        .filter_async(drop_empty)
        .endpoint(thesaurus_lookup_handler)
}
