use crate::commands::command::Command;
use shuttle_runtime::async_trait;
use teloxide::prelude::{Message, Requester};
use teloxide::types::Me;
use teloxide::Bot;

pub struct UnknownCommand {}

#[async_trait]
impl Command for UnknownCommand {
    fn name(&self) -> &'static str {
        ""
    }

    fn description(&self) -> &'static str {
        "Handles unknown commands and prints this message"
    }

    async fn handle(&self, _me: &Me, bot: &Bot, message: &Message, _args: Vec<String>) -> anyhow::Result<()> {
        bot.send_message(
            message.chat.id,
            "I don't know that command, sorry.",
        ).await?;
        Ok(())
    }
}