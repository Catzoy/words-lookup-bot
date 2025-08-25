use crate::{
    commands::{BotExt, CommandHandler, FullMessageFormatter, MessageCommands},
    formatting::LookupFormatter,
    stands4::{
        AbbreviationDefinition,
        Stands4Client,
        VecAbbreviationsExt,
        WordDefinition,
    },
};
use std::string::FromUtf8Error;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{Message, ParseMode},
    Bot,
};

fn word_link(word: &str) -> String {
    format!("https://www.definitions.net/definition/{}", word)
}

fn abbr_link(word: &str) -> String {
    format!("https://www.abbreviations.com/{}", word)
}

fn compose_word_defs(word: &str, defs: Vec<WordDefinition>) -> Result<String, FromUtf8Error> {
    let mut formatter = FullMessageFormatter::default();
    formatter.builder.append(format!("Found {} definitions\n\n", defs.len()));

    for (i, def) in defs.iter().take(5).enumerate() {
        formatter.visit_word(i, def);
    }
    if defs.len() > 5 {
        formatter.append_link(word_link(word))
    }
    formatter.build()
}

fn compose_abbr_defs(word: &str, defs: Vec<AbbreviationDefinition>) -> Result<String, FromUtf8Error> {
    let mut formatter = FullMessageFormatter::default();
    formatter.builder.append(format!("Found {} definitions\n\n", defs.len()));

    let categorized = defs.categorized();
    for (i, (category, defs)) in categorized.iter().take(5).enumerate() {
        formatter.visit_abbreviations(i, category, defs);
    }
    if categorized.len() > 5 {
        formatter.append_link(abbr_link(word))
    }
    formatter.build()
}

fn compose_words_with_abbrs(
    word: &str,
    words: Vec<WordDefinition>,
    abbrs: Vec<AbbreviationDefinition>,
) -> Result<String, FromUtf8Error> {
    let mut formatter = FullMessageFormatter::default();
    formatter.builder.append(format!("Found {} definitions\n\n", words.len()));

    for (i, def) in words.iter().take(5).enumerate() {
        formatter.visit_word(i, def);
    }
    if words.len() > 5 {
        formatter.append_link(word_link(word))
    }

    formatter.builder.append("And also\n");
    formatter.builder.append(format!("Found {} abbreviations\n\n", abbrs.len()));

    let categorized = abbrs.categorized();
    for (i, (category, defs)) in categorized.iter().take(5).enumerate() {
        formatter.visit_abbreviations(i, category, defs);
    }
    if categorized.len() > 5 {
        formatter.append_link(abbr_link(word))
    }

    formatter.build()
}

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

            let msg = match results {
                (Ok(words), Ok(abbrs)) =>
                    match (words.len(), abbrs.len()) {
                        (0, 0) => "Found 0 definitions".to_string(),
                        (0, _) => compose_abbr_defs(word, abbrs)?,
                        (_, 0) => compose_word_defs(word, words)?,
                        (_, _) => compose_words_with_abbrs(word, words, abbrs)?
                    }

                (Ok(words), _) =>
                    compose_word_defs(word, words)?,

                (_, Ok(abbrs)) =>
                    compose_abbr_defs(word, abbrs)?,

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