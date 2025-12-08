use crate::bloc::common::CommandHandler;
use crate::bloc::phrase_lookup::PhraseLookupHandler;
use crate::bloc::suggestions::SuggestionsHandler;
use crate::bloc::thesaurus_lookup::ThesaurusLookupHandler;
use crate::bloc::urban_lookup::UrbanLookupHandler;
use crate::bloc::word_finder::WordFinderHandler;
use crate::bloc::word_lookup::WordLookupHandler;
use crate::bot::InlineBot;
use crate::inlines::debounce_inline_queries;
use regex::Regex;
use std::sync::LazyLock;
use teloxide::{
    dispatching::UpdateFilterExt,
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
    Finder(String),
}
static COMMAND_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(u.|sa.|f.)?(.+)").unwrap());
/// Parse an inline query string and map it to a `QueryCommands` variant.
///
/// The function examines the query text and selects a command according to these rules:
/// - An empty query yields `QueryCommands::Suggestions`.
/// - If the pattern has no explicit prefix:
///   - a single word becomes `QueryCommands::WordLookup(word)`,
///   - multiple words become `QueryCommands::PhraseLookup(phrase)`,
///   - no words becomes `QueryCommands::Suggestions`.
/// - A prefix of `u.` with trailing text becomes `QueryCommands::UrbanLookup(phrase)`.
/// - A prefix of `sa.` with trailing text becomes `QueryCommands::ThesaurusLookup(phrase)`.
/// - Any unmatched input yields `QueryCommands::Suggestions`.
///
/// # Returns
///
/// `Some(QueryCommands::...)` with the parsed command when the query matches the command pattern, or
/// `None` if the query does not match `COMMAND_PATTERN`.
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
        (Some(m), Some(phrase)) if m.as_str().eq("f.") => {
            QueryCommands::Finder(phrase.as_str().to_string())
        }
        _ => QueryCommands::Suggestions,
    };
    Some(cmd)
}

/// Builds the inline-query command handler that routes parsed inline queries to their specific handlers.
///
/// The returned handler accepts inline queries, parses them into a command variant, applies debounce filtering,
/// and dispatches each command to the corresponding inline handler (suggestions, word lookup, phrase lookup,
/// urban lookup, or thesaurus lookup).
///
/// # Examples
///
/// ```
/// // Construct the handler and keep it for registration with the dispatcher.
/// let handler = crate::inlines_tree();
/// let _ = handler;
/// ```
pub fn inlines_tree() -> CommandHandler {
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
        .branch(
            teloxide::dptree::case![QueryCommands::Finder(phrase)]
                .branch(InlineBot::word_finder_handler()),
        )
}
