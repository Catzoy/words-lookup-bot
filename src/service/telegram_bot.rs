use crate::commands::commands_tree;
use crate::datamuse::client::DatamuseClient;
use crate::inlines::{inlines_tree, InlineQueryDebouncer};
use crate::service::telegram::TelegramService;
use crate::urban::UrbanDictionaryClient;
use crate::wordle::cache::WordleCache;
use teloxide::dispatching::Dispatcher;
use teloxide::dptree::{deps, entry};
use teloxide::error_handlers::LoggingErrorHandler;
use teloxide::{update_listeners, Bot};

impl crate::bot::runner::BotRunner for TelegramService {
    /// Starts the Teloxide-based Telegram bot dispatcher and runs it until shutdown.
    ///
    /// This configures a bot with the service's token, a dependency set (including
    /// the wordle cache and various API clients), and a dispatch tree that handles
    /// inline queries and commands. Non-message update types are ignored by the
    /// default handler. The dispatcher uses a polling listener and a logging error
    /// handler and will stop on Ctrl+C or when the listener completes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crate::service::TelegramService;
    /// use crate::wordle::WordleCache;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     // Constructing TelegramService is omitted; replace with your initializer.
    ///     let service = TelegramService::new("YOUR_TELEGRAM_BOT_TOKEN".into());
    ///     let cache = WordleCache::default();
    ///     service.run_bot(&cache).await?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Returns
    ///
    /// `Ok(())` if the dispatcher exits normally, or an `Err` if startup or runtime
    /// errors occur.
    async fn run_bot(&self, wordle_cache: &WordleCache) -> anyhow::Result<()> {
        // Other update types are of no interest to use since this REPL is only for
        // messages. See <https://github.com/teloxide/teloxide/issues/557>.
        let ignore_update = |_upd| Box::pin(async {});
        let bot = Bot::new(self.token.clone());
        let cloned_bot = bot.clone();
        let deps = deps![
            self.stands4_client.clone(),
            InlineQueryDebouncer::default(),
            UrbanDictionaryClient::default(),
            DatamuseClient::default(),
            wordle_cache.clone()
        ];
        let tree = entry().branch(inlines_tree()).branch(commands_tree());
        Dispatcher::builder(bot, tree)
            .default_handler(ignore_update)
            .dependencies(deps)
            .enable_ctrlc_handler()
            .build()
            .dispatch_with_listener(
                update_listeners::polling_default(cloned_bot).await,
                LoggingErrorHandler::with_custom_text("An error from the update listener"),
            )
            .await;
        Ok(())
    }
}