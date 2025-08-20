use crate::commands::command::Command;
use shuttle_runtime::async_trait;
use teloxide::prelude::{Message, Requester};
use teloxide::types::Me;
use teloxide::Bot;

pub struct UnknownCommand {}

#[async_trait]
impl Command for UnknownCommand {
    async fn handle(&self, _me: &Me, bot: &Bot, message: &Message, _args: Vec<String>) -> anyhow::Result<()> {
        bot.send_message(
            message.chat.id,
            "I don't know that command, sorry.",
        ).await?;
        Ok(())
    }
}