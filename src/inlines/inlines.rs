use crate::inlines::debounce_inline_queries;
use crate::{
    inlines::phrase_lookup,
    inlines::suggestions,
    inlines::word_lookup,
};
use teloxide::prelude::Requester;
use teloxide::{dispatching::{DpHandlerDescription, UpdateFilterExt}, dptree::Handler, prelude::{InlineQuery, Update}, Bot};

#[derive(Debug, Clone)]
pub enum QueryCommands {
    Suggestions,
    WordLookup(String),
    PhraseLookup(String),
}

pub type InlineHandler = Handler<'static, anyhow::Result<()>, DpHandlerDescription>;

fn extract_command(query: InlineQuery) -> QueryCommands {
    let words = query.query.split_whitespace()
        .map(|s| s.to_lowercase())
        .collect::<Vec<String>>();
    match &words[..] {
        [] => QueryCommands::Suggestions,
        [word] => QueryCommands::WordLookup(word.to_owned()),
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
}