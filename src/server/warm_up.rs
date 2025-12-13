use crate::server::ServerState;
use axum::extract::State;
use std::sync::Arc;

pub async fn warm_up(
    State(state): State<Arc<ServerState>>,
) -> Result<String, axum::http::StatusCode> {
    let mut wordle_cache = state.wordle_cache.clone();
    wordle_cache.require_fresh_answer().await.map_err(|err| {
        log::error!("Couldn't retrieve wordle answer on warmup {:?}", err);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok("Warmup complete".to_string())
}
