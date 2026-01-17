use crate::wordle::WordleAnswer;
use chrono::{DateTime, Local, Utc};
use reqwest::Client;

#[derive(Clone)]
pub struct WordleClient {
    client: Client,
}

impl WordleClient {
    /// Creates a WordleClient that uses the provided HTTP client.
    ///
    /// # Examples
    ///
    /// ```
    /// use reqwest::Client;
    /// let client = Client::new();
    /// let wc = crate::WordleClient::new(client);
    /// ```
    pub fn new(client: Client) -> WordleClient {
        WordleClient { client }
    }

    /// Fetches the Wordle answer for the specified local date from the NYT Wordle service.
    ///
    /// # Parameters
    ///
    /// - `day`: The local date for which to retrieve the Wordle answer; only the date (YYYY-MM-DD) portion is used.
    ///
    /// # Returns
    ///
    /// `WordleAnswer` parsed from the service response on success, or an error if the request or deserialization fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use chrono::Local;
    /// # use crate::WordleClient;
    ///
    /// let client = WordleClient::default();
    /// let day = Local::now();
    /// let answer = tokio::runtime::Runtime::new()
    ///     .unwrap()
    ///     .block_on(async { client.get_word(&day).await })
    ///     .unwrap();
    /// println!("Wordle answer for {}: {:?}", day.format("%Y-%m-%d"), answer);
    /// ```
    pub async fn get_word(&self, day: &DateTime<Local>) -> anyhow::Result<WordleAnswer> {
        let url = format!(
            "https://www.nytimes.com/svc/wordle/v2/{}.json",
            day.format("%Y-%m-%d").to_string()
        );
        let res = self.client.get(&url).send().await?;
        Ok(res.json::<WordleAnswer>().await?)
    }
}

impl Default for WordleClient {
    fn default() -> WordleClient {
        WordleClient::new(Client::new())
    }
}