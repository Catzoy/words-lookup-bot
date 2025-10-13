use crate::bloc::common::{CommonLookup, EscapingEntity, HandlerOwner, Lookup};
use crate::bloc::phrase_lookup::PhraseLookup;
use crate::commands::{drop_empty, CommandHandler, FullMessageFormatter, MessageCommands};
use crate::stands4::PhraseDefinition;
use teloxide::prelude::Message;

#[derive(Clone, Debug)]
pub struct MessagePhraseLookup;

impl PhraseLookup for MessagePhraseLookup {
    type Formatter = FullMessageFormatter;
}
impl Lookup for MessagePhraseLookup {
    type Request = Message;
    type Entity = Vec<PhraseDefinition>;
    type Response = String;
}

impl HandlerOwner for MessagePhraseLookup {
    fn handler() -> CommandHandler {
        teloxide::dptree::case![MessageCommands::PhraseLookup(args)]
            .filter_async(drop_empty)
            .map_async(Self::get_definitions)
            .filter_map_async(Self::ensure_request_success)
            .map(Self::escaped_values)
            .map(Self::compose_response)
            .filter_map_async(Self::retrieve_or_generic_err)
            .endpoint(Self::respond)
    }
}
