pub mod runner;
pub mod warm_up;

use crate::wordle::cache::WordleCache;

#[derive(Clone)]
pub struct ServerState {
    pub(crate) wordle_cache: WordleCache,
}
