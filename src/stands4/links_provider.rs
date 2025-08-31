use crate::format::formatter::LinkProvider;

pub struct DefaultLinksProvider {}

impl LinkProvider for DefaultLinksProvider {
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

    fn urban_link(&self, term: &str) -> String {
        format!(
            "https://www.urbandictionary.com/define.php?term={}",
            urlencoding::encode(term),
        )
    }
}