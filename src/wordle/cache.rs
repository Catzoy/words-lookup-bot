use crate::stands4::Stands4Client;
use crate::stands4::requests::SearchWordRequest;
use crate::wordle::requests::WordleAnswerRequest;
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

    /// Obtain the WordleDayAnswer for the current local day, using the cache when possible.
    ///
    /// If the cached entry matches today's date, that cached `WordleDayAnswer` is returned.
    /// Otherwise the function fetches the latest answer and definitions, updates the cache, and returns the newly fetched `WordleDayAnswer`.
    ///
    /// # Returns
    ///
    /// The `WordleDayAnswer` for the current local day.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn example(mut cache: crate::wordle::WordleCache) {
    /// let today_answer = cache.require_fresh_answer().await.unwrap();
    /// println!("{}", today_answer.answer.solution);
    /// # }
    /// ```
    pub async fn require_fresh_answer(&mut self) -> anyhow::Result<WordleDayAnswer> {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let mut latest = self.latest.lock().await;
        if let Some(answer) = latest.as_ref()
            && today == answer.day
        {
            log::info!("Wordle cache hit!");
            return Ok(answer.clone());
        }

        log::info!("Wordle cache miss!");
        let newest = {
            let request = WordleAnswerRequest {
                date: today.clone(),
            };
            self.wordle_client.exec(request).await?
        };
        let sw_request = SearchWordRequest {
            word: newest.solution.to_string(),
        };
        let definitions = self.stands4_client.exec(sw_request).await?;
        let answer = WordleDayAnswer {
            day: today,
            answer: newest,
            definitions,
        };
        log::info!("New Wordle answer {:?}", answer);
        latest.replace(answer.clone());
        Ok(answer)
    }
}
