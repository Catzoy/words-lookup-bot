use crate::bloc::common::{InlineLookup, Lookup};
use crate::bloc::phrase_lookup::PhraseLookup;
use crate::commands::CommandHandler;
use crate::inlines::{formatting::InlineFormatter, inlines::drop_empty, QueryCommands};
use teloxide::prelude::InlineQuery;
use teloxide::types::InlineQueryResult;

#[derive(Clone)]
pub struct InlinePhraseLookup;

impl PhraseLookup for InlinePhraseLookup {
    type Formatter = InlineFormatter;
}

impl Lookup for InlinePhraseLookup {
    type Request = InlineQuery;
    type Response = Vec<InlineQueryResult>;

    fn handler() -> CommandHandler {
        teloxide::dptree::case![QueryCommands::PhraseLookup(phrase)]
            .filter_async(drop_empty)
            .map_async(Self::get_definitions)
            .map(Self::compose_response)
            .endpoint(Self::respond_inline)
    }
}
