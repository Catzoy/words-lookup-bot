use crate::commands::{BotExt, CommandHandler, MessageCommands};
use teloxide::prelude::{Message, Requester};
use teloxide::Bot;

async fn unknown_command_handler(bot: Bot, message: Message) -> anyhow::Result<()> {
    bot.send_message(
        message.chat.id,
        "I don't know that command, sorry.",
    ).await?;
    Ok(())
}

pub fn unknown() -> CommandHandler {
    teloxide::dptree::case![MessageCommands::Unknown]
        .endpoint(|bot: Bot, message: Message| async move {
            bot.with_err_response(message, unknown_command_handler).await
        })
}