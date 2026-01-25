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

static TEXT_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^([a-z_ ]+)$").unwrap());
static URBAN_PATTER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(u)\.([a-z ]+)$").unwrap());
static SYNO_PATTER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(sa)\.([a-z]+)$").unwrap());
static FINDER_PATTER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(f)\.([a-z_]+,? ?[a-z]+)?$").unwrap());
#[derive(Debug, Clone, PartialEq)]
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
            "u" => Some(CommandTag::Urban),
            "sa" => Some(CommandTag::Thesaurus),
            "f" => Some(CommandTag::Finder),
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
fn extract_command(query: String) -> Option<QueryCommands> {
    if query.is_empty() {
        return Some(QueryCommands::Suggestions);
    }

    let query = query.to_lowercase();
    URBAN_PATTER
        .captures(&query)
        .or_else(|| SYNO_PATTER.captures(&query))
        .or_else(|| FINDER_PATTER.captures(&query))
        .and_then(|captures| {
            let tag = captures.get(1)?.as_str();
            let tag = CommandTag::from(tag)?;
            let input = captures.get(2)?.as_str();
            Some((tag, input.to_owned()))
        })
        .map(|(tag, input)| match tag {
            CommandTag::Urban => QueryCommands::UrbanLookup(input.to_owned()),
            CommandTag::Thesaurus => QueryCommands::ThesaurusLookup(input.to_owned()),
            CommandTag::Finder => QueryCommands::Finder(input.to_owned()),
        })
        .or_else(|| {
            let input = TEXT_PATTERN.captures(&query)?;
            let input = input.get(1)?.as_str();
            if input.contains("_") {
                return Some(QueryCommands::Finder(input.to_owned()));
            }

            let words = input
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>();
            let cmd = match &words[..] {
                [] => QueryCommands::Suggestions,
                [word] => QueryCommands::WordLookup(word.to_owned()),
                _ => QueryCommands::PhraseLookup(words.join(" ")),
            };
            Some(cmd)
        })
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
        .filter_map(|InlineQuery { query, .. }: InlineQuery| extract_command(query))
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

mod tests {
    use super::*;

    #[test]
    fn empty_input_displays_suggestions() {
        let cmd = extract_command("".to_owned());
        assert_eq!(cmd, Some(QueryCommands::Suggestions));
    }

    #[test]
    fn unrecognized_chars_display_nothing() {
        let cmd = extract_command("123".to_owned());
        assert_eq!(cmd, None);
    }

    #[test]
    fn incomplete_query_display_nothing() {
        let cmd = extract_command("u.".to_owned());
        assert_eq!(cmd, None);
    }
    #[test]
    fn u_displays_urban() {
        let cmd = extract_command("u.urban".to_owned());
        assert_eq!(cmd, Some(QueryCommands::UrbanLookup("urban".to_owned())));
    }

    #[test]
    fn sa_displays_thesaurus() {
        let cmd = extract_command("sa.thesaurus".to_owned());
        assert_eq!(
            cmd,
            Some(QueryCommands::ThesaurusLookup("thesaurus".to_owned()))
        );
    }

    #[test]
    fn f_displays_finder() {
        let cmd = extract_command("f.f__der".to_owned());
        assert_eq!(cmd, Some(QueryCommands::Finder("f__der".to_owned())));
    }
    #[test]
    fn f_banned_displays_finder() {
        let cmd = extract_command("f.f__der, xxx".to_owned());
        assert_eq!(cmd, Some(QueryCommands::Finder("f__der, xxx".to_owned())));
    }

    #[test]
    fn underscores_display_finder() {
        let cmd = extract_command("___ly".to_owned());
        assert_eq!(cmd, Some(QueryCommands::Finder("___ly".to_owned())));
    }

    #[test]
    fn single_word_displays_word_lookup() {
        let cmd = extract_command("look".to_owned());
        assert_eq!(cmd, Some(QueryCommands::WordLookup("look".to_owned())));
    }
    #[test]
    fn multiple_words_display_phrase_lookup() {
        let cmd = extract_command("turn down".to_owned());
        assert_eq!(
            cmd,
            Some(QueryCommands::PhraseLookup("turn down".to_owned()))
        );
    }
}
