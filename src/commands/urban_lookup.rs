use crate::{
    commands::{drop_empty, BotExt, CommandHandler, FullMessageFormatter, MessageCommands},
    format::compose_urban_defs,
    urban::UrbanDictionaryClient,
};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{Message, ParseMode},
    Bot,
};

async fn urban_lookup_handler(
    bot: Bot,
    message: Message,
    client: UrbanDictionaryClient,
    term: String,
) -> anyhow::Result<()> {
    log::info!("(U) Looking up term {}", term);

    let definitions = client.search_term(&term).await?;
    let formatter = FullMessageFormatter::default();
    let msg = compose_urban_defs(formatter, &term, &definitions)?;

    bot.send_message(message.chat.id, msg)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;
    Ok(())
}

pub fn urban_lookup() -> CommandHandler {
    teloxide::dptree::case![MessageCommands::Urban(args)]
        .filter_async(drop_empty)
        .endpoint(
            |term: String, bot: Bot, message: Message, client: UrbanDictionaryClient| async move {
                bot.with_err_response(message, move |bot, message| async {
                    urban_lookup_handler(bot, message, client, term).await
                })
                .await
            },
        )
}
