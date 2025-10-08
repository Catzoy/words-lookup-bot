use crate::commands::CommandHandler;
use shuttle_runtime::async_trait;
use std::fmt::Debug;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{InlineQuery, Message, Requester};
use teloxide::types::{InlineQueryResult, ParseMode};
use teloxide::Bot;

#[derive(Debug, Clone)]
pub enum LookupError {
    FailedResponseBuilder,
}
#[async_trait]
pub trait BotExt {
    async fn respond_generic_err(&self, message: Message) -> anyhow::Result<()>;
}

#[async_trait]
impl BotExt for Bot {
    async fn respond_generic_err(&self, message: Message) -> anyhow::Result<()> {
        let chat_id = message.chat.id;
        let text = "There was an error processing your query, try again later, sorry.";
        if let Err(err) = self.send_message(chat_id, text).await {
            log::error!("Couldn't send error-response: {}", err);
        }
        Ok(())
    }
}

#[async_trait]
pub trait Lookup: Clone {
    type Request: Clone + Send + Sync;
    type Response: Clone + Send + Sync + Default;

    fn handler() -> CommandHandler;
}

pub trait MessageLookup {
    async fn retrieve_or_generic_err(
        bot: Bot,
        message: Message,
        response: Result<String, LookupError>,
    ) -> Option<String>;

    async fn respond_message(bot: Bot, message: Message, response: String) -> anyhow::Result<()>;
}

impl<T> MessageLookup for T
where
    T: Lookup<Request = Message, Response = String>,
{
    async fn retrieve_or_generic_err(
        bot: Bot,
        message: Message,
        response: Result<String, LookupError>,
    ) -> Option<String> {
        match response {
            Ok(value) => Some(value),
            Err(_) => {
                bot.respond_generic_err(message)
                    .await
                    .expect("Generic response OK");
                None
            }
        }
    }

    async fn respond_message(bot: Bot, message: Message, response: String) -> anyhow::Result<()> {
        bot.send_message(message.chat.id, response)
            .parse_mode(ParseMode::MarkdownV2)
            .await?;
        Ok(())
    }
}

pub trait InlineLookup {
    async fn respond_inline(
        bot: Bot,
        query: InlineQuery,
        response: Vec<InlineQueryResult>,
    ) -> anyhow::Result<()>;
}

impl<T> InlineLookup for T
where
    T: Lookup<Request = InlineQuery, Response = Vec<InlineQueryResult>>,
{
    async fn respond_inline(
        bot: Bot,
        query: InlineQuery,
        response: Vec<InlineQueryResult>,
    ) -> anyhow::Result<()> {
        bot.answer_inline_query(query.id, response).await?;
        Ok(())
    }
}
