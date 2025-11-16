use crate::bot::LookupBot;
use crate::commands::FullMessageFormatter;
use shuttle_runtime::async_trait;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::Requester;
use teloxide::types::{Message, ParseMode};
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
        "You meed to specify a phrase to look up, like so: `\\phrase buckle up`".to_string()
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
