use crate::bloc::common::{Lookup, LookupError};
use crate::format::{compose_phrase_defs, LookupFormatter};
use crate::stands4::{PhraseDefinition, Stands4Client};
use shuttle_runtime::async_trait;

#[async_trait]
pub trait PhraseLookup: Lookup {
    type Formatter: LookupFormatter<Self::Response> + Default;

    async fn get_definitions(
        client: Stands4Client,
        phrase: String,
    ) -> Result<Vec<PhraseDefinition>, LookupError> {
        client.search_phrase(phrase.as_str()).await.map_err(|e| {
            log::error!("phrase search error: {:?}", e);
            LookupError::FailedRequest
        })
    }

    fn compose_response(
        phrase: String,
        defs: Vec<PhraseDefinition>,
    ) -> Result<Self::Response, LookupError> {
        let formatter = Self::Formatter::default();
        compose_phrase_defs(formatter, phrase.as_str(), &defs).map_err(|err| {
            log::error!("Failed to construct a response: {:?}", err);
            LookupError::FailedResponseBuilder
        })
    }
}
