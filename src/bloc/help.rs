use crate::bloc::common::CommandHandler;
use crate::bot::LookupBot;
use teloxide::dptree::entry;

pub trait HelpBot<Value> {
    fn help(&self) -> Value;
}

pub trait HelpHandler {
    async fn send_help(&self) -> anyhow::Result<()>;
    fn help_handler() -> CommandHandler;
}
impl<Bot> HelpHandler for Bot
where
    Bot: HelpBot<Bot::Response> + LookupBot + Send + Sync + 'static,
{
    async fn send_help(&self) -> anyhow::Result<()> {
        self.answer(self.help()).await?;
        Ok(())
    }

    fn help_handler() -> CommandHandler {
        entry().endpoint(|bot: Bot| async move { bot.send_help().await })
    }
}
