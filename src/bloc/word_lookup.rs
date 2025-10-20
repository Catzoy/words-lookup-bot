use crate::bloc::common::{Lookup, LookupError};
use crate::format::LookupFormatter;
use crate::stands4::{AbbreviationDefinition, Stands4Client, VecAbbreviationsExt, WordDefinition};
use futures::TryFutureExt;
use shuttle_runtime::async_trait;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};

#[async_trait]
pub trait WordLookup: Lookup
where
    Self::Response: PartialEq,
{
    type Formatter: LookupFormatter<Self::Response> + Default;
    fn on_empty() -> Self::Response {
        Default::default()
    }

    async fn get_definitions(
        client: Stands4Client,
        word: String,
    ) -> (Vec<WordDefinition>, Vec<AbbreviationDefinition>) {
        futures::future::join(
            client.search_word(&word).unwrap_or_else(|err| {
                log::error!("Failed to retrieve definitions of a word: {:?}", err);
                vec![]
            }),
            client.search_abbreviation(&word).unwrap_or_else(|err| {
                log::error!("Failed to retrieve definitions of an abbr: {:?}", err);
                vec![]
            }),
        ).await
    }

    fn compose_response(
        word: String,
        (words, abbrs): (Vec<WordDefinition>, Vec<AbbreviationDefinition>),
    ) -> Result<Self::Response, LookupError> {
        let formatter = Self::Formatter::default();
        let text = match (words.len(), abbrs.len()) {
            (0, 0) => Ok(Self::on_empty()),
            (0, _) => formatter.compose_abbr_defs(&word, &abbrs),
            (_, 0) => formatter.compose_word_defs(&word, &words),
            (_, _) => formatter.compose_words_with_abbrs(&word, &words, &abbrs),
        };
        text.map_err(|err| {
            log::error!("Failed to construct a response: {:?}", err);
            LookupError::FailedResponseBuilder
        })
    }

    fn propose_replies(word: String, response: Result<Self::Response, LookupError>) -> Option<ReplyMarkup> {
        let mut keyboard = InlineKeyboardMarkup::new::<Vec<Vec<_>>>(vec![]);
        if let Ok(text) = response && text != Self::on_empty() {
            let search_thesaurus = InlineKeyboardButton::callback(
                "Search for synonyms/antonyms of the word",
                format!("sa.{}", word),
            );
            keyboard = keyboard.append_row(vec![search_thesaurus]);
        }
        let search_urban = InlineKeyboardButton::callback(
            "Search in Urban Dictionary",
            format!("u.{}", word),
        );
        keyboard = keyboard.append_row(vec![search_urban]);
        Some(ReplyMarkup::InlineKeyboard(keyboard))
    }
}

pub trait WordLookupFormatter<R, E> {
    fn compose_word_defs(self, word: &str, defs: &Vec<WordDefinition>) -> Result<R, E>;
    fn compose_abbr_defs(self, word: &str, defs: &Vec<AbbreviationDefinition>) -> Result<R, E>;

    fn compose_words_with_abbrs(
        self,
        word: &str,
        words: &Vec<WordDefinition>,
        abbrs: &Vec<AbbreviationDefinition>,
    ) -> Result<R, E>;
}

impl<T, R, E> WordLookupFormatter<R, E> for T
where
    T: LookupFormatter<R, Error=E>,
{
    fn compose_word_defs(mut self, word: &str, defs: &Vec<WordDefinition>) -> Result<R, E> {
        self.append_title(format!("Found {} definitions", defs.len()));

        for (i, def) in defs.iter().take(5).enumerate() {
            self.visit_word(i, def);
        }
        if defs.len() > 5 {
            self.append_link(self.link_provider().word_link(word))
        }
        self.build()
    }

    fn compose_abbr_defs(mut self, word: &str, defs: &Vec<AbbreviationDefinition>) -> Result<R, E> {
        self.append_title(format!("Found {} definitions", defs.len()));

        let categorized = defs.categorized();
        for (i, (category, defs)) in categorized.iter().take(5).enumerate() {
            self.visit_abbreviations(i, category, defs);
        }
        if categorized.len() > 5 {
            self.append_link(self.link_provider().abbr_link(word))
        }
        self.build()
    }

    fn compose_words_with_abbrs(
        mut self,
        word: &str,
        words: &Vec<WordDefinition>,
        abbrs: &Vec<AbbreviationDefinition>,
    ) -> Result<R, E> {
        self.append_title(format!("Found {} definitions", words.len()));

        for (i, def) in words.iter().take(5).enumerate() {
            self.visit_word(i, def);
        }
        if words.len() > 5 {
            self.append_link(self.link_provider().word_link(word))
        }

        self.append_title(format!("Found {} abbreviations", abbrs.len()));

        let categorized = abbrs.categorized();
        for (i, (category, defs)) in categorized.iter().take(5).enumerate() {
            self.visit_abbreviations(i, category, defs);
        }
        if categorized.len() > 5 {
            self.append_link(self.link_provider().abbr_link(word))
        }

        self.build()
    }
}
