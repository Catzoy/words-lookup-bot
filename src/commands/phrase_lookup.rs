use crate::commands::{BotExt, CommandHandler, MessageCommands};
use crate::formatting::{FullMessageFormatter, LookupFormatter};
use crate::stands4::client::Stands4Client;
use crate::stands4::entities::PhraseDefinition;
use std::string::FromUtf8Error;
use teloxide::prelude::{Message, Requester};
use teloxide::Bot;

fn phrase_link(phrase: &str) -> String {
    format!(
        "https://www.phrases.com/psearch/{}",
        phrase.replace(" ", "+")
    )
}

fn compose_phrase_defs(phrase: &str, defs: Vec<PhraseDefinition>) -> Result<String, FromUtf8Error> {
    let mut formatter = FullMessageFormatter::default();
    formatter.builder.append(format!("Found {} definitions\n\n", defs.len()));

    for (i, def) in defs.iter().take(5).enumerate() {
        formatter.visit_phrase(i, def);
    }
    if defs.len() > 5 {
        formatter.append_link(phrase_link(phrase));
    }

    formatter.build()
}

async fn phrase_lookup_handler(bot: Bot, message: Message, stands4_client: Stands4Client, phrase: String) -> anyhow::Result<()> {
    match phrase.as_str() {
        "" => {
            bot.send_message(
                message.chat.id,
                "You meed to specify a phrase to look up, like so: `\\phrase buckle up`",
            ).await?;
        }
        phrase => {
            log::info!("Looking up phrase {}", phrase);

            let defs = stands4_client.search_phrase(phrase).await?;
            let msg = compose_phrase_defs(phrase, defs)?;
            bot.send_message(message.chat.id, msg).await?;
        }
    };
    Ok(())
}

pub fn phrase_lookup() -> CommandHandler {
    teloxide::dptree::case![MessageCommands::PhraseLookup(args)]
        .endpoint(|bot: Bot, message: Message, stands4client: Stands4Client, phrase: String| async move {
            bot.with_err_response(message, move |bot, message| async {
                phrase_lookup_handler(bot, message, stands4client, phrase).await
            }).await
        })
}