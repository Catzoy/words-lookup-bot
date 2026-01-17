pub trait CronRunner {
    async fn run_cron(&self) -> anyhow::Result<()>;
}
