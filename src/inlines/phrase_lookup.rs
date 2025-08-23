use crate::{
    inlines::{InlineHandler, QueryCommands},
    stands4::client::Stands4Client,
};
use teloxide::{
    prelude::InlineQuery,
    prelude::Requester,
    types::{InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText},
    Bot,
};

pub fn phrase_lookup() -> InlineHandler {
    teloxide::dptree::case![QueryCommands::PhraseLookup(phrase)]
        .endpoint(|bot: Bot, stands4_client: Stands4Client, query: InlineQuery, phrase: String| async move {
            match phrase.as_str() {
                "" => {
                    Ok(())
                }
                phrase => {
                    log::info!("Looking up phrase {}", phrase);

                    // let defs = stands4_client.search_phrase(phrase).await?;
                    // let msg = crate::commands::phrase_lookup::compose_phrase_defs(phrase, defs)?;
                    // bot.send_message(message.chat.id, msg)
                    //     .parse_mode(ParseMode::MarkdownV2)
                    //     .await?;

                    let msg = InlineQueryResult::Article(
                        InlineQueryResultArticle::new(
                            "phrase",
                            format!("Look up phrase {}", phrase),
                            InputMessageContent::Text(
                                InputMessageContentText::new("Mock response for the phrase")
                            ),
                        ),
                    );
                    bot.answer_inline_query(query.id, vec![msg]).await?;
                    Ok(())
                }
            }
        })
}