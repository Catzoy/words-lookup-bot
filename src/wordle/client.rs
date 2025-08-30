use crate::wordle::WordleAnswer;
use chrono::{DateTime, Utc};
use reqwest::Client;

#[derive(Clone)]
pub struct WordleClient {
    client: Client,
}

impl WordleClient {
    pub fn new(client: Client) -> WordleClient {
        WordleClient { client }
    }

    pub async fn get_word(&self, day: &DateTime<Utc>) -> anyhow::Result<WordleAnswer> {
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
        let client = Client::new();
        WordleClient { client }
    }
}