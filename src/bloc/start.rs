use crate::bot::LookupBot;
use crate::commands::CommandHandler;
use teloxide::dptree::entry;

pub trait StartBot<Value> {
    fn start_response(&self) -> Value;
}

pub trait StartHandler {
    async fn send_start(self) -> anyhow::Result<()>;

    fn start_handler() -> CommandHandler;
}
impl<Bot> StartHandler for Bot
where
    Bot: StartBot<Bot::Response> + LookupBot + Send + Sync + 'static,
{
    async fn send_start(self) -> anyhow::Result<()> {
        self.answer(self.start_response()).await?;
        Ok(())
    }
    fn start_handler() -> CommandHandler {
        entry().endpoint(|bot: Bot| async move { bot.send_start().await })
    }
}
