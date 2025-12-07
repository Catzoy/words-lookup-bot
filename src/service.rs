use crate::commands::*;
use crate::datamuse::client::DatamuseClient;
use crate::inlines::*;
use crate::stands4::client::Stands4Client;
use crate::urban::UrbanDictionaryClient;
use crate::wordle::cache::WordleCache;
use crate::wordle::WordleClient;
use shuttle_runtime::Error;
use std::net::SocketAddr;
use teloxide::dispatching::Dispatcher;
use teloxide::dptree::{deps, entry};
use teloxide::error_handlers::LoggingErrorHandler;
use teloxide::{update_listeners, Bot};

#[derive(Clone)]
pub struct TelegramService {
    pub(crate) token: String,
    pub(crate) stands4_client: Stands4Client,
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for TelegramService {
    /// Starts and runs the Telegram bot dispatcher with its required dependencies and a polling update listener.
    ///
    /// The service constructs the bot and dependency set, registers command and inline handler trees,
    /// ignores non-message updates, and runs the dispatcher until completion.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the dispatcher runs to completion; an `Error` otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::net::SocketAddr;
    /// # use tokio::runtime::Runtime;
    /// # use my_crate::TelegramService;
    /// # fn make_service() -> TelegramService { unimplemented!() }
    /// # let mut rt = Runtime::new().unwrap();
    /// # rt.block_on(async {
    /// let svc = make_service();
    /// let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    /// let _ = svc.bind(addr).await;
    /// # });
    /// ```
    async fn bind(self, _: SocketAddr) -> Result<(), Error> {
        let bot = Bot::new(self.token.clone());
        let cloned_bot = bot.clone();

        // Other update types are of no interest to use since this REPL is only for
        // messages. See <https://github.com/teloxide/teloxide/issues/557>.
        let ignore_update = |_upd| Box::pin(async {});
        let deps = deps![
            self.stands4_client.clone(),
            InlineQueryDebouncer::default(),
            UrbanDictionaryClient::default(),
            DatamuseClient::default(),
            WordleCache::new(WordleClient::default(), self.stands4_client.clone(),)
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