use crate::wordle::cache::WordleCache;
use std::net::SocketAddr;

pub trait ServerRunner {
    async fn run_server(&self, addr: SocketAddr, wordle_cache: &WordleCache) -> anyhow::Result<()>;
}
