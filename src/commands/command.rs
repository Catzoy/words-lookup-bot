use crate::commands::{help, phrase_lookup, start, teapot, unknown, word_lookup};
use teloxide::dispatching::{DpHandlerDescription, UpdateFilterExt};
use teloxide::dptree::{Endpoint, Handler};
use teloxide::prelude::{Message, Requester, Update};
use teloxide::types::Me;
use teloxide::utils::command::{BotCommands, ParseError};
use teloxide::Bot;

#[derive(Clone, BotCommands, Debug)]
#[command(rename_rule = "lowercase", description = "Here are the supported commands:\n\n")]
pub enum MessageCommands {
    Unknown,
    #[command(description = "Print this helpful message")]
    Help,
    #[command(description = "Doesn't really do anything, is just here to greet you.")]
    Start,
    Teapot,
    #[command(
        alias = "word",
        description = "Find definition of the specified phrase.\nAny message containing at most 1 word, even with hyphens, will be looked up."
    )]
    WordLookup(String),
    #[command(
        alias = "phrase",
        description = "Find definition of the specified phrase.\nAny message with more than 1 word is considered to be a phrase"
    )]
    PhraseLookup(String),
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
            ParseError::UnknownCommand(_) => MessageCommands::Unknown,
            _ => extract_text_command(text)
        });

    log::info!("Received message: {:?}", text);
    log::info!("Processing command {:?}", cmd);
    cmd
}

pub type CommandHandler = Endpoint<'static, anyhow::Result<()>, DpHandlerDescription>;

pub fn commands_tree() -> Handler<'static, anyhow::Result<()>, DpHandlerDescription> {
    Update::filter_message()
        .map(extract_command)
        .branch(word_lookup())
        .branch(phrase_lookup())
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
        match handle(self.clone(), message.clone()).await {
            Ok(_) => {
                Ok(())
            }
            Err(err) => {
                let send_res = self.send_message(
                    chat_id,
                    "There was an error processing your query, try again later, sorry.",
                ).await;
                if let Err(err) = send_res {
                    log::error!("Couldn't send error-response: {}", err);
                }
                Err(err)
            }
        }
    }
}