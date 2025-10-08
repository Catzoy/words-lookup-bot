use crate::bloc::common::HandlerOwner;
use crate::commands::{CommandHandler, MessageCommands};
use crate::format::ToEscaped;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{Message, Requester};
use teloxide::types::ParseMode;
use teloxide::utils::command::BotCommands;
use teloxide::Bot;

pub struct HelpOwner;

impl HelpOwner {
    async fn send_help(bot: Bot, message: Message) -> anyhow::Result<()> {
        let msg = MessageCommands::descriptions().to_string().to_escaped();
        bot.send_message(message.chat.id, msg)
            .parse_mode(ParseMode::MarkdownV2)
            .await?;
        Ok(())
    }
}
impl HandlerOwner for HelpOwner {
    fn handler() -> CommandHandler {
        teloxide::dptree::case![MessageCommands::Help].endpoint(Self::send_help)
    }
}
