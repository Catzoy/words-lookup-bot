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
