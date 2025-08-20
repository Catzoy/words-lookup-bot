use crate::commands::{Command, HelpDescriptor};
use shuttle_runtime::async_trait;
use teloxide::prelude::{Message, Requester};
use teloxide::types::Me;
use teloxide::Bot;

pub struct TeapotCommand {}
impl TeapotCommand {
    pub(crate) const NAME: &'static str = "teapot";
}
#[async_trait]
impl Command for TeapotCommand {
    fn name(&self) -> &'static str {
        TeapotCommand::NAME
    }

    fn descriptor(&self) -> Option<HelpDescriptor> {
        None
    }

    async fn handle(&self, _me: &Me, bot: &Bot, message: &Message, _args: Vec<String>) -> anyhow::Result<()> {
        bot.send_message(message.chat.id, "I'm a teapot").await?;
        Ok(())
    }
}