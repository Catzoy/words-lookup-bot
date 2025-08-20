use crate::commands::{Command, HelpDescriptor};
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

    fn descriptor(&self) -> Option<HelpDescriptor> {
        None
    }

    async fn handle(&self, _me: &Me, bot: &Bot, message: &Message, _args: Vec<String>) -> anyhow::Result<()> {
        bot.send_message(
            message.chat.id,
            "I don't know that command, sorry.",
        ).await?;
        Ok(())
    }
}