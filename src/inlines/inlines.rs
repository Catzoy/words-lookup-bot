use crate::inlines::{
    debounce_inline_queries,
    phrase_lookup,
    suggestions,
    urban_lookup,
    word_lookup,
};
use teloxide::{
    dispatching::{DpHandlerDescription, UpdateFilterExt},
    dptree::Handler,
    prelude::Requester,
    prelude::{InlineQuery, Update},
    Bot,
};

#[derive(Debug, Clone)]
pub enum QueryCommands {
    Suggestions,
    WordLookup(String),
    PhraseLookup(String),
    UrbanLookup(String),
}
const URBAN_IDENTIFIER: &str = "u";
pub type InlineHandler = Handler<'static, anyhow::Result<()>, DpHandlerDescription>;
fn extract_command(InlineQuery { query, .. }: InlineQuery) -> QueryCommands {
    let words = query.split_whitespace()
        .map(|s| s.to_lowercase())
        .collect::<Vec<String>>();
    match &words[..] {
        [] => QueryCommands::Suggestions,
        [first] if first == URBAN_IDENTIFIER && query.len() > 1 => QueryCommands::UrbanLookup(String::default()),
        [word] => QueryCommands::WordLookup(word.to_owned()),
        [first, rest @ .. ] if first == URBAN_IDENTIFIER => QueryCommands::UrbanLookup(rest.join(" ")),
        _ => QueryCommands::PhraseLookup(words.join(" ")),
    }
}

pub async fn drop_empty(bot: Bot, InlineQuery { id, .. }: InlineQuery, input: String) -> bool {
    match input.as_str() {
        "" => {
            let _ = bot.answer_inline_query(id, vec![]).await;
            false
        }
        _ => true
    }
}


pub fn inlines_tree() -> Handler<'static, anyhow::Result<()>, DpHandlerDescription> {
    Update::filter_inline_query()
        .map(extract_command)
        .filter_async(debounce_inline_queries)
        .branch(suggestions())
        .branch(word_lookup())
        .branch(phrase_lookup())
        .branch(urban_lookup())
}