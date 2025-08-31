use crate::{
    commands::{
        drop_empty,
        BotExt,
        CommandHandler,
        FullMessageFormatter,
        MessageCommands,
    },
    format::formatter::compose_phrase_defs,
    stands4::{
        client::Stands4Client,
        Stands4LinksProvider,
    },
};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::{Message, Requester},
    types::ParseMode,
    Bot,
};

async fn phrase_lookup_handler(bot: Bot, message: Message, stands4_client: Stands4Client, phrase: String) -> anyhow::Result<()> {
    log::info!("Looking up phrase {}", phrase);

    let defs = stands4_client.search_phrase(&phrase).await?;
    let formatter = FullMessageFormatter::new(Stands4LinksProvider {});
    let msg = compose_phrase_defs(formatter, &phrase, &defs)?;
    bot.send_message(message.chat.id, msg)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;
    Ok(())
}

pub fn phrase_lookup() -> CommandHandler {
    teloxide::dptree::case![MessageCommands::PhraseLookup(args)]
        .filter_async(drop_empty)
        .endpoint(|bot: Bot, message: Message, stands4client: Stands4Client, phrase: String| async move {
            bot.with_err_response(message, move |bot, message| async {
                phrase_lookup_handler(bot, message, stands4client, phrase).await
            }).await
        })
}