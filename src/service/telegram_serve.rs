use crate::server::runner::ServerRunner;
use crate::server::warm_up::warm_up;
use crate::server::ServerState;
use crate::service::telegram::TelegramService;
use crate::wordle::cache::WordleCache;
use axum::routing::get;
use std::net::SocketAddr;
use std::sync::Arc;

impl ServerRunner for TelegramService {
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
