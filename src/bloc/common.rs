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
    FailedRequest,
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
    type Entity: Clone + Send + Sync;
    type Response: Clone + Send + Sync + Default;

    fn handler() -> CommandHandler;
}

pub trait MessageLookup<Entity> {
    async fn retrieve_or_generic_err(
        bot: Bot,
        message: Message,
        response: Result<String, LookupError>,
    ) -> Option<String>;

    async fn ensure_request_success(
        bot: Bot,
        message: Message,
        response: Result<Entity, LookupError>,
    ) -> Option<Entity>;

    async fn respond_message(bot: Bot, message: Message, response: String) -> anyhow::Result<()>;
}

impl<E, T> MessageLookup<E> for T
where
    E: Clone + Send + Sync,
    T: Lookup<Request = Message, Entity = E, Response = String>,
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
    async fn ensure_request_success(
        bot: Bot,
        message: Message,
        response: Result<E, LookupError>,
    ) -> Option<E> {
        match response {
            Ok(values) => Some(values),
            Err(_) => {
                let text = "Something went wrong, please try again later";
                let resp = bot.send_message(message.chat.id, text).await;
                if let Err(e) = resp {
                    log::error!("Couldn't send error-response: {:?}", e);
                }
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

pub trait InlineLookup<Entity> {
    async fn ensure_query_success(
        bot: Bot,
        query: InlineQuery,
        result: Result<Entity, LookupError>,
    ) -> Option<Entity>;
    fn ensure_built_response(
        result: Result<Vec<InlineQueryResult>, LookupError>,
    ) -> Option<Vec<InlineQueryResult>>;

    async fn respond_inline(
        bot: Bot,
        query: InlineQuery,
        response: Vec<InlineQueryResult>,
    ) -> anyhow::Result<()>;
}

impl<Entity, T> InlineLookup<Entity> for T
where
    Entity: Clone + Send + Sync,
    T: Lookup<Request = InlineQuery, Entity = Entity, Response = Vec<InlineQueryResult>>,
{
    async fn ensure_query_success(
        bot: Bot,
        query: InlineQuery,
        result: Result<Entity, LookupError>,
    ) -> Option<Entity> {
        match result {
            Ok(values) => Some(values),
            Err(_) => {
                let result = bot.answer_inline_query(query.id, vec![]).await;
                if let Err(e) = result {
                    log::error!("Failed to send no results: {:?}", e);
                }
                None
            }
        }
    }
    fn ensure_built_response(
        result: Result<Vec<InlineQueryResult>, LookupError>,
    ) -> Option<Vec<InlineQueryResult>> {
        result.ok()
    }
    async fn respond_inline(
        bot: Bot,
        query: InlineQuery,
        response: Vec<InlineQueryResult>,
    ) -> anyhow::Result<()> {
        bot.answer_inline_query(query.id, response).await?;
        Ok(())
    }
}
