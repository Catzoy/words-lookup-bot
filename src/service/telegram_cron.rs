use crate::cron::runner::CronRunner;
use crate::cron::warm_up::wordle_self_warmup_job;
use crate::service::telegram::TelegramService;
use tokio_cron_scheduler::JobScheduler;

impl CronRunner for TelegramService {
    /// Starts the cron scheduler, registers the Wordle self-warmup job, and begins executing scheduled jobs.
    ///
    /// On success, the scheduler has been created, the warmup job has been added, and the scheduler is running.
    /// Returns an error if scheduler creation, job registration, or scheduler start fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn example(service: &crate::service::TelegramService) -> anyhow::Result<()> {
    /// service.run_cron().await?;
    /// Ok(())
    /// # }
    /// ```
    async fn run_cron(&self) -> anyhow::Result<()> {
        let scheduler = JobScheduler::new().await?;
        scheduler.add(wordle_self_warmup_job()).await?;
        scheduler.start().await?;
        Ok(())
    }
}