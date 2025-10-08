use crate::bloc::common::{Lookup, LookupError};
use crate::format::{compose_phrase_defs, LookupFormatter};
use crate::stands4::{PhraseDefinition, Stands4Client};
use shuttle_runtime::async_trait;

#[async_trait]
pub trait PhraseLookup: Lookup {
    type Formatter: LookupFormatter<Self::Response> + Default;
    fn formatter(&self) -> Self::Formatter {
        Default::default()
    }

    async fn get_definitions(
        client: Stands4Client,
        phrase: String,
    ) -> anyhow::Result<Vec<PhraseDefinition>> {
        client.search_phrase(phrase.as_str()).await
    }

    fn compose_response(
        self,
        phrase: String,
        defs: Vec<PhraseDefinition>,
    ) -> Result<Self::Response, LookupError> {
        let formatter = self.formatter();
        compose_phrase_defs(formatter, phrase.as_str(), &defs).map_err(|err| {
            log::error!("Failed to construct a response: {:?}", err);
            LookupError::FailedResponseBuilder
        })
    }
}
