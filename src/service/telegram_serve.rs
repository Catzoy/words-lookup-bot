use crate::server::runner::ServerRunner;
use crate::server::warm_up::warm_up;
use crate::server::ServerState;
use crate::service::telegram::TelegramService;
use crate::wordle::cache::WordleCache;
use axum::routing::get;
use std::net::SocketAddr;
use std::sync::Arc;

impl ServerRunner for TelegramService {
    /// Start an HTTP server bound to `addr` that serves the Telegram service routes.
    ///
    /// The server creates a shared `ServerState` by cloning the provided `WordleCache`,
    /// registers a GET route at `/warm_up`, binds a TCP listener to `addr`, and serves
    /// the Axum application until it finishes or fails.
    ///
    /// # Returns
    ///
    /// `Ok(())` on successful startup and serving, or an `Err` with the underlying error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::net::SocketAddr;
    /// # use tokio::time::{sleep, Duration};
    /// # #[tokio::test]
    /// # async fn run_server_example() {
    /// // Construct service and cache (replace with real constructors).
    /// let svc = TelegramService::new();
    /// let cache = WordleCache::default();
    /// let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    ///
    /// // Run the server in the background and stop shortly after.
    /// let handle = tokio::spawn(async move { let _ = svc.run_server(addr, &cache).await; });
    /// sleep(Duration::from_millis(10)).await;
    /// handle.abort();
    /// # }
    /// ```
    async fn run_server(&self, addr: SocketAddr, wordle_cache: &WordleCache) -> anyhow::Result<()> {
        let state = Arc::new(ServerState {
            wordle_cache: wordle_cache.clone(),
        });

        let app = axum::Router::new()
            .route("/warm_up", get(warm_up))
            .with_state(state);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}