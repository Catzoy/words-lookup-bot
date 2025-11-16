use crate::bloc::common::HandlerOwner;
use crate::bot::LookupBot;
use crate::commands::CommandHandler;
use teloxide::dptree::entry;
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
    fn handler<Bot>() -> CommandHandler
    where
        Bot: LookupBot + Clone + Send + Sync + 'static,
    {
        entry().endpoint(Self::send_unknown)
    }
}
