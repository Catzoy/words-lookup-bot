use crate::bloc::common::HandlerOwner;
use crate::inlines::phrase_lookup::InlinePhraseLookup;
use crate::inlines::suggestions::SuggestionsOwner;
use crate::inlines::thesaurus_lookup::InlineThesaurusLookup;
use crate::inlines::urban_lookup::InlineUrbanLookup;
use crate::inlines::word_lookup::InlinesWordLookup;
use crate::inlines::debounce_inline_queries;
use regex::Regex;
use std::sync::LazyLock;
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
    ThesaurusLookup(String),
}
static COMMAND_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(u.|sa.)?(.+)").unwrap());
pub type InlineHandler = Handler<'static, anyhow::Result<()>, DpHandlerDescription>;
fn extract_command(InlineQuery { query, .. }: InlineQuery) -> Option<QueryCommands> {
    if query.is_empty() {
        return Some(QueryCommands::Suggestions);
    }

    let captures = COMMAND_PATTERN.captures(&query)?;
    let cmd = match (captures.get(1), captures.get(2)) {
        (None, Some(input)) => {
            let words = input
                .as_str()
                .split_whitespace()
                .map(|s| s.to_lowercase())
                .collect::<Vec<String>>();
            match &words[..] {
                [] => QueryCommands::Suggestions,
                [word] => QueryCommands::WordLookup(word.to_owned()),
                _ => QueryCommands::PhraseLookup(words.join(" ")),
            }
        }
        (Some(m), Some(phrase)) if m.as_str().eq("u.") => {
            QueryCommands::UrbanLookup(phrase.as_str().to_string())
        }
        (Some(m), Some(phrase)) if m.as_str().eq("sa.") => {
            QueryCommands::ThesaurusLookup(phrase.as_str().to_string())
        }
        _ => QueryCommands::Suggestions,
    };
    Some(cmd)
}

pub async fn drop_empty(bot: Bot, InlineQuery { id, .. }: InlineQuery, input: String) -> bool {
    match input.as_str() {
        "" => {
            let _ = bot.answer_inline_query(id, vec![]).await;
            false
        }
        _ => true,
    }
}

pub fn inlines_tree() -> Handler<'static, anyhow::Result<()>, DpHandlerDescription> {
    Update::filter_inline_query()
        .filter_map(extract_command)
        .filter_async(debounce_inline_queries)
        .branch(SuggestionsOwner::handler())
        .branch(InlinesWordLookup::handler())
        .branch(InlinePhraseLookup::handler())
        .branch(InlineUrbanLookup::handler())
        .branch(InlineThesaurusLookup::handler())
}
