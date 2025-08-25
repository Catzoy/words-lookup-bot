use crate::{
    formatting::LookupFormatter,
    inlines::formatting::InlineFormatter,
    inlines::inlines::drop_empty,
    inlines::{InlineHandler, QueryCommands},
    stands4::{PhraseDefinition, Stands4Client},
};
use teloxide::{
    prelude::InlineQuery,
    prelude::Requester,
    types::InlineQueryResult,
    Bot,
};

fn compose_phrase_defs(defs: Vec<PhraseDefinition>) -> Vec<InlineQueryResult> {
    let mut formatter = InlineFormatter::default();

    for (i, def) in defs.iter().take(5).enumerate() {
        formatter.visit_phrase(i, def);
    }

    formatter.build()
}

pub fn phrase_lookup() -> InlineHandler {
    teloxide::dptree::case![QueryCommands::PhraseLookup(phrase)]
        .filter_async(drop_empty)
        .endpoint(|bot: Bot, stands4_client: Stands4Client, query: InlineQuery, phrase: String| async move {
            log::info!("Looking up phrase {}", phrase);

            let defs = stands4_client.search_phrase(phrase.as_str()).await?;
            let msg = compose_phrase_defs(defs);
            bot.answer_inline_query(query.id, msg).await?;
            Ok(())
        })
}