#[derive(Default)]
pub struct LinksProvider {}

impl LinksProvider {
    pub(crate) fn word_link(&self, word: &str) -> String {
        format!("https://www.definitions.net/definition/{}", word)
    }

    pub(crate) fn abbr_link(&self, word: &str) -> String {
        format!("https://www.abbreviations.com/{}", word)
    }

    pub(crate) fn phrase_link(&self, phrase: &str) -> String {
        format!(
            "https://www.phrases.com/psearch/{}",
            phrase.replace(" ", "+")
        )
    }

    pub(crate) fn urban_link(&self, term: &str) -> String {
        format!(
            "https://www.urbandictionary.com/define.php?term={}",
            urlencoding::encode(term),
        )
    }
}
