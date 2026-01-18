use crate::bot::runner::BotRunner;
use crate::commands::commands_tree;
use crate::datamuse::client::DatamuseClient;
use crate::inlines::{InlineQueryDebouncer, inlines_tree};
use crate::service::telegram::TelegramService;
use crate::urban::UrbanDictionaryClient;
use futures::FutureExt;
use std::time::Duration;
use teloxide::dispatching::{DefaultKey, Dispatcher};
use teloxide::dptree::{deps, entry};
use teloxide::error_handlers::LoggingErrorHandler;
use teloxide::prelude::{DependencyMap, Requester};
use teloxide::types::{ChatId, Recipient};
use teloxide::{Bot, update_listeners};
use tokio::time::sleep;

impl TelegramService {
    fn deps(&self) -> DependencyMap {
        deps![
            self.stands4_client.clone(),
            self.wordle_cache.clone(),
            InlineQueryDebouncer::default(),
            UrbanDictionaryClient::default(),
            DatamuseClient::default()
        ]
    }

    fn build_dispatcher(&self, bot: Bot) -> Dispatcher<Bot, anyhow::Error, DefaultKey> {
        // Other update types are of no interest to use since this REPL is only for
        // messages. See <https://github.com/teloxide/teloxide/issues/557>.
        let ignore_update = |_upd| Box::pin(async {});
        let tree = entry().branch(inlines_tree()).branch(commands_tree());

        Dispatcher::builder(bot.clone(), tree)
            .default_handler(ignore_update)
            .dependencies(self.deps())
            .enable_ctrlc_handler()
            .build()
    }

    async fn notify_ready(&self, bot: Bot) {
        let msg = "Bot ready!";
        let recipient = Recipient::Id(ChatId(self.admin_chat));
        if let Err(e) = bot.send_message(recipient, msg).await {
            log::error!("Could not notify owner of Bot availability, {:?}", e);
        }
    }
}

impl BotRunner for TelegramService {
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
    ///     let service = TelegramService::new(your_config);
    ///     service.run_bot().await?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Returns
    ///
    /// `Ok(())` if the dispatcher exits normally, or an `Err` if startup or runtime
    /// errors occur.
    async fn run_bot(&self) -> anyhow::Result<()> {
        let bot = Bot::new(self.token.clone());
        let poller = update_listeners::polling_default(bot.clone()).await;
        let err_handler =
            LoggingErrorHandler::with_custom_text("An error from the update listener");
        let mut dispatcher = self.build_dispatcher(bot.clone());
        let dispatch = dispatcher.dispatch_with_listener(poller, err_handler);
        let notify = sleep(Duration::from_secs(2)).then(|_| self.notify_ready(bot.clone()));
        tokio::join!(dispatch, notify);
        Ok(())
    }
}
