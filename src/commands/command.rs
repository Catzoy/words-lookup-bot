use crate::bloc::common::HandlerOwner;
use crate::bloc::help::HelpOwner;
use crate::bloc::phrase_lookup::PhraseLookup;
use crate::bloc::start::StartOwner;
use crate::bloc::teapot::TeapotOwner;
use crate::bloc::thesaurus_lookup::ThesaurusLookup;
use crate::bloc::unknown::UnknownOwner;
use crate::bloc::urban_lookup::UrbanLookup;
use crate::bloc::word_lookup::WordLookup;
use crate::bloc::wordle::WordleLookup;
use crate::bot::LookupBot;
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
type MessageBot = LookupBot<Message>;

pub fn commands_tree() -> Handler<'static, anyhow::Result<()>, DpHandlerDescription> {
    Update::filter_message()
        .map(extract_command)
        .map(|bot: Bot, message: Message| LookupBot {
            bot,
            request: message,
        })
        .branch(
            teloxide::dptree::case![MessageCommands::Wordle]
                .branch(WordleLookup::handler::<MessageBot>()),
        )
        .branch(
            teloxide::dptree::case![MessageCommands::WordLookup(args)]
                .branch(WordLookup::handler::<MessageBot>()),
        )
        .branch(
            teloxide::dptree::case![MessageCommands::PhraseLookup(phrase)]
                .branch(PhraseLookup::handler::<MessageBot>()),
        )
        .branch(
            teloxide::dptree::case![MessageCommands::Urban(phrase)]
                .branch(UrbanLookup::handler::<MessageBot>()),
        )
        .branch(
            teloxide::dptree::case![MessageCommands::Thesaurus(word)]
                .branch(ThesaurusLookup::handler::<MessageBot>()),
        )
        .branch(
            teloxide::dptree::case![MessageCommands::Help]
                .branch(HelpOwner::handler::<MessageBot>()),
        )
        .branch(
            teloxide::dptree::case![MessageCommands::Unknown]
                .branch(UnknownOwner::handler::<MessageBot>()),
        )
        .branch(
            teloxide::dptree::case![MessageCommands::Start]
                .branch(StartOwner::handler::<MessageBot>()),
        )
        .branch(
            teloxide::dptree::case![MessageCommands::Teapot]
                .branch(TeapotOwner::handler::<MessageBot>()),
        )
}
