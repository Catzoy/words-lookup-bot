use crate::commands::{BoxedCommand, Command, CommandsRegistry};
use shuttle_runtime::async_trait;
use std::sync::Arc;
use teloxide::prelude::{Message, Requester};
use teloxide::types::Me;
use teloxide::Bot;

pub struct HelpCommand {
    commands: Vec<BoxedCommand>,
}

impl HelpCommand {
    pub fn new(registry: &CommandsRegistry) -> Self {
        Self {
            commands: registry.get_commands()
                .map(|v| Arc::clone(v))
                .collect()
        }
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
        msg = self.commands.iter().fold(msg, |mut builder, command| {
            let line = format!("{} - {}", command.name(), command.description());
            builder.append(line);
            builder
        });
        let msg = msg.string()?;
        bot.send_message(message.chat.id, msg).await?;
        Ok(())
    }
}