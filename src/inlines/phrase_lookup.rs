use crate::bloc::common::{CommonLookup, HandlerOwner, Lookup};
use crate::bloc::phrase_lookup::PhraseLookup;
use crate::commands::CommandHandler;
use crate::inlines::{formatting::InlineFormatter, QueryCommands};
use crate::stands4::PhraseDefinition;
use teloxide::prelude::InlineQuery;
use teloxide::types::InlineQueryResult;

#[derive(Clone)]
pub struct InlinePhraseLookup;

impl PhraseLookup for InlinePhraseLookup {
    type Formatter = InlineFormatter;
}

impl Lookup for InlinePhraseLookup {
    type Request = InlineQuery;
    type Entity = Vec<PhraseDefinition>;
    type Response = Vec<InlineQueryResult>;
}

impl HandlerOwner for InlinePhraseLookup {
    fn handler() -> CommandHandler {
        teloxide::dptree::case![QueryCommands::PhraseLookup(phrase)]
            .filter_async(crate::inlines::drop_empty)
            .map_async(Self::get_definitions)
            .filter_map_async(Self::ensure_request_success)
            .map(Self::compose_response)
            .filter_map_async(Self::retrieve_or_generic_err)
            .endpoint(Self::respond)
    }
}
