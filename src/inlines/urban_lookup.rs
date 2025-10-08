use crate::bloc::common::{InlineLookup, Lookup};
use crate::bloc::urban_lookup::UrbanLookup;
use crate::commands::CommandHandler;
use crate::inlines::{formatting::InlineFormatter, QueryCommands};
use teloxide::types::InlineQueryResult;
use teloxide::prelude::InlineQuery;

#[derive(Debug, Clone)]
pub struct InlineUrbanLookup;
impl UrbanLookup for InlineUrbanLookup {
    type Formatter = InlineFormatter;
}
impl Lookup for InlineUrbanLookup {
    type Request = InlineQuery;
    type Response = Vec<InlineQueryResult>;

    fn handler() -> CommandHandler {
        teloxide::dptree::case![QueryCommands::UrbanLookup(args)]
            .filter_async(crate::commands::drop_empty)
            .map_async(Self::get_definitions)
            .map(Self::compose_response)
            .endpoint(Self::respond_inline)
    }
}
