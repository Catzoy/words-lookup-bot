use crate::bloc::common::{CommonLookup, HandlerOwner, Lookup};
use crate::bloc::thesaurus_lookup::ThesaurusLookup;
use crate::commands::{drop_empty, CommandHandler, FullMessageFormatter, MessageCommands};
use crate::stands4::SynAntDefinitions;
use teloxide::prelude::Message;

#[derive(Debug, Clone)]
pub struct MessageThesaurusLookup;

impl ThesaurusLookup for MessageThesaurusLookup {
    type Formatter = FullMessageFormatter;
}

impl Lookup for MessageThesaurusLookup {
    type Request = Message;
    type Entity = Vec<SynAntDefinitions>;
    type Response = String;
}

impl HandlerOwner for MessageThesaurusLookup {
    fn handler() -> CommandHandler {
        teloxide::dptree::case![MessageCommands::Thesaurus(args)]
            .filter_async(drop_empty)
            .map_async(Self::get_definitions)
            .filter_map_async(Self::ensure_request_success)
            .map(Self::compose_response)
            .filter_map_async(Self::retrieve_or_generic_err)
            .endpoint(Self::respond)
    }
}
