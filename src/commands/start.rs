use crate::commands::{Command, HelpDescriptor, Payload};
use shuttle_runtime::async_trait;
use teloxide::prelude::Requester;

pub struct StartCommand {}
impl StartCommand {
    const NAME: &'static str = "start";
}
#[async_trait]
impl Command for StartCommand {
    fn name(&self) -> &'static str {
        Self::NAME
    }

    fn descriptor(&self) -> Option<HelpDescriptor> {
        Some(HelpDescriptor {
            name: StartCommand::NAME,
            description: "Doesn't really do anything, is just here to greet you.",
        })
    }

    async fn handle(&self, &Payload { bot, message, .. }: &Payload) -> anyhow::Result<()> {
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