use crate::bloc::common::{HandlerOwner, InlineLookup, Lookup};
use crate::bloc::urban_lookup::UrbanLookup;
use crate::inlines::{formatting::InlineFormatter, InlineHandler, QueryCommands};
use crate::urban::UrbanDefinition;
use teloxide::prelude::InlineQuery;
use teloxide::types::InlineQueryResult;

#[derive(Debug, Clone)]
pub struct InlineUrbanLookup;
impl UrbanLookup for InlineUrbanLookup {
    type Formatter = InlineFormatter;
}
impl Lookup for InlineUrbanLookup {
    type Request = InlineQuery;
    type Entity = Vec<UrbanDefinition>;
    type Response = Vec<InlineQueryResult>;
}

impl HandlerOwner for InlineUrbanLookup {
    fn handler() -> InlineHandler {
        teloxide::dptree::case![QueryCommands::UrbanLookup(args)]
            .filter_async(crate::inlines::drop_empty)
            .map_async(Self::get_definitions)
            .filter_map_async(Self::ensure_query_success)
            .map(Self::compose_response)
            .filter_map(Self::ensure_built_response)
            .endpoint(Self::respond_inline)
    }
}
