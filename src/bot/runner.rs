use crate::wordle::cache::WordleCache;

pub trait BotRunner {
    async fn run_bot(&self, wordle_cache: &WordleCache) -> anyhow::Result<()>;
}
