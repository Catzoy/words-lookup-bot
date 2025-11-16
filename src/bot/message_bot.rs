use crate::bloc::common::LookupError;
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
    async fn answer_generic_err(&self) -> anyhow::Result<()> {
        let chat_id = self.message.chat.id;
        let text = "There was an error processing your query, try again later, sorry.";
        if let Err(err) = self.bot.send_message(chat_id, text).await {
            log::error!("Couldn't send error-response: {}", err);
        }
        Ok(())
    }

    async fn answer(&self, text: String) -> anyhow::Result<()> {
        &self
            .bot
            .send_message(self.message.chat.id, text)
            .parse_mode(ParseMode::MarkdownV2)
            .await?;
        Ok(())
    }

    async fn drop_empty(&self, phrase: String) -> bool {
        if phrase.is_empty() {
            let _ = self
                .answer(
                    "You meed to specify a phrase to look up, like so: `\\phrase buckle up`"
                        .to_string(),
                )
                .await;
            false
        } else {
            true
        }
    }

    async fn ensure_request_success<Entity>(
        &self,
        response: Result<Entity, LookupError>,
    ) -> Option<Entity>
    where
        Entity: Send,
    {
        match response {
            Ok(values) => Some(values),
            Err(_) => {
                let text = "Something went wrong, please try again later";
                let resp = &self.answer(text.to_string()).await;
                if let Err(e) = resp {
                    log::error!("Couldn't send error-response: {:?}", e);
                    let _ = &self.answer_generic_err().await;
                }
                None
            }
        }
    }

    async fn retrieve_or_generic_err(
        &self,
        response: Result<Self::Response, LookupError>,
    ) -> Option<Self::Response> {
        match response {
            Ok(value) => Some(value),
            Err(_) => {
                let _ = self.answer_generic_err().await;
                None
            }
        }
    }

    async fn respond(&self, response: Self::Response) -> anyhow::Result<()> {
        let res = self.answer(response).await;
        if let Err(e) = res {
            log::error!("Couldn't send response: {:?}", e);
            let _ = self.answer_generic_err().await;
        }
        Ok(())
    }
}
