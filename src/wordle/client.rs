use crate::networking::api_client::ApiClient;
use rustify::Endpoint;

#[derive(Clone)]
pub struct WordleClient {
    client: reqwest::Client,
}

impl WordleClient {
    /// Constructs a WordleClient that uses the provided reqwest HTTP client.
    ///
    /// # Examples
    ///
    /// ```
    /// use reqwest::Client;
    ///
    /// let client = Client::new();
    /// let wc = crate::WordleClient::new(client);
    /// ```
    pub fn new(client: reqwest::Client) -> WordleClient {
        WordleClient { client }
    }

    /// Constructs an ApiClient configured for the NYT Wordle v2 API using the internal HTTP client.
    ///
    /// The returned `ApiClient` wraps a `rustify::Client` rooted at
    /// "https://www.nytimes.com/svc/wordle/v2" and reuses this instance's `reqwest::Client`.
    ///
    /// # Examples
    ///
    /// ```
    /// let wc = WordleClient::default();
    /// let api = wc.client();
    /// // `api` is ready to execute endpoints against the NYT Wordle v2 service.
    /// ```
    fn client(&self) -> ApiClient {
        ApiClient {
            client: rustify::Client::new(
                "https://www.nytimes.com/svc/wordle/v2",
                self.client.clone(),
            ),
        }
    }

    /// Execute the given rustify `Endpoint` against the NYT Wordle API using this client's configured HTTP client.
    ///
    /// The provided `request` describes the HTTP operation and expected response type; this method performs the request and returns the deserialized response.
    ///
    /// # Parameters
    ///
    /// - `request`: The endpoint to execute; its associated `Response` type is returned on success.
    ///
    /// # Returns
    ///
    /// `E::Response` on success, `anyhow::Error` on failure.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rustify::Endpoint;
    /// # use crate::wordle::client::WordleClient;
    /// # async fn example() -> anyhow::Result<()> {
    /// let client = WordleClient::default();
    /// // `my_endpoint` must implement `Endpoint` and set its `Response` type.
    /// let my_endpoint = /* construct endpoint implementing `Endpoint` */;
    /// let resp = client.exec(my_endpoint).await?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn exec<E: Endpoint<Response: std::fmt::Debug>>(
        &self,
        request: E,
    ) -> anyhow::Result<E::Response> {
        self.client().exec(request).await
    }
}

impl Default for WordleClient {
    /// Creates a WordleClient using a default-configured reqwest HTTP client.
    ///
    /// # Examples
    ///
    /// ```
    /// let client = WordleClient::default();
    /// ```
    fn default() -> WordleClient {
        WordleClient::new(reqwest::Client::new())
    }
}