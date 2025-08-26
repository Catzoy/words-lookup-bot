use crate::commands::*;
use crate::inlines::debouncer::InlineQueryDebouncer;
use crate::inlines::inlines::inlines_tree;
use crate::stands4::client::Stands4Client;
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
    async fn bind(self, _: SocketAddr) -> Result<(), Error> {
        let bot = Bot::new(self.token.clone());
        let cloned_bot = bot.clone();

        // Other update types are of no interest to use since this REPL is only for
        // messages. See <https://github.com/teloxide/teloxide/issues/557>.
        let ignore_update = |_upd| Box::pin(async {});
        let deps = deps![
            self.stands4_client.clone(),
            InlineQueryDebouncer::default()
        ];

        let tree = entry()
            .branch(inlines_tree())
            .branch(commands_tree());
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