use crate::commands::command::Command;
use shuttle_runtime::async_trait;
use teloxide::prelude::{Message, Requester};
use teloxide::types::Me;
use teloxide::Bot;

pub struct StartCommand {}

#[async_trait]
impl Command for StartCommand {
    async fn handle(&self, me: &Me, bot: &Bot, message: &Message, args: Vec<String>) -> anyhow::Result<()> {
        bot.send_message(
            message.chat.id,
            "Hi!\n\
            I'm a bot that can look up words and phrases.\n\
            Simply send me a message and I'll search for the definition of the text."
                .to_string(),
        ).await?;
        Ok(())
    }
}