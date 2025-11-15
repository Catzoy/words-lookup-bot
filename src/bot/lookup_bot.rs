use crate::bloc::common::LookupError;
use crate::commands::FullMessageFormatter;
use crate::format::LookupFormatter;
use crate::inlines::formatting::InlineFormatter;
use shuttle_runtime::async_trait;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::Requester;
use teloxide::types::{InlineQuery, InlineQueryResult, Message, ParseMode};
use teloxide::Bot;

#[derive(Debug, Clone)]
pub struct LookupBot<T> {
    pub bot: Bot,
    pub request: T,
}

#[async_trait]
pub trait LookupBotX: Clone {
    type Request: Clone + Send + Sync;
    type Formatter: LookupFormatter + Default;
    type Response: Clone + Send + Sync;
    async fn answer_generic_err(&self) -> anyhow::Result<()>;
    async fn answer(&self, response: Self::Response) -> anyhow::Result<()>;
    async fn drop_empty(&self, phrase: String) -> bool;
    async fn ensure_request_success<Entity>(
        &self,
        response: Result<Entity, LookupError>,
    ) -> Option<Entity>
    where
        Entity: Send;
    async fn retrieve_or_generic_err(
        &self,
        response: Result<Self::Response, LookupError>,
    ) -> Option<Self::Response>;

    async fn respond(&self, response: Self::Response) -> anyhow::Result<()>;
    fn formatter(&self) -> Self::Formatter {
        Self::Formatter::default()
    }
}

#[async_trait]
impl LookupBotX for LookupBot<Message> {
    type Request = Message;
    type Formatter = FullMessageFormatter;
    type Response = String;
    async fn answer_generic_err(&self) -> anyhow::Result<()> {
        let chat_id = self.request.chat.id;
        let text = "There was an error processing your query, try again later, sorry.";
        if let Err(err) = self.bot.send_message(chat_id, text).await {
            log::error!("Couldn't send error-response: {}", err);
        }
        Ok(())
    }

    async fn answer(&self, text: String) -> anyhow::Result<()> {
        &self
            .bot
            .send_message(self.request.chat.id, text)
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

#[async_trait]
impl LookupBotX for LookupBot<InlineQuery> {
    type Request = InlineQuery;
    type Formatter = InlineFormatter;
    type Response = Vec<InlineQueryResult>;
    async fn answer_generic_err(&self) -> anyhow::Result<()> {
        let query_id = self.request.id.clone();
        if let Err(err) = self.bot.answer_inline_query(query_id, vec![]).await {
            log::error!("Couldn't send error-response: {}", err);
        }
        Ok(())
    }

    async fn answer(&self, answers: Vec<InlineQueryResult>) -> anyhow::Result<()> {
        let query_id = self.request.id.clone();
        &self.bot.answer_inline_query(query_id, answers).await?;
        Ok(())
    }

    async fn drop_empty(&self, phrase: String) -> bool {
        match phrase.as_str() {
            "" => {
                let _ = self.answer(vec![]).await;
                false
            }
            _ => true,
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
            Err(err) => {
                log::error!("Failed to get request: {:?}", err);
                let result = self.answer(vec![]).await;
                if let Err(e) = result {
                    log::error!("Failed to send no results: {:?}", e);
                    let _ = self.answer_generic_err().await;
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
            Ok(values) => Some(values),
            Err(err) => {
                log::error!("Failed to build response: {:?}", err);
                let result = self.answer(vec![]).await;
                if let Err(e) = result {
                    log::error!("Failed to respond generic err: {:?}", e);
                    let _ = self.answer_generic_err().await;
                }
                None
            }
        }
    }

    async fn respond(&self, response: Self::Response) -> anyhow::Result<()> {
        if let Err(e) = self.answer(response).await {
            log::error!("Failed to respond with query: {:?}", e);
            let _ = self.answer_generic_err().await;
        }
        Ok(())
    }
}
