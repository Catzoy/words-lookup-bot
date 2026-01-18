use crate::Config;
use crate::bot::runner::BotRunner;
use crate::cron::runner::CronRunner;
use crate::server::runner::ServerRunner;
use crate::stands4::client::Stands4Client;
use crate::wordle::WordleClient;
use crate::wordle::cache::WordleCache;
use std::net::SocketAddr;

#[derive(Clone)]
pub struct TelegramService {
    pub(crate) admin_chat: i64,
    pub(crate) token: String,
    pub(crate) stands4_client: Stands4Client,
    pub(crate) wordle_cache: WordleCache,
}

impl TelegramService {
    pub fn new(config: Config) -> Self {
        let stands4_client = Stands4Client::new(config.stands4_user_id, config.stands4_token);
        let wordle_cache = WordleCache::new(WordleClient::default(), stands4_client.clone());
        TelegramService {
            admin_chat: config.admin_chat,
            token: config.teloxide_token,
            stands4_client,
            wordle_cache,
        }
    }

    /// Runs the Telegram cron routine, the HTTP server bound to `addr`, and the Telegram bot dispatcher concurrently until they complete.
    ///
    /// A shared WordleCache is created and passed to the server and bot tasks; this function awaits the cron, server, and bot tasks and propagates any error returned by them.
    ///
    /// # Returns
    ///
    /// `Ok(())` if all tasks complete successfully, `Err` if any task returns an error.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::net::SocketAddr;
    /// use tokio::runtime::Runtime;
    /// use my_crate::TelegramService;
    ///
    /// fn make_service() -> TelegramService { unimplemented!() }
    ///
    /// let mut rt = Runtime::new().unwrap();
    /// rt.block_on(async {
    ///     let svc = make_service();
    ///     let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    ///     let _ = svc.bind(addr).await;
    /// });
    /// ```
    pub(crate) async fn bind(self, addr: SocketAddr) -> Result<(), anyhow::Error> {
        tokio::try_join!(self.run_cron(), self.run_server(addr), self.run_bot())?;
        Ok(())
    }
}
