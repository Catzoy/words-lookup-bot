use crate::bloc::common::HandlerOwner;
use crate::commands::{CommandHandler, MessageCommands};
use teloxide::prelude::{Message, Requester};
use teloxide::Bot;

pub struct TeapotOwner;
impl TeapotOwner {
    async fn send_teapot(bot: Bot, message: Message) -> anyhow::Result<()> {
        bot.send_message(message.chat.id, "I'm a teapot").await?;
        Ok(())
    }
}

impl HandlerOwner for TeapotOwner {
    fn handler() -> CommandHandler {
        teloxide::dptree::case![MessageCommands::Teapot].endpoint(Self::send_teapot)
    }
}
