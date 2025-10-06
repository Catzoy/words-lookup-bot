use crate::format::ToEscaped;

#[derive(Default)]
pub struct LinksProvider {}

impl LinksProvider {
    pub(crate) fn word_link(&self, word: &str) -> String {
        format!("https://www.definitions.net/definition/{}", word).to_escaped()
    }

    pub(crate) fn abbr_link(&self, word: &str) -> String {
        format!("https://www.abbreviations.com/{}", word).to_escaped()
    }

    pub(crate) fn phrase_link(&self, phrase: &str) -> String {
        format!(
            "https://www.phrases.com/psearch/{}",
            phrase.replace(" ", "+")
        ).to_escaped()
    }

    pub(crate) fn urban_link(&self, term: &str) -> String {
        format!(
            "https://www.urbandictionary.com/define.php?term={}",
            urlencoding::encode(term),
        ).to_escaped()
    }

    pub(crate) fn syn_ant_link(&self, term: &str) -> String {
        format!(
            "https://www.synonyms.com/synonym/{}",
            term.replace(" ", "+")
        ).to_escaped()
    }
}
