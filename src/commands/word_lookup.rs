use crate::{
    commands::{BotExt, CommandHandler, FullMessageFormatter, MessageCommands},
    format::formatter::{compose_abbr_defs, compose_word_defs, compose_words_with_abbrs},
    stands4::Stands4LinksProvider,
    stands4::{
        Stands4Client,
        VecAbbreviationsExt,
        WordDefinition,
    },
};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{Message, ParseMode},
    Bot,
};

async fn word_lookup_handler(bot: Bot, message: Message, stands4_client: Stands4Client, word: String) -> anyhow::Result<()> {
    match word.as_str() {
        "" => {
            bot.send_message(
                message.chat.id,
                "You need to specify a word to look up, like so: `\\word cookies`",
            )
                .parse_mode(ParseMode::MarkdownV2)
                .await?;
        }
        word => {
            log::info!("Looking up word {}", word);

            let results = futures::future::join(
                stands4_client.search_word(word),
                stands4_client.search_abbreviation(word),
            ).await;

            let formatter = FullMessageFormatter::new(Stands4LinksProvider {});

            let msg = match results {
                (Ok(words), Ok(abbrs)) =>
                    match (words.len(), abbrs.len()) {
                        (0, 0) => "Found 0 definitions".to_string(),
                        (0, _) => compose_abbr_defs(formatter, word, abbrs)?,
                        (_, 0) => compose_word_defs(formatter, word, words)?,
                        (_, _) => compose_words_with_abbrs(formatter, word, words, abbrs)?
                    }

                (Ok(words), _) =>
                    compose_word_defs(formatter, word, words)?,

                (_, Ok(abbrs)) =>
                    compose_abbr_defs(formatter, word, abbrs)?,

                (Err(_), Err(_)) =>
                    "Found 0 definitions".to_string(),
            };

            bot.send_message(message.chat.id, msg)
                .parse_mode(ParseMode::MarkdownV2)
                .await?;
        }
    }
    Ok(())
}
pub fn word_lookup() -> CommandHandler {
    teloxide::dptree::case![MessageCommands::WordLookup(args)]
        .endpoint(|word: String, bot: Bot, message: Message, stands4_client: Stands4Client| async move {
            bot.with_err_response(message, move |bot, message| async {
                word_lookup_handler(bot, message, stands4_client, word).await
            }).await
        })
}