use crate::bloc::common::{Lookup, MessageLookup};
use crate::bloc::urban_lookup::UrbanLookup;
use crate::commands::{drop_empty, CommandHandler, FullMessageFormatter, MessageCommands};
use crate::urban::UrbanDefinition;
use teloxide::prelude::Message;

#[derive(Debug, Clone)]
pub struct MessageUrbanLookup;

impl UrbanLookup for MessageUrbanLookup {
    type Formatter = FullMessageFormatter;
}

impl Lookup for MessageUrbanLookup {
    type Request = Message;
    type Entity = Vec<UrbanDefinition>;
    type Response = String;

    fn handler() -> CommandHandler {
        teloxide::dptree::case![MessageCommands::Urban(term)]
            .filter_async(drop_empty)
            .map_async(Self::get_definitions)
            .filter_map_async(Self::ensure_request_success)
            .map(Self::compose_response)
            .filter_map_async(Self::retrieve_or_generic_err)
            .endpoint(Self::respond_message)
    }
}
