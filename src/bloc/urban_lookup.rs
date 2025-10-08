use crate::bloc::common::{Lookup, LookupError};
use crate::format::{compose_urban_defs, LookupFormatter};
use crate::urban::{UrbanDefinition, UrbanDictionaryClient};

pub trait UrbanLookup: Lookup {
    type Formatter: LookupFormatter<Self::Response> + Default;

    async fn get_definitions(
        client: UrbanDictionaryClient,
        term: String,
    ) -> anyhow::Result<Vec<UrbanDefinition>> {
        client.search_term(&term).await
    }

    fn compose_response(
        self,
        term: String,
        defs: Vec<UrbanDefinition>,
    ) -> Result<Self::Response, LookupError> {
        let formatter = Self::Formatter::default();
        compose_urban_defs(formatter, &term, &defs).map_err(|err| {
            log::error!("Failed to construct a response: {:?}", err);
            LookupError::FailedResponseBuilder
        })
    }
}
