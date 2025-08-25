use crate::stands4::VecAbbreviationsExt;
use crate::{
    formatting::LookupFormatter,
    inlines::drop_empty,
    inlines::formatting::InlineFormatter,
    inlines::InlineHandler,
    inlines::QueryCommands,
    stands4::{AbbreviationDefinition, Stands4Client, WordDefinition},
};
use teloxide::{
    payloads::AnswerInlineQuerySetters,
    prelude::{InlineQuery, Requester},
    types::InlineQueryResult,
    Bot,
};

fn compose_word_defs(defs: Vec<WordDefinition>) -> Vec<InlineQueryResult> {
    let mut formatter = InlineFormatter::default();

    for (i, def) in defs.iter().take(5).enumerate() {
        formatter.visit_word(i, def);
    }
    formatter.build()
}

fn compose_abbr_defs(defs: Vec<AbbreviationDefinition>) -> Vec<InlineQueryResult> {
    let mut formatter = InlineFormatter::default();

    let categorized = defs.categorized();
    for (i, (category, defs)) in categorized.iter().take(5).enumerate() {
        formatter.visit_abbreviations(i, category, defs);
    }
    formatter.build()
}

fn compose_words_with_abbrs(
    words: Vec<WordDefinition>,
    abbrs: Vec<AbbreviationDefinition>,
) -> Vec<InlineQueryResult> {
    let mut formatter = InlineFormatter::default();

    for (i, def) in words.iter().take(5).enumerate() {
        formatter.visit_word(i, def);
    }

    let categorized = abbrs.categorized();
    for (i, (category, defs)) in categorized.iter().take(5).enumerate() {
        formatter.visit_abbreviations(i, category, defs);
    }

    formatter.build()
}
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

            let msg = match results {
                (Ok(words), Ok(abbrs)) =>
                    match (words.len(), abbrs.len()) {
                        (0, 0) => empty_result(),
                        (0, _) => compose_abbr_defs(abbrs),
                        (_, 0) => compose_word_defs(words),
                        (_, _) => compose_words_with_abbrs(words, abbrs)
                    }

                (Ok(words), _) =>
                    compose_word_defs(words),

                (_, Ok(abbrs)) =>
                    compose_abbr_defs(abbrs),

                (Err(_), Err(_)) =>
                    empty_result(),
            };

            bot.answer_inline_query(query.id, msg)
                .cache_time(0)
                .await?;
            Ok(())
        })
}