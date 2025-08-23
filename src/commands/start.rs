use crate::commands::{BotExt, CommandHandler, MessageCommands};
use teloxide::prelude::Requester;
use teloxide::types::Message;
use teloxide::Bot;

async fn start_handler(bot: Bot, message: Message) -> anyhow::Result<()> {
    bot.send_message(
        message.chat.id,
        "Hi!\n\
        I'm a bot that can look up words and phrases.\n\
        Simply send me a message and I'll search for the definition of the text."
            .to_string(),
    ).await?;
    Ok(())
}

pub fn start() -> CommandHandler {
    teloxide::dptree::case![MessageCommands::Start]
        .endpoint(|bot: Bot, message: Message| async move {
            bot.with_err_response(message, start_handler).await
        })
}