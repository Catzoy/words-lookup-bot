use crate::bloc::common::{InlineLookup, Lookup};
use crate::bloc::word_lookup::WordLookup;
use crate::commands::CommandHandler;
use crate::inlines::{formatting::InlineFormatter, QueryCommands};
use crate::stands4::{AbbreviationDefinition, WordDefinition};
use shuttle_runtime::async_trait;
use teloxide::prelude::InlineQuery;
use teloxide::types::InlineQueryResult;

#[derive(Debug, Clone)]
pub struct InlinesWordLookup;

#[async_trait]
impl WordLookup for InlinesWordLookup {
    type Formatter = InlineFormatter;
}

impl Lookup for InlinesWordLookup {
    type Request = InlineQuery;
    type Entity = (Vec<WordDefinition>, Vec<AbbreviationDefinition>);
    type Response = Vec<InlineQueryResult>;

    fn handler() -> CommandHandler {
        teloxide::dptree::case![QueryCommands::WordLookup(args)]
            .filter_async(crate::inlines::drop_empty)
            .map_async(Self::get_definitions)
            .map(Self::compose_response)
            .filter_map(Self::ensure_built_response)
            .endpoint(Self::respond_inline)
    }
}
