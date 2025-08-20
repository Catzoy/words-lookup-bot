use crate::commands::{Command, Payload};
use shuttle_runtime::async_trait;
use teloxide::prelude::Requester;

pub struct UnknownCommand {}

#[async_trait]
impl Command for UnknownCommand {
    fn name(&self) -> &'static str {
        ""
    }

    async fn handle(&self, &Payload { bot, message, .. }: &Payload) -> anyhow::Result<()> {
        bot.send_message(
            message.chat.id,
            "I don't know that command, sorry.",
        ).await?;
        Ok(())
    }
}