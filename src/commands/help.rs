use crate::commands::{Command, CommandsRegistry};
use shuttle_runtime::async_trait;
use std::sync::Arc;
use teloxide::prelude::{Message, Requester};
use teloxide::types::Me;
use teloxide::Bot;

struct HelpCommand {
    registry: Arc<CommandsRegistry>,
}

impl HelpCommand {
    pub fn new(registry: Arc<CommandsRegistry>) -> Self {
        Self { registry }
    }
}

#[async_trait]
impl Command for HelpCommand {
    fn name(&self) -> &'static str {
        "help"
    }

    fn description(&self) -> &'static str {
        "Print this helpful message"
    }

    async fn handle(&self, _me: &Me, bot: &Bot, message: &Message, _args: Vec<String>) -> anyhow::Result<()> {
        let mut msg = string_builder::Builder::default();
        msg.append("Here are supported commands:\n\n");
        msg = self.registry.get_commands().fold(msg, |mut builder, command| {
            let line = format!("{} - {}", command.name(), command.description());
            builder.append(line);
            builder
        });
        let msg = msg.string()?;
        bot.send_message(message.chat.id, msg).await?;
        Ok(())
    }
}