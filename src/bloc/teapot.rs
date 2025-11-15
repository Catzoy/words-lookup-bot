use crate::bloc::common::HandlerOwner;
use crate::bot::LookupBotX;
use crate::commands::CommandHandler;
use teloxide::dptree::entry;
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
    fn handler<Bot>() -> CommandHandler
    where
        Bot: LookupBotX + Clone + Send + Sync + 'static,
    {
        entry().endpoint(Self::send_teapot)
    }
}
