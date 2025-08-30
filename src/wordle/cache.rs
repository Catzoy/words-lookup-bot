use crate::stands4::Stands4Client;
use crate::wordle::{WordleClient, WordleDayAnswer};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct WordleCache {
    wordle_client: WordleClient,
    stands4_client: Stands4Client,
    latest: Arc<Mutex<Option<WordleDayAnswer>>>,
}

impl WordleCache {
    pub fn new(wordle_client: WordleClient, stands4_client: Stands4Client) -> WordleCache {
        WordleCache {
            latest: Arc::default(),
            wordle_client,
            stands4_client,
        }
    }

    pub async fn with_answer<T>(&self, mapper: fn(&WordleDayAnswer) -> T) -> anyhow::Result<T> {
        match self.latest.lock().await.as_ref() {
            Some(value) => Ok(mapper(value)),
            None => anyhow::bail!("Cache doesn't contain an answer to the wordle!"),
        }
    }

    pub async fn require_fresh_answer(&mut self) -> anyhow::Result<()> {
        let today = chrono::Utc::now();
        let mut latest = self.latest.lock().await;
        if let Some(latest) = latest.as_ref() {
            let today = today.format("%Y-%m-%d").to_string();
            let known = latest.day.format("%Y-%m-%d").to_string();
            if today == known {
                log::info!("Wordle cache hit!");
                return Ok(());
            }
        }
        log::info!("Wordle cache miss!");

        let newest = self.wordle_client.get_word(&today).await?;
        let definitions = self.stands4_client.search_word(&newest.solution).await?;
        latest.replace(WordleDayAnswer { day: today, answer: newest, definitions });
        Ok(())
    }
}