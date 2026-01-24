use log::log_enabled;
use serde::de::DeserializeOwned;

pub struct ApiClient {
    pub client: rustify::Client,
}

impl ApiClient {
    /// Execute an endpoint against the client's base URL and return the parsed response converted into `Entity`.
    ///
    /// This sends the provided `request` using the client's HTTP runtime, parses the response body into the
    /// endpoint's `Response` type, and converts that parsed response into `Entity` via `From<Response>`.
    ///
    /// # Returns
    ///
    /// `Entity` converted from the endpoint's parsed response.
    ///
    /// # Examples
    ///
    /// ```
    /// // async context required
    /// # async fn example_usage() -> anyhow::Result<()> {
    /// // let client = ApiClient { client: /* initialized rustify::Client */ };
    /// // let request = /* an Endpoint implementation */;
    /// // let entity = client.exec(request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub(crate) async fn exec<Entity, Response, Endpoint>(
        &self,
        request: Endpoint,
    ) -> anyhow::Result<Entity>
    where
        Response: std::fmt::Debug + Send + Sync + DeserializeOwned,
        Endpoint: rustify::Endpoint<Response = Response>,
        Entity: From<Response>,
    {
        let url = request.url(self.client.base.as_str())?;
        log::info!("REQUEST URL {:?}", url);

        let response = request.exec(&self.client).await?;
        if log_enabled!(log::Level::Debug) {
            let str = String::from_utf8(response.raw());
            if let Ok(str) = str {
                log::debug!("STR={:?}", str);
            } else {
                log::debug!("Received a non-string response");
            }
        }

        let response = response.parse()?;
        if log_enabled!(log::Level::Debug) {
            log::debug!("RESPONSE={:?}", response);
        }

        Ok(response.into())
    }
}