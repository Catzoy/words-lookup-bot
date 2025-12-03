use crate::bloc::help::HelpBot;
use crate::bloc::phrase_lookup::PhraseLookupBot;
use crate::bloc::start::StartBot;
use crate::bloc::teapot::TeapotBot;
use crate::bloc::thesaurus_lookup::ThesaurusLookupBot;
use crate::bloc::unknown::UnknownBot;
use crate::bloc::urban_lookup::UrbanLookupBot;
use crate::bloc::word_lookup::WordLookupBot;
use crate::bloc::wordle::WordleBot;
use crate::bot::LookupBot;
use crate::commands::{FullMessageFormatter, MessageCommands};
use crate::format::ToEscaped;
use shuttle_runtime::async_trait;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::Requester;
use teloxide::types::{Message, ParseMode};
use teloxide::utils::command::BotCommands;
use teloxide::Bot;

#[derive(Debug, Clone)]
pub struct MessageBot {
    pub bot: Bot,
    pub message: Message,
}

#[async_trait]
impl LookupBot for MessageBot {
    type Request = Message;
    type Formatter = FullMessageFormatter;
    type Response = String;

    fn error_response() -> Self::Response {
        "There was an error processing your query, try again later, sorry.".to_string()
    }

    fn empty_response() -> Self::Response {
        "You need to specify a phrase to look up, like so: `\\phrase buckle up`".to_string()
    }

    async fn answer(&self, text: String) -> anyhow::Result<()> {
        let _ = &self
            .bot
            .send_message(self.message.chat.id, text)
            .parse_mode(ParseMode::MarkdownV2)
            .await?;
        Ok(())
    }
}

impl StartBot<String> for MessageBot {
    fn start_response(&self) -> String {
        "Hi!\n\
        I'm a bot that can look up words and phrases.\n\
        Simply send me a message and I'll search for the definition of the text."
            .to_string()
            .to_escaped()
    }
}

impl HelpBot<String> for MessageBot {
    fn help(&self) -> String {
        MessageCommands::descriptions().to_string().to_escaped()
    }
}

impl TeapotBot<String> for MessageBot {
    fn teapot(&self) -> String {
        "I'm a teapot".to_string()
    }
}

impl UnknownBot<String> for MessageBot {
    fn unknown_response(&self) -> String {
        "I don't know that command, sorry.".to_string()
    }
}

impl WordleBot<String> for MessageBot {
    fn wordle_error_response() -> String {
        "Could not get today's wordle, sorry, try again in an hour or so.".to_string()
    }
}
impl WordLookupBot for MessageBot {}
impl PhraseLookupBot for MessageBot {}
impl ThesaurusLookupBot for MessageBot {}
impl UrbanLookupBot for MessageBot {}
