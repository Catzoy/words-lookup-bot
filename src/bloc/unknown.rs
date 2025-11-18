use crate::bloc::common::CommandHandler;
use crate::bot::LookupBot;
use teloxide::dptree::entry;

pub trait UnknownBot<Response> {
    fn unknown_response(&self) -> Response;
}
pub trait UnknownHandler {
    async fn send_unknown(self) -> anyhow::Result<()>;

    fn unknown_handler() -> CommandHandler;
}

impl<Bot> UnknownHandler for Bot
where
    Bot: UnknownBot<Bot::Response> + LookupBot + Send + Sync + 'static,
{
    async fn send_unknown(self) -> anyhow::Result<()> {
        self.answer(self.unknown_response()).await?;
        Ok(())
    }
    fn unknown_handler() -> CommandHandler {
        entry().endpoint(|bot: Bot| async move { bot.send_unknown().await })
    }
}
