use crate::bloc::common::{Lookup, MessageLookup};
use crate::bloc::thesaurus_lookup::ThesaurusLookup;
use crate::commands::{drop_empty, CommandHandler, FullMessageFormatter, MessageCommands};
use teloxide::prelude::Message;

#[derive(Debug, Clone)]
pub struct MessageThesaurusLookup;

impl ThesaurusLookup for MessageThesaurusLookup {
    type Formatter = FullMessageFormatter;
}

impl Lookup for MessageThesaurusLookup {
    type Request = Message;
    type Response = String;

    fn handler() -> CommandHandler {
        teloxide::dptree::case![MessageCommands::Thesaurus(args)]
            .filter_async(drop_empty)
            .map_async(Self::get_definitions)
            .map(Self::compose_response)
            .filter_map_async(Self::retrieve_or_generic_err)
            .endpoint(Self::respond_message)
    }
}
