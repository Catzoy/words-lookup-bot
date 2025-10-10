use crate::bloc::common::HandlerOwner;
use crate::commands::{CommandHandler, MessageCommands};
use teloxide::prelude::{Message, Requester};
use teloxide::Bot;

pub struct UnknownOwner {}
impl UnknownOwner {
    async fn send_unknown(bot: Bot, message: Message) -> anyhow::Result<()> {
        bot.send_message(message.chat.id, "I don't know that command, sorry.")
            .await?;
        Ok(())
    }
}

impl HandlerOwner for UnknownOwner {
    fn handler() -> CommandHandler {
        teloxide::dptree::case![MessageCommands::Unknown].endpoint(Self::send_unknown)
    }
}
