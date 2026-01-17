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
    /// Creates a new WordleCache containing the given clients and an initially empty cached answer.
    ///
    /// # Examples
    ///
    /// ```
    /// let wordle_client = WordleClient::new(/* config */);
    /// let stands4_client = Stands4Client::new(/* config */);
    /// let cache = WordleCache::new(wordle_client, stands4_client);
    /// ```
    pub fn new(wordle_client: WordleClient, stands4_client: Stands4Client) -> WordleCache {
        WordleCache {
            latest: Arc::default(),
            wordle_client,
            stands4_client,
        }
    }

    /// Fetches and the current local day's Wordle answer, returning a cached value when it already matches today's date.
    ///
    /// If the cache contains an entry for the current local day, that cached `WordleDayAnswer` is returned; otherwise the function obtains the latest answer from the configured clients, updates the cache, and returns the new value.
    ///
    /// # Returns
    ///
    /// `WordleDayAnswer` for the current local day.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn example(mut cache: WordleCache) {
    /// let today_answer = cache.require_fresh_answer().await.unwrap();
    /// println!("{}", today_answer.answer.solution);
    /// # }
    /// ```
    pub async fn require_fresh_answer(&mut self) -> anyhow::Result<WordleDayAnswer> {
        let today = chrono::Local::now();
        let mut latest = self.latest.lock().await;
        if let Some(latest) = latest.as_ref() {
            let today = today.format("%Y-%m-%d").to_string();
            let known = latest.day.format("%Y-%m-%d").to_string();
            if today == known {
                log::info!("Wordle cache hit!");
                return Ok(latest.clone());
            }
        }
        log::info!("Wordle cache miss!");

        let newest = self.wordle_client.get_word(&today).await?;
        let definitions = self.stands4_client.search_word(&newest.solution).await?;
        let answer = WordleDayAnswer {
            day: today,
            answer: newest,
            definitions,
        };
        let clone = answer.clone();
        latest.replace(answer);
        Ok(clone)
    }
}