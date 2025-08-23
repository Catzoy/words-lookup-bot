use crate::{
    inlines::phrase_lookup,
    inlines::word_lookup,
};
use teloxide::{
    dispatching::{DpHandlerDescription, UpdateFilterExt},
    dptree::Handler,
    prelude::{InlineQuery, Update},
};

#[derive(Debug, Clone)]
pub enum QueryCommands {
    WordLookup(String),
    PhraseLookup(String),
}

pub type InlineHandler = Handler<'static, anyhow::Result<()>, DpHandlerDescription>;

fn extract_command(query: InlineQuery) -> Option<QueryCommands> {
    let words = query.query.split_whitespace()
        .map(|s| s.to_lowercase())
        .collect::<Vec<String>>();
    match &words[..] {
        [] => None,
        [word] => Some(QueryCommands::WordLookup(word.to_owned())),
        _ => Some(QueryCommands::PhraseLookup(words.join(" "))),
    }
}

pub fn inlines_tree() -> Handler<'static, anyhow::Result<()>, DpHandlerDescription> {
    Update::filter_inline_query()
        .filter_map(extract_command)
        .branch(word_lookup())
        .branch(phrase_lookup())
}