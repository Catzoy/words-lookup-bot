use crate::bloc::common::CommandHandler;
use crate::bot::LookupBot;
use teloxide::dptree::entry;

pub trait UnknownBot<Response> {
    fn unknown_response(&self) -> Response;
}
pub trait UnknownHandler {
    async fn send_unknown(&self) -> anyhow::Result<()>;

    fn unknown_handler() -> CommandHandler;
}

impl<Bot> UnknownHandler for Bot
where
    Bot: UnknownBot<Bot::Response> + LookupBot + Send + Sync + 'static,
{
    /// Sends the implementor's "unknown" response using its `answer` method.
    ///
    /// The method obtains the response from `unknown_response()` and forwards it to `answer`, propagating any error.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or the error returned by `answer` on failure.
    ///
    /// # Examples
    ///
    /// ```
    /// async fn example<B: crate::bloc::unknown::UnknownHandler>(bot: B) -> anyhow::Result<()> {
    ///     bot.send_unknown().await
    /// }
    /// ```
    async fn send_unknown(&self) -> anyhow::Result<()> {
        self.answer(self.unknown_response()).await?;
        Ok(())
    }
    /// Creates a CommandHandler that routes unknown commands to the bot's `send_unknown` method.
    ///
    /// # Examples
    ///
    /// ```
    /// let handler = unknown_handler();
    /// // register `handler` with your dispatcher
    /// ```
    fn unknown_handler() -> CommandHandler {
        entry().endpoint(|bot: Bot| async move { bot.send_unknown().await })
    }
}