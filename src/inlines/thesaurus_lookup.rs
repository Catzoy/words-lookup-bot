use crate::bloc::common::{InlineLookup, Lookup};
use crate::bloc::thesaurus_lookup::ThesaurusLookup;
use crate::commands::{drop_empty, CommandHandler};
use crate::inlines::formatting::InlineFormatter;
use crate::inlines::QueryCommands;
use teloxide::types::{InlineQuery, InlineQueryResult};

#[derive(Debug, Clone)]
pub struct InlineThesaurusLookup;

impl ThesaurusLookup for InlineThesaurusLookup {
    type Formatter = InlineFormatter;
}
impl Lookup for InlineThesaurusLookup {
    type Request = InlineQuery;
    type Response = Vec<InlineQueryResult>;

    fn handler() -> CommandHandler {
        teloxide::dptree::case![QueryCommands::ThesaurusLookup(args)]
            .filter_async(drop_empty)
            .map_async(Self::get_definitions)
            .map(Self::compose_response)
            .endpoint(Self::respond_inline)
    }
}
