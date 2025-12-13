use crate::bot::runner::BotRunner;
use crate::server::runner::ServerRunner;
use crate::stands4::client::Stands4Client;
use crate::wordle::cache::WordleCache;
use crate::wordle::WordleClient;
use shuttle_runtime::Error;
use std::net::SocketAddr;

#[derive(Clone)]
pub struct TelegramService {
    pub(crate) token: String,
    pub(crate) stands4_client: Stands4Client,
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for TelegramService {
    /// Starts and runs the Telegram bot dispatcher and the HTTP server until completion.
    ///
    /// Constructs the service dependencies, registers command and inline handlers, ignores non-message updates, and runs the server and bot dispatcher concurrently.
    ///
    /// # Parameters
    ///
    /// * `addr` - Socket address to bind the HTTP server to.
    ///
    /// # Returns
    ///
    /// `Ok(())` if both the server and bot dispatcher complete successfully, `Error` otherwise.
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
    async fn bind(self, addr: SocketAddr) -> Result<(), Error> {
        let wordle_cache = WordleCache::new(WordleClient::default(), self.stands4_client.clone());

        tokio::try_join!(
            self.run_server(addr, &wordle_cache),
            self.run_bot(&wordle_cache)
        )?;
        Ok(())
    }
}