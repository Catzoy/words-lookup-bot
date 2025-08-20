use crate::commands::{Command, Payload};
use shuttle_runtime::async_trait;
use teloxide::prelude::Requester;

pub struct TeapotCommand {}
impl TeapotCommand {
    pub(crate) const NAME: &'static str = "teapot";
}
#[async_trait]
impl Command for TeapotCommand {
    fn name(&self) -> &'static str {
        TeapotCommand::NAME
    }

    async fn handle(&self, &Payload { bot, message, .. }: &Payload) -> anyhow::Result<()> {
        bot.send_message(message.chat.id, "I'm a teapot").await?;
        Ok(())
    }
}