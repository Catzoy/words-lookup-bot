use reqwest::Client;
use serde::Deserialize;
use shuttle_runtime::__internals::serde_json;

const WORDS_API_URL: &str = "https://www.stands4.com/services/v2/defs.php";
const PHRASES_API_URL: &str = "https://www.stands4.com/services/v2/phrases.php";

#[derive(Clone)]
pub struct Stands4Client {
    client: Client,
    user_id: String,
    token: String,
}

#[derive(Deserialize, Debug)]
pub struct Results<T> {
    results: Vec<T>,
}

#[derive(Deserialize, Debug)]
pub struct WordResult {
    term: String,
    definition: String,
    #[serde(rename = "partofspeech")]
    part_of_speech: String,
}

#[derive(Deserialize, Debug)]
pub struct PhraseResult {
    term: String,
    explanation: String,
}

impl Stands4Client {
    pub fn new(user_id: String, token: String) -> Self {
        Stands4Client {
            client: Client::new(),
            user_id,
            token,
        }
    }
    pub async fn search_word(&self, word: &str) -> anyhow::Result<Vec<WordResult>> {
        let query = &[
            ("user_id", self.user_id.as_str()),
            ("token", self.token.as_str()),
            ("format", "json"),
            ("word", word),
        ];
        let request = self.client.get(WORDS_API_URL).query(query);
        let request = request.build()?;
        let url = request.url().to_string();
        log::info!("REQUEST URL {:?}", url);

        let response = self.client.execute(request).await?;
        let txt = response.text().await?;
        log::info!("RESPONSE={:?}", txt);

        let results = serde_json::from_slice::<Results<WordResult>>(txt.as_bytes())
            .map_err(anyhow::Error::msg)?;
        Ok(results.results)
    }
    pub async fn search_phrase(&self, phrase: &str) -> anyhow::Result<Vec<PhraseResult>> {
        let query = &[
            ("user_id", self.user_id.as_str()),
            ("token", self.token.as_str()),
            ("format", "json"),
            ("phrase", phrase),
        ];
        let request = self.client.get(PHRASES_API_URL).query(query);
        let request = request.build()?;
        let url = request.url().to_string();
        log::info!("REQUEST URL {:?}", url);

        let response = self.client.execute(request).await?;
        let txt = response.text().await?;
        log::info!("RESPONSE={:?}", txt);

        let results = serde_json::from_slice::<Results<PhraseResult>>(txt.as_bytes())
            .map_err(anyhow::Error::msg)?;
        Ok(results.results)
    }
}