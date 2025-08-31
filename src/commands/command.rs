use crate::commands::{
    help,
    phrase_lookup,
    start,
    teapot,
    unknown,
    urban_lookup,
    word_lookup,
    wordle_lookup,
};
use teloxide::dispatching::{DpHandlerDescription, UpdateFilterExt};
use teloxide::dptree::{Endpoint, Handler};
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{Message, Requester, Update};
use teloxide::types::{Me, ParseMode};
use teloxide::utils::command::{BotCommands, ParseError};
use teloxide::Bot;

#[derive(Clone, BotCommands, Debug)]
#[command(rename_rule = "lowercase", description = "Here are the supported commands:")]
pub enum MessageCommands {
    #[command(hide)]
    Unknown,
    #[command(
        description = "Print this helpful message"
    )]
    Help,
    #[command(
        description = "Doesn't really do anything, is just here to greet you."
    )]
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
    #[command(
        description = "Get definition(s) of today's wordle.\n\
        Can also be sent in any chat tagging the bot and picking \"Send Today's wordle definition\""
    )]
    Wordle,
    #[command(
        description = "Get definition(s) of a word or a phrase from UrbanDictionary.\n\
        You can also look up words right in the chat by writing `@WordsLookupBot u.word`,\
        where `u.` will point to look in the UrbanDictionary"
    )]
    Urban(String),
}
fn extract_text_command(text: &str) -> MessageCommands {
    let words = text.split_whitespace()
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
    let cmd = MessageCommands::parse(text, &username)
        .unwrap_or_else(|err| match err {
            ParseError::UnknownCommand(cmd) if cmd.starts_with("/")
            => MessageCommands::Unknown,
            _ => extract_text_command(text)
        });

    log::info!("Received message: {:?}", text);
    log::info!("Processing command {:?}", cmd);
    cmd
}

pub async fn drop_empty(bot: Bot, message: Message, phrase: String) -> bool {
    if phrase.is_empty() {
        let _ = bot.send_message(
            message.chat.id,
            "You meed to specify a phrase to look up, like so: `\\phrase buckle up`",
        )
            .parse_mode(ParseMode::MarkdownV2)
            .await;
        false
    } else {
        true
    }
}

pub type CommandHandler = Endpoint<'static, anyhow::Result<()>, DpHandlerDescription>;

pub fn commands_tree() -> Handler<'static, anyhow::Result<()>, DpHandlerDescription> {
    Update::filter_message()
        .map(extract_command)
        .branch(wordle_lookup())
        .branch(word_lookup())
        .branch(phrase_lookup())
        .branch(urban_lookup())
        .branch(help())
        .branch(unknown())
        .branch(start())
        .branch(teapot())
}


pub trait BotExt {
    async fn with_err_response(
        &self,
        message: Message,
        handle: impl AsyncFnOnce(Bot, Message) -> anyhow::Result<()>,
    ) -> anyhow::Result<()>;
}
impl BotExt for Bot {
    async fn with_err_response(
        &self,
        message: Message,
        handle: impl AsyncFnOnce(Bot, Message) -> anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        let chat_id = message.chat.id;
        if let Err(err) = handle(self.clone(), message.clone()).await {
            let send_res = self.send_message(
                chat_id,
                "There was an error processing your query, try again later, sorry.",
            ).await;
            if let Err(err) = send_res {
                log::error!("Couldn't send error-response: {}", err);
            }
            Err(err)
        } else {
            Ok(())
        }
    }
}