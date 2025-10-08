use crate::bloc::common::{Lookup, LookupError};
use crate::format::{LookupFormatter, StringBuilderExt};
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
        let mut formatter = Self::Formatter::default();
        formatter.append_title(format!(
            "Found {} different definitions with respective information",
            defs.len()
        ));
        for (i, def) in defs.iter().take(5).enumerate() {
            formatter.visit_syn_ant(i, def)
        }
        if defs.len() > 5 {
            formatter.append_link(formatter.link_provider().syn_ant_link(&term))
        }

        formatter.build().map_err(|err| {
            log::error!("Failed to construct a response: {:?}", err);
            LookupError::FailedResponseBuilder
        })
    }
}
