use crate::commands::{BotExt, CommandHandler, MessageCommands};
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{Message, Requester};
use teloxide::types::ParseMode;
use teloxide::utils::command::BotCommands;
use teloxide::Bot;

async fn help_handler(bot: Bot, message: Message) -> anyhow::Result<()> {
    let msg = MessageCommands::descriptions().to_string();
    let msg = teloxide::utils::markdown::escape(&msg);
    bot.send_message(message.chat.id, msg)
        .parse_mode(ParseMode::MarkdownV2)
        .await?;
    Ok(())
}

pub fn help() -> CommandHandler {
    teloxide::dptree::case![MessageCommands::Help]
        .endpoint(|bot: Bot, message: Message| async move {
            bot.with_err_response(message, help_handler).await
        })
}