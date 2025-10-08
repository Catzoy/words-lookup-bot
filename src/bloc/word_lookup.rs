use crate::bloc::common::{Lookup, LookupError};
use crate::format::{
    compose_abbr_defs, compose_word_defs, compose_words_with_abbrs, LookupFormatter,
};
use crate::stands4::{AbbreviationDefinition, Stands4Client, WordDefinition};
use futures::TryFutureExt;
use shuttle_runtime::async_trait;

#[async_trait]
pub trait WordLookup: Lookup {
    type Formatter: LookupFormatter<Self::Response> + Default;
    fn on_empty(&self) -> Self::Response {
        Default::default()
    }
    fn formatter(&self) -> Self::Formatter {
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
        )
        .await
    }

    fn compose_response(
        self,
        word: String,
        (words, abbrs): (Vec<WordDefinition>, Vec<AbbreviationDefinition>),
    ) -> Result<Self::Response, LookupError> {
        let formatter = self.formatter();
        let text = match (words.len(), abbrs.len()) {
            (0, 0) => Ok(self.on_empty()),
            (0, _) => compose_abbr_defs(formatter, &word, &abbrs),
            (_, 0) => compose_word_defs(formatter, &word, &words),
            (_, _) => compose_words_with_abbrs(formatter, &word, &words, &abbrs),
        };
        text.map_err(|err| {
            log::error!("Failed to construct a response: {:?}", err);
            LookupError::FailedResponseBuilder
        })
    }
}
