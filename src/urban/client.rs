use crate::networking::api_client::ApiClient;
use crate::urban::{UrbanDefinition, UrbanResponse};
use rustify::errors::ClientError;
use std::default::Default;

#[derive(Clone)]
pub struct UrbanDictionaryClient {
    client: reqwest::Client,
}

impl UrbanDictionaryClient {
    /// Create a client for interacting with the unofficial Urban Dictionary API.
    ///
    /// # Returns
    ///
    /// An `UrbanDictionaryClient` that will use the provided `reqwest::Client` for HTTP requests.
    ///
    /// # Examples
    ///
    /// ```
    /// let client = reqwest::Client::new();
    /// let ud = UrbanDictionaryClient::new(client);
    /// ```
    pub fn new(client: reqwest::Client) -> Self {
        UrbanDictionaryClient { client }
    }

    /// Creates an ApiClient configured for the Urban Dictionary API using the internal reqwest client.
    ///
    /// # Returns
    ///
    /// An `ApiClient` configured with the Urban Dictionary base URL and a clone of this client's `reqwest::Client`.
    ///
    /// # Examples
    ///
    /// ```
    /// let ud = UrbanDictionaryClient::new(Default::default());
    /// let api = ud.client();
    /// ```
    fn client(&self) -> ApiClient {
        ApiClient {
            client: rustify::Client::new(
                "https://unofficialurbandictionaryapi.com/api",
                self.client.clone(),
            ),
        }
    }

    /// Execute the given Endpoint against the Urban Dictionary API and return the parsed definitions.
    ///
    /// On a successful API response with HTTP status 200, returns the response's definitions. If the
    /// API responds with a non-200 status, this returns an error containing the API `message` field or
    /// the default message "Urban lookup failed without an error!". If the request fails with a server
    /// response error with status code 404, this method returns an empty vector instead of an error.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crate::urban::client::UrbanDictionaryClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let client = UrbanDictionaryClient::default();
    ///     // Construct an Endpoint that implements `rustify::Endpoint<Response = UrbanResponse>`,
    ///     // then call `client.exec(endpoint).await` to obtain definitions.
    ///     Ok(())
    /// }
    /// ```
    pub async fn exec<Endpoint: rustify::Endpoint<Response = UrbanResponse>>(
        &self,
        request: Endpoint,
    ) -> anyhow::Result<Vec<UrbanDefinition>> {
        self.client()
            .exec::<UrbanResponse, _, _>(request)
            .await
            .and_then(|response| match response.status_code {
                200 => Ok(response.data),
                _ => {
                    let err_msg = response
                        .message
                        .unwrap_or_else(|| "Urban lookup failed without an error!".to_string());
                    anyhow::bail!(err_msg);
                }
            })
            .or_else(|err| match err.downcast::<ClientError>()? {
                ClientError::ServerResponseError { code: 404, .. } => Ok(vec![]),
                it => Err(it.into()),
            })
    }
}

impl Default for UrbanDictionaryClient {
    /// Create an UrbanDictionaryClient configured with a default `reqwest::Client`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let _: UrbanDictionaryClient = UrbanDictionaryClient::default();
    /// ```
    fn default() -> Self {
        UrbanDictionaryClient::new(Default::default())
    }
}