use crate::bloc::common::CommandHandler;
use crate::bot::LookupBot;
use teloxide::dptree::entry;

pub trait HelpBot<Value> {
    fn help(&self) -> Value;
}

pub trait HelpHandler {
    fn send_help(&self) -> impl Future<Output = anyhow::Result<()>> + Send;
    fn help_handler() -> CommandHandler;
}
impl<Bot> HelpHandler for Bot
where
    Bot: HelpBot<Bot::Response> + LookupBot + Send + Sync + 'static,
{
    /// Sends the bot's help content to the current chat.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the help message was sent successfully, an `Err` otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// #     // `bot` should implement `HelpHandler`.
    /// #     let bot = /* obtain bot instance */ unimplemented!();
    ///     bot.send_help().await?;
    ///     Ok(())
    /// # }
    /// ```
    async fn send_help(&self) -> anyhow::Result<()> {
        self.answer(self.help()).await?;
        Ok(())
    }

    /// Builds a CommandHandler that routes the help command to the bot's `send_help` method.
    ///
    /// The returned handler invokes `Bot::send_help()` when the help command is dispatched.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // `MyBot` is a type that implements `HelpHandler`.
    /// let handler = <MyBot as HelpHandler>::help_handler();
    /// ```
    fn help_handler() -> CommandHandler {
        entry().endpoint(|bot: Bot| async move { bot.send_help().await })
    }
}
