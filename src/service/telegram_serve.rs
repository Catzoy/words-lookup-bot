use crate::server::ServerState;
use crate::server::runner::ServerRunner;
use crate::server::warm_up::warm_up;
use crate::service::telegram::TelegramService;
use axum::routing::get;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal;

impl ServerRunner for TelegramService {
    /// Start an HTTP server bound to `addr` that serves the Telegram service routes.
    ///
    /// The server constructs a shared `ServerState` by cloning the service's wordle cache,
    /// registers a GET route at `/warm_up`, binds a TCP listener to `addr`, and runs the
    /// Axum application until a shutdown signal is received or serving fails.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::net::SocketAddr;
    /// # use tokio::time::{sleep, Duration};
    /// # #[tokio::test]
    /// # async fn run_server_example() {
    /// let svc = TelegramService::new();
    /// let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    /// let handle = tokio::spawn(async move { let _ = svc.run_server(addr).await; });
    /// sleep(Duration::from_millis(10)).await;
    /// handle.abort();
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, `Err` with the underlying error otherwise.
    async fn run_server(&self, addr: SocketAddr) -> anyhow::Result<()> {
        let state = Arc::new(ServerState {
            wordle_cache: self.wordle_cache.clone(),
        });

        let app = axum::Router::new()
            .route("/warm_up", get(warm_up))
            .with_state(state);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await?;
        Ok(())
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}