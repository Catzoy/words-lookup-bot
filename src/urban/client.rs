use crate::networking::api_client::ApiClient;
use crate::urban::{UrbanDefinition, UrbanResponse};
use std::default::Default;

#[derive(Clone)]
pub struct UrbanDictionaryClient {
    client: reqwest::Client,
}

impl UrbanDictionaryClient {
    pub fn new(client: reqwest::Client) -> Self {
        UrbanDictionaryClient { client }
    }

    fn client(&self) -> ApiClient {
        ApiClient {
            client: rustify::Client::new(
                "https://unofficialurbandictionaryapi.com/api",
                self.client.clone(),
            ),
        }
    }

    pub async fn exec<Endpoint: rustify::Endpoint<Response = UrbanResponse>>(
        &self,
        request: Endpoint,
    ) -> anyhow::Result<Vec<UrbanDefinition>> {
        let response: UrbanResponse = self.client().exec(request).await?;
        if response.status_code != 200 {
            let err_msg = response
                .message
                .unwrap_or_else(|| "Urban lookup failed without an error!".to_string());
            anyhow::bail!(err_msg);
        }
        Ok(response.data)
    }
}

impl Default for UrbanDictionaryClient {
    fn default() -> Self {
        UrbanDictionaryClient::new(Default::default())
    }
}
