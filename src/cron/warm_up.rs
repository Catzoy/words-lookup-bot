use chrono::Local;
use reqwest::Client;
use tokio_cron_scheduler::{Job, JobBuilder};

/// Creates a cron job that runs daily at 00:00 in the local timezone and invokes `refresh_wordle`.
///
/// The returned `Job` is configured with the local timezone, a schedule of `0 0 0 * * *` (daily at
/// midnight), and an asynchronous runner that calls `refresh_wordle` and logs success or failure.
///
/// # Examples
///
/// ```
/// let job = wordle_self_warmup_job();
/// // `job` is ready to be added to a scheduler; constructing it should not panic.
/// ```
pub fn wordle_self_warmup_job() -> Job {
    JobBuilder::new()
        .with_timezone(Local::now().timezone())
        .with_schedule("0 0 0 * * *") // Every Day At 00:00:00
        .expect("Schedule parser failed")
        .with_cron_job_type()
        .with_run_async(Box::new(|_u, _l| {
            Box::pin(async move {
                match refresh_wordle().await {
                    Ok(_) => {
                        log::info!("Wordle refresh success")
                    }
                    Err(ex) => {
                        log::error!("Failed to refresh wordle {:?}", ex)
                    }
                }
            })
        }))
        .build()
        .expect("Wordle job must succeed")
}

/// Triggers the local Wordle warm-up endpoint.
///
/// Sends an HTTP GET to http://127.0.0.1:8080/warm_up and returns success if the request completes.
///
/// # Errors
/// Returns an error if building or executing the HTTP request fails.
///
/// # Examples
///
/// ```no_run
/// use anyhow::Result;
/// # async fn run() -> Result<()> {
/// refresh_wordle().await?;
/// # Ok(())
/// # }
/// ```
async fn refresh_wordle() -> anyhow::Result<()> {
    let client = Client::new();
    let request = client.get("http://127.0.0.1:8080/warm_up").build()?;
    client.execute(request).await?;
    Ok(())
}