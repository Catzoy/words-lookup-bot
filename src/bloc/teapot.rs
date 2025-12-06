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
    /// Sends the bot's teapot response to the current chat.
    ///
    /// Calls the bot's `teapot()` to obtain the response value and sends it with `answer`.
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or an error propagated from the underlying send operation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use anyhow::Result;
    /// # async fn example<B: crate::bloc::teapot::TeapotHandler>(bot: &B) -> Result<()> {
    /// bot.send_teapot().await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn send_teapot(&self) -> anyhow::Result<()> {
        self.answer(self.teapot()).await?;
        Ok(())
    }

    /// Builds a CommandHandler that routes the teapot command to the bot's `send_teapot` method.
    ///
    /// # Returns
    ///
    /// A `CommandHandler` which, when executed by teloxide, calls `Bot::send_teapot`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::bloc::teapot::TeapotHandler;
    ///
    /// // Construct the handler for the implementing Bot type
    /// let handler = <Bot as TeapotHandler>::teapot_handler();
    /// ```
    fn teapot_handler() -> CommandHandler {
        entry().endpoint(|bot: Bot| async move { bot.send_teapot().await })
    }
}