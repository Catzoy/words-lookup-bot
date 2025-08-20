use crate::commands::command::Command;
use shuttle_runtime::async_trait;
use teloxide::prelude::{Message, Requester};
use teloxide::types::Me;
use teloxide::Bot;

pub struct TeapotCommand {}

#[async_trait]
impl Command for TeapotCommand {
    async fn handle(&self, _me: &Me, bot: &Bot, message: &Message, _args: Vec<String>) -> anyhow::Result<()> {
        bot.send_message(message.chat.id, "I'm a teapot").await?;
        Ok(())
    }
}