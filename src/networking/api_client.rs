use log::log_enabled;
use serde::de::DeserializeOwned;

pub struct ApiClient {
    pub client: rustify::Client,
}

impl ApiClient {
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
        let response = response.parse()?;
        if log_enabled!(log::Level::Debug) {
            log::debug!("RESPONSE={:?}", response);
        }

        Ok(response.into())
    }
}
