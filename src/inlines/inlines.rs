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
    Bot,
    dispatching::UpdateFilterExt,
    prelude::{InlineQuery, Update},
};

static COMMAND_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(u.|sa.|f.)?(.+)").unwrap());
#[derive(Debug, Clone)]
pub enum QueryCommands {
    Suggestions,
    WordLookup(String),
    PhraseLookup(String),
    UrbanLookup(String),
    ThesaurusLookup(String),
    Finder(String),
}

enum CommandTag {
    Urban,
    Thesaurus,
    Finder,
}

impl CommandTag {
    fn from<S: Into<String>>(str: S) -> Option<Self> {
        match str.into().as_str() {
            "u." => Some(CommandTag::Urban),
            "sa." => Some(CommandTag::Thesaurus),
            "f." => Some(CommandTag::Finder),
            _ => None,
        }
    }
}

/// Map an inline query string to a `QueryCommands` variant.
///
/// Parses the query text using `COMMAND_PATTERN` and selects a command:
/// - empty query → `QueryCommands::Suggestions`
/// - no explicit prefix:
///   - one word → `QueryCommands::WordLookup(word)`
///   - multiple words → `QueryCommands::PhraseLookup(phrase)`
/// - `u.` prefix → `QueryCommands::UrbanLookup(phrase)`
/// - `sa.` prefix → `QueryCommands::ThesaurusLookup(phrase)`
/// - `f.` prefix → `QueryCommands::Finder(phrase)`
///
/// # Returns
///
/// `Some(QueryCommands::...)` with the parsed command when the query matches `COMMAND_PATTERN`, or
/// `None` if the query does not match `COMMAND_PATTERN`.
///
/// # Examples
///
/// ```
/// // Illustrative usage:
/// // let iq = InlineQuery { query: "u.example".into(), .. };
/// // assert!(matches!(extract_command(iq), Some(QueryCommands::UrbanLookup(p)) if p == "example"));
/// ```
fn extract_command(InlineQuery { query, .. }: InlineQuery) -> Option<QueryCommands> {
    if query.is_empty() {
        return Some(QueryCommands::Suggestions);
    }

    let captures = COMMAND_PATTERN.captures(&query)?;
    let input = captures.get(2)?.as_str();
    let cmd = captures
        .get(1)
        .and_then(|m| CommandTag::from(m.as_str()))
        .map(|tag| match tag {
            CommandTag::Urban => QueryCommands::UrbanLookup(input.to_owned()),
            CommandTag::Thesaurus => QueryCommands::ThesaurusLookup(input.to_owned()),
            CommandTag::Finder => QueryCommands::Finder(input.to_owned()),
        })
        .unwrap_or_else(|| {
            if input.contains("_") {
                return QueryCommands::Finder(input.to_owned());
            }

            let words = input
                .split_whitespace()
                .map(|s| s.to_lowercase())
                .collect::<Vec<String>>();
            match &words[..] {
                [] => QueryCommands::Suggestions,
                [word] => QueryCommands::WordLookup(word.to_owned()),
                _ => QueryCommands::PhraseLookup(words.join(" ")),
            }
        });
    Some(cmd)
}

/// Builds the inline-query command handler that parses inline queries, debounces them, and dispatches each
/// parsed command to its corresponding inline handler (suggestions, word lookup, phrase lookup, urban lookup,
/// thesaurus lookup, or finder).
///
/// The handler filters incoming updates for inline queries, converts each query into a `QueryCommands` variant,
/// wraps it into an `InlineBot`, applies `debounce_inline_queries`, and routes to the appropriate handler.
///
/// # Examples
///
/// ```
/// // Construct the handler for registration with a dispatcher.
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
