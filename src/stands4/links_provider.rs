use crate::format::formatter::LinkProvider;

pub struct Stands4LinksProvider {}

impl LinkProvider for Stands4LinksProvider {
    fn word_link(&self, word: &str) -> String {
        format!("https://www.definitions.net/definition/{}", word)
    }

    fn abbr_link(&self, word: &str) -> String {
        format!("https://www.abbreviations.com/{}", word)
    }

    fn phrase_link(&self, phrase: &str) -> String {
        format!(
            "https://www.phrases.com/psearch/{}",
            phrase.replace(" ", "+")
        )
    }
}