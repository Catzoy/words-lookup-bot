use crate::commands::{BotExt, CommandHandler, MessageCommands};
use teloxide::prelude::{Message, Requester};
use teloxide::Bot;

async fn teapot_handler(bot: Bot, message: Message) -> anyhow::Result<()> {
    bot.send_message(message.chat.id, "I'm a teapot").await?;
    Ok(())
}

pub fn teapot() -> CommandHandler {
    teloxide::dptree::case![MessageCommands::Teapot]
        .endpoint(|bot: Bot, message: Message| async move {
            bot.with_err_response(message, teapot_handler).await
        })
}