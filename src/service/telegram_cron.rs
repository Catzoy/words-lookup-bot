use crate::cron::runner::CronRunner;
use crate::cron::warm_up::wordle_self_warmup_job;
use crate::service::telegram::TelegramService;
use tokio_cron_scheduler::JobScheduler;

impl CronRunner for TelegramService {
    async fn run_cron(&self) -> anyhow::Result<()> {
        let scheduler = JobScheduler::new().await?;
        scheduler.add(wordle_self_warmup_job()).await?;
        scheduler.start().await?;
        Ok(())
    }
}
