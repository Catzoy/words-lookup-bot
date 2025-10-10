use crate::bloc::common::{CommonLookup, HandlerOwner, Lookup};
use crate::bloc::thesaurus_lookup::ThesaurusLookup;
use crate::inlines::formatting::InlineFormatter;
use crate::inlines::{InlineHandler, QueryCommands};
use crate::stands4::SynAntDefinitions;
use teloxide::types::{InlineQuery, InlineQueryResult};

#[derive(Debug, Clone)]
pub struct InlineThesaurusLookup;

impl ThesaurusLookup for InlineThesaurusLookup {
    type Formatter = InlineFormatter;
}
impl Lookup for InlineThesaurusLookup {
    type Request = InlineQuery;
    type Entity = Vec<SynAntDefinitions>;
    type Response = Vec<InlineQueryResult>;
}

impl HandlerOwner for InlineThesaurusLookup {
    fn handler() -> InlineHandler {
        teloxide::dptree::case![QueryCommands::ThesaurusLookup(args)]
            .filter_async(crate::inlines::drop_empty)
            .map_async(Self::get_definitions)
            .filter_map_async(Self::ensure_request_success)
            .map(Self::compose_response)
            .filter_map_async(Self::retrieve_or_generic_err)
            .endpoint(Self::respond)
    }
}
