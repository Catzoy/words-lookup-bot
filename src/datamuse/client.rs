use crate::datamuse::responses::Word;
use crate::networking::api_client::ApiClient;

#[derive(Debug, Clone, Default)]
pub struct DatamuseClient {
    client: reqwest::Client,
}

impl DatamuseClient {
    fn client(&self) -> ApiClient {
        ApiClient {
            client: rustify::Client::new("https://api.datamuse.com", self.client.clone()),
        }
    }
    /// Query the Datamuse API for words matching a pattern and return the matching words sorted in ascending order.
    ///
    /// The `mask` may contain underscore characters (`_`) which are treated as single-character wildcards (they are converted to Datamuse `?` placeholders before the request).
    ///
    /// # Returns
    ///
    /// A `Vec<String>` containing the matching words sorted ascending by the word.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crate::datamuse::client::DatamuseClient;
    /// # async fn example() -> anyhow::Result<()> {
    /// let client = DatamuseClient::default();
    /// let words = client.find("c_t".to_string()).await?;
    /// // `words` is a sorted `Vec<String>`; network access is required for real results.
    /// assert!(words.len() >= 0);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn exec<Endpoint: rustify::Endpoint<Response = Vec<Word>>>(
        &self,
        request: Endpoint,
    ) -> anyhow::Result<Vec<String>> {
        let mut words: Vec<Word> = self.client().exec(request).await?;
        words.sort_by(|a, b| a.word.cmp(&b.word));
        Ok(words.into_iter().map(|word| word.word).collect())
    }
}
