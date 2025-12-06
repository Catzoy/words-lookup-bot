use crate::bloc::common::CommandHandler;
use crate::bot::LookupBot;
use teloxide::dptree::entry;

pub trait StartBot<Value> {
    fn start_response(&self) -> Value;
}

pub trait StartHandler {
    async fn send_start(&self) -> anyhow::Result<()>;

    fn start_handler() -> CommandHandler;
}
impl<Bot> StartHandler for Bot
where
    Bot: StartBot<Bot::Response> + LookupBot + Send + Sync + 'static,
{
    /// Sends the bot's start response and completes when the response is delivered.
    ///
    /// # Errors
    ///
    /// Returns an error if delivering the response fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // Assuming `bot` implements `StartHandler`
    /// # async fn example<B: StartHandler>(bot: B) -> anyhow::Result<()> {
    /// bot.send_start().await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn send_start(&self) -> anyhow::Result<()> {
        self.answer(self.start_response()).await?;
        Ok(())
    }
    /// Constructs a command handler that invokes a bot's start action when the handler is executed.
    ///
    /// The returned `CommandHandler` routes incoming start commands to the bot's `send_start` method.
    ///
    /// # Examples
    ///
    /// ```
    /// let handler = start_handler();
    /// // register `handler` with the bot dispatcher so `send_start` is called for start commands
    /// ```
    fn start_handler() -> CommandHandler {
        entry().endpoint(|bot: Bot| async move { bot.send_start().await })
    }
}
