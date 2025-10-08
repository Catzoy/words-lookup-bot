use crate::bloc::common::{InlineLookup, Lookup};
use crate::bloc::thesaurus_lookup::ThesaurusLookup;
use crate::commands::CommandHandler;
use crate::inlines::formatting::InlineFormatter;
use crate::inlines::QueryCommands;
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

    fn handler() -> CommandHandler {
        teloxide::dptree::case![QueryCommands::ThesaurusLookup(args)]
            .filter_async(crate::inlines::drop_empty)
            .map_async(Self::get_definitions)
            .filter_map_async(Self::ensure_query_success)
            .map(Self::compose_response)
            .filter_map(Self::ensure_built_response)
            .endpoint(Self::respond_inline)
    }
}
