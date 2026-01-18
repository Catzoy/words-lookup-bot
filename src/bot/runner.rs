pub trait BotRunner {
    async fn run_bot(&self) -> anyhow::Result<()>;
}
