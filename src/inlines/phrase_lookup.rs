use crate::format::formatter::compose_phrase_defs;
use crate::{
    inlines::{formatting::InlineFormatter, inlines::drop_empty, InlineHandler, QueryCommands},
    stands4::Stands4Client,
};
use teloxide::{
    prelude::{InlineQuery, Requester},
    Bot,
};

async fn phrase_lookup_handler(
    bot: Bot,
    query: InlineQuery,
    stands4_client: Stands4Client,
    phrase: String,
) -> anyhow::Result<()> {
    log::info!("Looking up phrase {}", phrase);

    let defs = stands4_client.search_phrase(phrase.as_str()).await?;
    let formatter = InlineFormatter::default();
    let msg = compose_phrase_defs(formatter, phrase.as_str(), &defs)?;
    bot.answer_inline_query(query.id, msg).await?;
    Ok(())
}
pub fn phrase_lookup() -> InlineHandler {
    teloxide::dptree::case![QueryCommands::PhraseLookup(phrase)]
        .filter_async(drop_empty)
        .endpoint(phrase_lookup_handler)
}
