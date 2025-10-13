use crate::bloc::common::{CommonLookup, EscapingEntity, HandlerOwner, Lookup};
use crate::bloc::word_lookup::WordLookup;
use crate::commands::{drop_empty, CommandHandler, FullMessageFormatter, MessageCommands};
use crate::stands4::{AbbreviationDefinition, WordDefinition};
use shuttle_runtime::async_trait;
use teloxide::types::Message;

#[derive(Debug, Clone)]
pub struct MessageWordLookup;

#[async_trait]
impl WordLookup for MessageWordLookup {
    type Formatter = FullMessageFormatter;
    fn on_empty() -> String {
        "Found 0 definitions".to_string()
    }
}
#[async_trait]
impl Lookup for MessageWordLookup {
    type Request = Message;
    type Entity = (Vec<WordDefinition>, Vec<AbbreviationDefinition>);
    type Response = String;
}

impl HandlerOwner for MessageWordLookup {
    fn handler() -> CommandHandler {
        teloxide::dptree::case![MessageCommands::WordLookup(args)]
            .filter_async(drop_empty)
            .map_async(Self::get_definitions)
            .map(Self::escaped_values)
            .map(Self::compose_response)
            .filter_map_async(Self::retrieve_or_generic_err)
            .endpoint(Self::respond)
    }
}
