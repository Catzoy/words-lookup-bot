use crate::bloc::common::{Lookup, LookupError};
use crate::format::{compose_syn_ant_defs, LookupFormatter};
use crate::stands4::{Stands4Client, SynAntDefinitions};
use shuttle_runtime::async_trait;

#[async_trait]
pub trait ThesaurusLookup: Lookup {
    type Formatter: LookupFormatter<Self::Response> + Default;

    async fn get_definitions(
        client: Stands4Client,
        term: String,
    ) -> Result<Vec<SynAntDefinitions>, LookupError> {
        client.search_syn_ant(&term).await.map_err(|e| {
            log::error!("term lookup error: {:?}", e);
            LookupError::FailedRequest
        })
    }

    fn compose_response(
        term: String,
        defs: Vec<SynAntDefinitions>,
    ) -> Result<Self::Response, LookupError> {
        let formatter = Self::Formatter::default();
        compose_syn_ant_defs(formatter, &term, &defs).map_err(|err| {
            log::error!("Failed to construct a response: {:?}", err);
            LookupError::FailedResponseBuilder
        })
    }
}
