use crate::bloc::common::{InlineLookup, Lookup};
use crate::bloc::word_lookup::WordLookup;
use crate::commands::{drop_empty, CommandHandler};
use crate::inlines::{formatting::InlineFormatter, QueryCommands};
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
    type Response = Vec<InlineQueryResult>;

    fn handler() -> CommandHandler {
        teloxide::dptree::case![QueryCommands::WordLookup(args)]
            .filter_async(drop_empty)
            .map_async(Self::get_definitions)
            .map(Self::compose_response)
            .endpoint(Self::respond_inline)
    }
}
