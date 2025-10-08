use crate::bloc::common::{Lookup, MessageLookup};
use crate::bloc::word_lookup::WordLookup;
use crate::commands::{drop_empty, CommandHandler, FullMessageFormatter, MessageCommands};
use shuttle_runtime::async_trait;
use teloxide::types::Message;

#[derive(Debug, Clone)]
pub struct MessageWordLookup;

#[async_trait]
impl WordLookup for MessageWordLookup {
    type Formatter = FullMessageFormatter;
    fn on_empty(&self) -> String {
        "Found 0 definitions".to_string()
    }
}
#[async_trait]
impl Lookup for MessageWordLookup {
    type Request = Message;
    type Response = String;

    fn handler() -> CommandHandler {
        teloxide::dptree::case![MessageCommands::WordLookup(args)]
            .filter_async(drop_empty)
            .map_async(Self::get_definitions)
            .map(Self::compose_response)
            .filter_map_async(Self::retrieve_or_generic_err)
            .endpoint(Self::respond_message)
    }
}
