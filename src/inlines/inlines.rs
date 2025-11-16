use crate::bloc::phrase_lookup::PhraseLookupHandler;
use crate::bloc::suggestions::SuggestionsHandler;
use crate::bloc::thesaurus_lookup::ThesaurusLookupHandler;
use crate::bloc::urban_lookup::UrbanLookupHandler;
use crate::bloc::word_lookup::WordLookupHandler;
use crate::bot::InlineBot;
use crate::inlines::debounce_inline_queries;
use regex::Regex;
use std::sync::LazyLock;
use teloxide::{
    dispatching::{DpHandlerDescription, UpdateFilterExt},
    dptree::Handler,
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

pub fn inlines_tree() -> Handler<'static, anyhow::Result<()>, DpHandlerDescription> {
    Update::filter_inline_query()
        .filter_map(extract_command)
        .map(|bot: Bot, query: InlineQuery| InlineBot { bot, query })
        .filter_async(debounce_inline_queries)
        .branch(
            teloxide::dptree::case![QueryCommands::Suggestions]
                .branch(InlineBot::suggestions_handler()),
        )
        .branch(
            teloxide::dptree::case![QueryCommands::WordLookup(args)]
                .branch(InlineBot::word_lookup_handler()),
        )
        .branch(
            teloxide::dptree::case![QueryCommands::PhraseLookup(phrase)]
                .branch(InlineBot::phrase_lookup_handler()),
        )
        .branch(
            teloxide::dptree::case![QueryCommands::UrbanLookup(phrase)]
                .branch(InlineBot::urban_lookup_handler()),
        )
        .branch(
            teloxide::dptree::case![QueryCommands::ThesaurusLookup(phrase)]
                .branch(InlineBot::thesaurus_lookup_handler()),
        )
}
