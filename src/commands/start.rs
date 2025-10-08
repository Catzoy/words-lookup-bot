use crate::bloc::common::HandlerOwner;
use crate::commands::{CommandHandler, MessageCommands};
use crate::format::ToEscaped;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::Requester;
use teloxide::types::{Message, ParseMode};
use teloxide::Bot;

pub struct StartOwner;

impl StartOwner {
    async fn send_start(bot: Bot, message: Message) -> anyhow::Result<()> {
        let msg = "Hi!\n\
        I'm a bot that can look up words and phrases.\n\
        Simply send me a message and I'll search for the definition of the text.";
        bot.send_message(message.chat.id, msg.to_string().to_escaped())
            .parse_mode(ParseMode::MarkdownV2)
            .await?;
        Ok(())
    }
}
impl HandlerOwner for StartOwner {
    fn handler() -> CommandHandler {
        teloxide::dptree::case![MessageCommands::Start].endpoint(Self::send_start)
    }
}
