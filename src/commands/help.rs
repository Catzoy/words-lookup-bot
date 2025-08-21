use crate::commands::{Command, Payload};
use shuttle_runtime::async_trait;
use teloxide::prelude::Requester;
use teloxide::types::ParseMode;

pub struct HelpCommand {}

pub struct HelpDescriptor {
    pub(crate) name: &'static str,
    pub(crate) description: &'static str,
}

impl HelpCommand {
    const NAME: &'static str = "help";
}

#[async_trait]
impl Command for HelpCommand {
    fn name(&self) -> &'static str {
        HelpCommand::NAME
    }

    fn descriptor(&self) -> Option<HelpDescriptor> {
        Some(HelpDescriptor {
            name: HelpCommand::NAME,
            description: "Print this helpful message",
        })
    }

    async fn handle(&self, &Payload { service, bot, message, .. }: &Payload) -> anyhow::Result<()> {
        let mut msg = string_builder::Builder::default();
        msg.append("Here are the supported commands:\n\n");
        msg = service.registry
            .get_commands()
            .flat_map(|cmd| cmd.descriptor())
            .fold(msg, |mut builder, command| {
                let line = format!("/{} - {}\n", command.name, command.description);
                builder.append(line);
                builder
            });
        let msg = msg.string()?;
        let mut msg = bot.send_message(message.chat.id, msg);
        msg.parse_mode = Some(ParseMode::MarkdownV2);
        msg.await?;
        
        Ok(())
    }
}