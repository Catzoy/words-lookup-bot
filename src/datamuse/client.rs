use crate::datamuse::responses::Word;

#[derive(Debug, Clone, Default)]
pub struct DatamuseClient {
    client: reqwest::Client,
}

impl DatamuseClient {
    fn finding(mask: String) -> String {
        format!(
            "https://api.datamuse.com/words?sp={:}",
            mask.replace("_", "?")
        )
    }
    pub async fn find(&self, mask: String) -> anyhow::Result<Vec<String>> {
        let response = self.client.get(Self::finding(mask)).send().await?;
        let mut words = response.json::<Vec<Word>>().await?;
        words.sort_by(|a, b| a.word.cmp(&b.word));
        Ok(words.into_iter().map(|word| word.word).collect())
    }
}
