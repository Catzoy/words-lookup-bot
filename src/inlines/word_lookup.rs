use crate::{
    inlines::InlineHandler,
    inlines::QueryCommands,
    stands4::client::Stands4Client,
};
use teloxide::types::{InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText};
use teloxide::{
    prelude::{InlineQuery, Requester},
    Bot,
};

pub fn word_lookup() -> InlineHandler {
    teloxide::dptree::case![QueryCommands::WordLookup(word)]
        .endpoint(|bot: Bot, stands4_client: Stands4Client, query: InlineQuery, word: String| async move {
            match word.as_str() {
                "" => {
                    Ok(())
                }
                word => {
                    log::info!("Looking up word {}", word);

                    // let results = futures::future::join(
                    //     stands4_client.search_word(word),
                    //     stands4_client.search_abbreviation(word),
                    // ).await;
                    //
                    // let msg = match results {
                    //     (Ok(words), Ok(abbrs)) =>
                    //         match (words.len(), abbrs.len()) {
                    //             (0, 0) => "Found 0 definitions".to_string(),
                    //             (0, _) => crate::commands::word_lookup::compose_abbr_defs(word, abbrs)?,
                    //             (_, 0) => crate::commands::word_lookup::compose_word_defs(word, words)?,
                    //             (_, _) => crate::commands::word_lookup::compose_words_with_abbrs(word, words, abbrs)?
                    //         }
                    //
                    //     (Ok(words), _) =>
                    //         crate::commands::word_lookup::compose_word_defs(word, words)?,
                    //
                    //     (_, Ok(abbrs)) =>
                    //         crate::commands::word_lookup::compose_abbr_defs(word, abbrs)?,
                    //
                    //     (Err(_), Err(_)) =>
                    //         "Found 0 definitions".to_string(),
                    // };

                    let msg = InlineQueryResult::Article(
                        InlineQueryResultArticle::new(
                            "word",
                            format!("Look up word {}", word),
                            InputMessageContent::Text(
                                InputMessageContentText::new("Mock response for the word")
                            ),
                        ),
                    );
                    bot.answer_inline_query(query.id, vec![msg]).await?;
                    Ok(())
                }
            }
        })
}