use crate::bloc::common::CommandHandler;
use crate::bot::LookupBot;
use teloxide::dptree::entry;

pub trait TeapotBot<Value> {
    fn teapot(&self) -> Value;
}
pub trait TeapotHandler {
    async fn send_teapot(&self) -> anyhow::Result<()>;
    fn teapot_handler() -> CommandHandler;
}
impl<Bot> TeapotHandler for Bot
where
    Bot: TeapotBot<Bot::Response> + LookupBot + Send + Sync + 'static,
{
    async fn send_teapot(&self) -> anyhow::Result<()> {
        self.answer(self.teapot()).await?;
        Ok(())
    }

    fn teapot_handler() -> CommandHandler {
        entry().endpoint(|bot: Bot| async move { bot.send_teapot().await })
    }
}
