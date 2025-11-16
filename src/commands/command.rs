use crate::bloc::help::HelpHandler;
use crate::bloc::phrase_lookup::PhraseLookupHandler;
use crate::bloc::start::StartHandler;
use crate::bloc::teapot::TeapotHandler;
use crate::bloc::thesaurus_lookup::ThesaurusLookupHandler;
use crate::bloc::unknown::UnknownHandler;
use crate::bloc::urban_lookup::UrbanLookupHandler;
use crate::bloc::word_lookup::WordLookupHandler;
use crate::bloc::wordle::WordleHandler;
use crate::bot::MessageBot;
use teloxide::dispatching::{DpHandlerDescription, UpdateFilterExt};
use teloxide::dptree::{Endpoint, Handler};
use teloxide::prelude::{Message, Update};
use teloxide::types::Me;
use teloxide::utils::command::{BotCommands, ParseError};
use teloxide::Bot;

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
}
fn extract_text_command(text: &str) -> MessageCommands {
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

pub type CommandHandler = Endpoint<'static, anyhow::Result<()>, DpHandlerDescription>;

pub fn commands_tree() -> Handler<'static, anyhow::Result<()>, DpHandlerDescription> {
    Update::filter_message()
        .map(extract_command)
        .map(|bot: Bot, message: Message| MessageBot { bot, message })
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
