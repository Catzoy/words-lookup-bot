use std::net::SocketAddr;

pub trait ServerRunner {
    async fn run_server(&self, addr: SocketAddr) -> anyhow::Result<()>;
}
