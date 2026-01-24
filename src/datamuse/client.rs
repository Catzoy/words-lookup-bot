use crate::datamuse::responses::Word;
use crate::networking::api_client::ApiClient;

#[derive(Debug, Clone, Default)]
pub struct DatamuseClient {
    client: reqwest::Client,
}

impl DatamuseClient {
    /// Creates an ApiClient configured for the Datamuse API.
    ///
    /// This constructs an `ApiClient` that uses this instance's HTTP client and is targeted
    /// at the Datamuse base URL.
    ///
    /// # Examples
    ///
    /// ```
    /// // Assuming `datamuse` is a `DatamuseClient`
    /// // let api = datamuse.client();
    /// ```
    fn client(&self) -> ApiClient {
        ApiClient {
            client: rustify::Client::new("https://api.datamuse.com", self.client.clone()),
        }
    }
    /// Execute a Datamuse API endpoint and return the words from its response sorted in ascending order.
    ///
    /// The provided `request` must implement `rustify::Endpoint` with `Response = Vec<Word>`. The result is the list
    /// of `word` fields from the response, sorted lexicographically.
    ///
    /// # Parameters
    ///
    /// - `request`: An endpoint describing the Datamuse API request; its response type must be `Vec<Word>`.
    ///
    /// # Returns
    ///
    /// A `Vec<String>` containing the `word` values from the endpoint response, sorted ascending by the word.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crate::datamuse::client::DatamuseClient;
    /// # async fn example() -> anyhow::Result<()> {
    /// let client = DatamuseClient::default();
    /// // `request` should be any type implementing `rustify::Endpoint<Response = Vec<crate::datamuse::Word>>`
    /// let request = /* build request */ ;
    /// let words = client.exec(request).await?; // network access required
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