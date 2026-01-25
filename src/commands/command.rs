use crate::bloc::common::CommandHandler;
use crate::bloc::help::HelpHandler;
use crate::bloc::phrase_lookup::PhraseLookupHandler;
use crate::bloc::start::StartHandler;
use crate::bloc::teapot::TeapotHandler;
use crate::bloc::thesaurus_lookup::ThesaurusLookupHandler;
use crate::bloc::unknown::UnknownHandler;
use crate::bloc::urban_lookup::UrbanLookupHandler;
use crate::bloc::word_finder::WordFinderHandler;
use crate::bloc::word_lookup::WordLookupHandler;
use crate::bloc::wordle::WordleHandler;
use crate::bot::MessageBot;
use teloxide::Bot;
use teloxide::dispatching::UpdateFilterExt;
use teloxide::prelude::{Message, Update};
use teloxide::types::Me;
use teloxide::utils::command::{BotCommands, ParseError};

#[derive(Clone, BotCommands, Debug)]
#[command(
    rename_rule = "lowercase",
    description = "Here are the supported commands:"
)]
pub enum MessageCommands {
    #[command(hide)]
    Unknown,
    #[command(description = "Print this helpful message")]
    Help,
    #[command(description = "Doesn't really do anything, is just here to greet you.")]
    Start,
    #[command(hide)]
    Teapot,
    #[command(
        rename = "word",
        description = "Find definition of the specified word.\n\
        Any message containing at most 1 word, even with hyphens, will be looked up.\n\
        Also you can request to look up a word in any chat by writing `@WordsLookupBot look`"
    )]
    WordLookup(String),
    #[command(
        rename = "phrase",
        description = "Find definition of the specified phrase.\n\
        Any message with more than 1 word is considered to be a phrase.\n\
        Also you can request to look up a phrase in any chat just by writing `@WordsLookupBot look up`"
    )]
    PhraseLookup(String),
    #[command(description = "Get definition(s) of today's wordle.\n\
        Can also be sent in any chat tagging the bot and picking \"Send Today's wordle definition\"")]
    Wordle,
    #[command(
        description = "Get definition(s) of a word or a phrase from UrbanDictionary.\n\
        You can also look up words right in the chat by writing `@WordsLookupBot u.word`,\
        where `u.` will point to look in the UrbanDictionary"
    )]
    Urban(String),
    #[command(
        description = "Get synonyms, antonyms, and a definition of a given word or a phrase.\n\
        You can also look up these things right in the chat by writing `@WordsLookupBot sa.word`,\
        where `sa.` will point to look in the Thesaurus"
    )]
    Thesaurus(String),
    #[command(
        description = "Get a list of words that have characters at specified positions.\n\
        For example, `___ly` will return all 5-letter words that end with `ly`.\
        Also you can request to look up a word in any chat by writing `@WordsLookupBot f.___ly`, \
        where `f.` will point try match words against the specified mask.\
        Furthermore, you can specify a list of letters to exclude from being used in a word, \
        just add a comma and a continuous string, like a `wqg`"
    )]
    Finder(String),
}
/// Convert plain text into a MessageCommands value based on word count and content.
///
/// Maps empty input to `Teapot`; a single word containing an underscore to `Finder(word)`; a single other word to `WordLookup(word)`; and multiple words to `PhraseLookup(phrase)` where words are lowercased and joined with single spaces.
///
/// # Examples
///
/// ```
/// assert_eq!(extract_text_command(""), MessageCommands::Teapot);
/// assert_eq!(extract_text_command("Hello"), MessageCommands::WordLookup("hello".into()));
/// assert_eq!(extract_text_command("f__nd_me"), MessageCommands::Finder("f__nd_me".into()));
/// assert_eq!(extract_text_command("Hello WORLD"), MessageCommands::PhraseLookup("hello world".into()));
/// ```
fn extract_text_command(text: &str) -> MessageCommands {
    if text.contains("_") {
        return MessageCommands::Finder(text.to_owned());
    }

    let words = text
        .split_whitespace()
        .map(|s| s.to_lowercase())
        .collect::<Vec<String>>();
    match &words[..] {
        [] => MessageCommands::Teapot,
        [word] => MessageCommands::WordLookup(word.to_owned()),
        _ => MessageCommands::PhraseLookup(words.join(" ")),
    }
}
/// Parse a Telegram message together with the bot identity into a `MessageCommands` variant.
///
/// Attempts to parse the message text as a bot command (taking the bot's username into account).
/// If parsing yields an unknown slash command (starts with `/`) the function returns
/// `MessageCommands::Unknown`. For other parse failures it falls back to text-based extraction:
/// a single word becomes `MessageCommands::WordLookup`, multiple words become
/// `MessageCommands::PhraseLookup`.
///
/// # Parameters
///
/// - `message` — the incoming Telegram `Message` whose text should be interpreted as a command or lookup.
/// - `me` — the bot's `Me` identity used to resolve username-qualified commands.
///
/// # Returns
///
/// A `MessageCommands` value representing the resolved command or lookup.
///
/// # Examples
///
/// ```
/// // Construct appropriate `Message` and `Me` values in your test harness and call:
/// // let cmd = extract_command(message, me);
/// // assert!(matches!(cmd, MessageCommands::WordLookup(_) | MessageCommands::PhraseLookup(_)));
/// ```
fn extract_command(message: Message, me: Me) -> MessageCommands {
    let text = message.text().unwrap_or_default();
    let username = me.username.clone().unwrap_or_default();
    let cmd = MessageCommands::parse(text, &username).unwrap_or_else(|err| match err {
        ParseError::UnknownCommand(cmd) if cmd.starts_with("/") => MessageCommands::Unknown,
        _ => extract_text_command(text),
    });

    log::info!("Received message: {:?}", text);
    log::info!("Processing command {:?}", cmd);
    cmd
}

/// Builds the update dispatch tree that routes incoming message updates to their command handlers.
///
/// The handler filters for message updates, converts each message into a `MessageCommands` value,
/// wraps the bot and message into a `MessageBot`, and dispatches to the matching handler branch
/// (Finder, Wordle, WordLookup, PhraseLookup, Urban, Thesaurus, Help, Unknown, Start, Teapot).
///
/// # Examples
///
/// ```
/// let handler = commands_tree();
/// // Attach `handler` to a teloxide dispatcher to process incoming updates.
/// ```
pub fn commands_tree() -> CommandHandler {
    Update::filter_message()
        .map(extract_command)
        .inspect(|message: Message| {
            log::debug!("Answering chat {:?}", message.chat.id);
        })
        .map(|bot: Bot, message: Message| MessageBot { bot, message })
        .branch(
            teloxide::dptree::case![MessageCommands::Finder(mask)]
                .branch(MessageBot::word_finder_handler()),
        )
        .branch(
            teloxide::dptree::case![MessageCommands::Wordle].branch(MessageBot::wordle_handler()),
        )
        .branch(
            teloxide::dptree::case![MessageCommands::WordLookup(args)]
                .branch(MessageBot::word_lookup_handler()),
        )
        .branch(
            teloxide::dptree::case![MessageCommands::PhraseLookup(phrase)]
                .branch(MessageBot::phrase_lookup_handler()),
        )
        .branch(
            teloxide::dptree::case![MessageCommands::Urban(phrase)]
                .branch(MessageBot::urban_lookup_handler()),
        )
        .branch(
            teloxide::dptree::case![MessageCommands::Thesaurus(word)]
                .branch(MessageBot::thesaurus_lookup_handler()),
        )
        .branch(teloxide::dptree::case![MessageCommands::Help].branch(MessageBot::help_handler()))
        .branch(
            teloxide::dptree::case![MessageCommands::Unknown].branch(MessageBot::unknown_handler()),
        )
        .branch(teloxide::dptree::case![MessageCommands::Start].branch(MessageBot::start_handler()))
        .branch(
            teloxide::dptree::case![MessageCommands::Teapot].branch(MessageBot::teapot_handler()),
        )
}
