use crate::bloc::common::{Lookup, MessageLookup};
use crate::bloc::phrase_lookup::PhraseLookup;
use crate::commands::{drop_empty, CommandHandler, FullMessageFormatter, MessageCommands};
use teloxide::prelude::Message;

#[derive(Clone, Debug)]
pub struct MessagePhraseLookup;

impl PhraseLookup for MessagePhraseLookup {
    type Formatter = FullMessageFormatter;
}
impl Lookup for MessagePhraseLookup {
    type Request = Message;
    type Response = String;

    fn handler() -> CommandHandler {
        teloxide::dptree::case![MessageCommands::PhraseLookup(args)]
            .filter_async(drop_empty)
            .map_async(Self::get_definitions)
            .map(Self::compose_response)
            .filter_map_async(Self::retrieve_or_generic_err)
            .endpoint(Self::respond_message)
    }
}
