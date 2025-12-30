use crate::urban::{UrbanDefinition, UrbanResponse};
use reqwest::Client;

const API_URL: &str = "https://unofficialurbandictionaryapi.com/api/search";

#[derive(Clone)]
pub struct UrbanDictionaryClient {
    client: Client,
}

impl UrbanDictionaryClient {
    pub fn new(client: Client) -> Self {
        UrbanDictionaryClient { client }
    }

    pub async fn search_term(&self, term: &str) -> anyhow::Result<Vec<UrbanDefinition>> {
        let query = &[("term", term)];
        let request = self.client.get(API_URL).query(query);
        let request = request.build()?;
        let url = request.url().to_string();
        log::info!("REQUEST URL {:?}", url);

        let response = self.client.execute(request).await?;
        let txt = response.text().await?;
        log::info!("RESPONSE={:?}", txt);

        let response =
            serde_json::from_slice::<UrbanResponse>(txt.as_bytes()).map_err(anyhow::Error::msg)?;
        if response.status_code != 200 {
            anyhow::bail!(
                response
                    .message
                    .unwrap_or_else(|| "Urban lookup failed without an error!".to_string())
            );
        }
        Ok(response.data)
    }
}

impl Default for UrbanDictionaryClient {
    fn default() -> Self {
        UrbanDictionaryClient::new(Client::new())
    }
}
