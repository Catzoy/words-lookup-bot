use crate::server::ServerState;
use axum::extract::State;
use std::sync::Arc;

/// Triggers retrieval of a fresh Wordle answer using the shared server state and returns a confirmation message.
///
/// Attempts to refresh the cached Wordle answer via the state's `wordle_cache`. On failure, the error is logged
/// and an `INTERNAL_SERVER_ERROR` status is returned.
///
/// # Returns
///
/// `Ok(String)` containing `"Warmup complete"` on success, or `Err(StatusCode::INTERNAL_SERVER_ERROR)` if refreshing the answer fails.
///
/// # Examples
///
/// ```no_run
/// use std::sync::Arc;
/// use axum::extract::State;
/// // Assume `ServerState` and `warm_up` are available in scope.
/// // let state = Arc::new(ServerState::new(...));
/// // let result = tokio::runtime::Runtime::new().unwrap().block_on(async {
/// //     warm_up(State(state)).await
/// // });
/// // assert_eq!(result.unwrap(), "Warmup complete".to_string());
/// ```
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