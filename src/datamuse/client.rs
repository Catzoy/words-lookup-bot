use crate::datamuse::responses::Word;

#[derive(Debug, Clone, Default)]
pub struct DatamuseClient {
    client: reqwest::Client,
}

impl DatamuseClient {
    /// Builds a Datamuse API URL for a word pattern, converting underscores to Datamuse `?` wildcards.
    ///
    /// The returned `String` is the full HTTP GET URL querying Datamuse words with the given pattern.
    /// Underscores in `mask` are replaced with `?` before being inserted into the `sp` query parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// let url = finding("a_e_".to_string());
    /// assert!(url.starts_with("https://api.datamuse.com/words?sp="));
    /// assert!(url.contains("a?e?"));
    /// ```
    fn finding(mask: String) -> String {
        format!(
            "https://api.datamuse.com/words?sp={:}",
            mask.replace("_", "?")
        )
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
    pub async fn find(&self, mask: String) -> anyhow::Result<Vec<String>> {
        let response = self.client.get(Self::finding(mask)).send().await?;
        let mut words = response.json::<Vec<Word>>().await?;
        words.sort_by(|a, b| a.word.cmp(&b.word));
        Ok(words.into_iter().map(|word| word.word).collect())
    }
}