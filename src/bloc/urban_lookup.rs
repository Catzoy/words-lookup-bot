use crate::bloc::common::{Lookup, LookupError};
use crate::format::LookupFormatter;
use crate::urban::{UrbanDefinition, UrbanDictionaryClient};

pub trait UrbanLookup: Lookup {
    type Formatter: LookupFormatter<Self::Response> + Default;

    async fn get_definitions(
        client: UrbanDictionaryClient,
        term: String,
    ) -> Result<Vec<UrbanDefinition>, LookupError> {
        client.search_term(&term).await.map_err(|e| {
            log::error!("term lookup error: {:?}", e);
            LookupError::FailedRequest
        })
    }

    fn compose_response(
        term: String,
        defs: Vec<UrbanDefinition>,
    ) -> Result<Self::Response, LookupError> {
        let mut formatter = Self::Formatter::default();
        formatter.append_title(format!(
            "Found {} definitions from Urban Dictionary",
            defs.len()
        ));

        for (i, def) in defs.iter().take(5).enumerate() {
            formatter.visit_urban_definition(i, def);
        }
        if defs.len() > 5 {
            formatter.append_link(formatter.link_provider().urban_link(&term))
        }
        formatter.build().map_err(|err| {
            log::error!("Failed to construct a response: {:?}", err);
            LookupError::FailedResponseBuilder
        })
    }
}
