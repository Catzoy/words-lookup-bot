use chrono::Local;
use reqwest::Client;
use tokio_cron_scheduler::{Job, JobBuilder};

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

async fn refresh_wordle() -> anyhow::Result<()> {
    let client = Client::new();
    let request = client.get("http://127.0.0.1:8080/warm_up").build()?;
    client.execute(request).await?;
    Ok(())
}
