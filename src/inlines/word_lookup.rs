use crate::bloc::common::{CommonLookup, HandlerOwner, Lookup};
use crate::bloc::word_lookup::WordLookup;
use crate::inlines::{formatting::InlineFormatter, InlineHandler, QueryCommands};
use crate::stands4::{AbbreviationDefinition, WordDefinition};
use shuttle_runtime::async_trait;
use teloxide::prelude::InlineQuery;
use teloxide::types::InlineQueryResult;

#[derive(Debug, Clone)]
pub struct InlinesWordLookup;

#[async_trait]
impl WordLookup for InlinesWordLookup {
    type Formatter = InlineFormatter;
}

impl Lookup for InlinesWordLookup {
    type Request = InlineQuery;
    type Entity = (Vec<WordDefinition>, Vec<AbbreviationDefinition>);
    type Response = Vec<InlineQueryResult>;
}

impl HandlerOwner for InlinesWordLookup {
    fn handler() -> InlineHandler {
        teloxide::dptree::case![QueryCommands::WordLookup(args)]
            .filter_async(crate::inlines::drop_empty)
            .map_async(Self::get_definitions)
            .map(Self::compose_response)
            .filter_map_async(Self::retrieve_or_generic_err)
            .endpoint(Self::respond)
    }
}
